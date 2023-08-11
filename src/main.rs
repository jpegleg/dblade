use zeroize::Zeroize;
use chacha20poly1305::{
    aead::{AeadInPlace, KeyInit},
    XChaCha20Poly1305,
};
use chacha20poly1305::aead::generic_array::GenericArray;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use hex;
use uuid::Uuid;

use curl::easy::{Easy, List};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

use std::env;
use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;

mod hashkey;

fn forge<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut rng = StdRng::from_entropy();
    let mut nonce = [0u8; 24];
    rng.fill(&mut nonce);
    let bnon = hex::encode(&nonce);
    let binding = "00000000".to_owned() + &bnon;
    let salt = binding.as_bytes();
    let mut strpassword = rpassword::prompt_password("Password: ")?;
    let password = strpassword.as_bytes();
    let mut hashed_key = hashkey::a2(&password, &salt);
    let aead = XChaCha20Poly1305::new(GenericArray::from_slice(&hashed_key));
    let mut ciphertext = Vec::new();
    let _ = input.read_to_end(&mut ciphertext);
    let tag = aead.encrypt_in_place_detached(&nonce.into(), &[], &mut ciphertext)
        .expect("Encryption failed.");
    output.write_all(&nonce)?;
    output.write_all(&tag)?;
    output.write_all(&ciphertext)?;
    strpassword.zeroize();
    hashed_key.zeroize();
    Ok(())
}

fn unforge<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut ciphertext = Vec::new();
    input.read_to_end(&mut ciphertext)?;
    let nonce = chacha20poly1305::XNonce::from_slice(&ciphertext[..24]);
    let tag = GenericArray::clone_from_slice(&ciphertext[24..40]);
    let mut plaintext = ciphertext[40..].to_vec();
    let bnon = hex::encode(&nonce);
    let binding = "00000000".to_owned() + &bnon;
    let salt = binding.as_bytes();
    let mut strpassword = rpassword::prompt_password("Password: ")?;
    let password = strpassword.as_bytes();
    let mut hashed_key = hashkey::a2(&password, &salt);
    let aead = XChaCha20Poly1305::new(GenericArray::from_slice(&hashed_key));
    aead.decrypt_in_place_detached(&nonce, &[], &mut plaintext, &tag)
        .expect("Error: Decryption failed.");
    output.write_all(&plaintext)?;
    strpassword.zeroize();
    hashed_key.zeroize();
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct AzureOpenAI {
    prompt: String,
    max_tokens: i32,
}


fn read_chunks<T: BufRead>(mut reader: T) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut chunk = Vec::new();
    let mut line_count = 0;
    let mut next_line = String::new();

    while let Ok(read) = reader.read_line(&mut next_line) {
        if read == 0 || line_count >= 20 {
            let line = next_line.trim().to_string();
            next_line.clear();
            if !line.is_empty() {
                chunk.push(line);
                line_count += 1;
            } else {
                chunks.push(chunk.join("\n"));
                chunk.clear();
                line_count = 0;
            }
            if read == 0 {
                break;
            }
        } else {
            let line = next_line.trim().to_string();
            next_line.clear();
            chunk.push(line);
            line_count += 1;
        }
    }
    chunks
}

fn main() -> io::Result<()> {
    let ddat1 = Utc::now();
    let txid = Uuid::new_v4();
    println!("[{} INFO] {} - Running dblade, reading password for .env decryption...", ddat1, &txid);
    let file_path = ".env";
    let mut input_file = fs::File::open(".env")?;
    let temp_file_path = format!("{}.tmp_{}", file_path, &txid);
    let mut output_file = fs::File::create(&temp_file_path)?;
    unforge(&mut input_file, &mut output_file)?;
    std::fs::rename(&temp_file_path, file_path)?;
    let ddat2 = Utc::now();
    println!("[{} INFO] {} - Env decrypted for usage: {}", ddat2, &txid, file_path);
    dotenv::dotenv().ok();
    let mut api_key = env::var("API_KEY").expect("API_KEY not set in .env");
    let mut api_url = env::var("API_URL").expect("API_URL not set in .env");
    let mut input_file = fs::File::open(".env")?;
    let temp_file_path = format!("{}.tmp_{}", file_path, &txid);
    let mut output_file = fs::File::create(&temp_file_path)?;
    forge(&mut input_file, &mut output_file)?;
    std::fs::rename(&temp_file_path, file_path)?;
    let ddat3 = Utc::now();
    println!("[{} INFO] {} - Env loaded, reading password for re-encrypting .env...", &ddat3, &txid);
    println!("[{} INFO] {} - Env encrypted for storage: {}", ddat3, &txid, file_path);
    let ddat4 = Utc::now();
    println!("[{} INFO] {} - Opening evaluate.txt for files to process...", ddat4, &txid);
    let txt_contents = fs::read_to_string("evaluate.txt")?;
    let mut processing_files = Vec::new();
    let lines = txt_contents.lines();
    let mut processing = false;
    for line in lines {

        if line.trim() == "[process]" {
            processing = true;
            continue;
        }

        if processing && !line.is_empty() {
            processing_files.push(line.trim());
            let ddat5 = Utc::now();
            println!("[{} INFO] {} - Will process: {}", ddat5, &txid, line);
        } else {
            let ddat6 = Utc::now();
            println!("[{} INFO] {} - File list loaded.", ddat6, &txid);
        }

    }
 
    for file in processing_files {
        let file_path = Path::new(file);
        let input_file = fs::File::open(file_path)?;
        let reader = io::BufReader::new(input_file);
        let chunks = read_chunks(reader);

        let mut output = String::new();
        for chunk in &chunks {
            let json_payload = AzureOpenAI {
                prompt: chunk.to_owned(),
                max_tokens: 500.to_owned(),
            };

            let j = serde_json::to_string(&json_payload)?;
            let mut handle = Easy::new();
            let mut list = List::new();
            let mut data = Vec::new();
            let ddat7 = Utc::now();
            println!("[{} INFO] {} - Calling AI API for chunk: {} ...", ddat7, &txid, &j);
            list.append(&("Content-Type:".to_owned() + "application/json")).unwrap();
            list.append(&("api-key:".to_owned() + &api_key)).unwrap();
            handle.post(true)?;
            handle.post_fields_copy(&j.into_bytes())?;
            handle.url(&api_url).unwrap();
            handle.http_headers(list).unwrap();
            let mut transfer = handle.transfer();
            {
                transfer.write_function(|new_d| {
                    data.extend_from_slice(new_d);
                    let mez: String = String::from_utf8(data.clone()).unwrap();
                    output.push_str(&mez);
                    output.push('\n');
                    Ok(new_d.len())
                }).unwrap();
       
                transfer.perform().unwrap();
            }
        }

        let ugen = &txid.to_string();
        let output_filename = format!("review__{}_{}", file_path.file_name().unwrap().to_str().unwrap(), ugen);
        let mut output_file = fs::File::create(&output_filename)?;
        let ddat8 = Utc::now();
        println!("[{} INFO] {} - Saved output from AI API to {:?}", ddat8, &txid, output_filename);
        output_file.write_all(output.as_bytes())?;
    }

    api_key.zeroize();
    api_url.zeroize();

    Ok(())
}
