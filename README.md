# dblade ⚔️ 🟦 🧠

Dblade is a command line program for linux. The default setup is for Azure OpenAI APi batch processing, however this code could be used for any JSON HTTPS/REST type API that takes POST requests with a JSON body.

The secrets for the API to send data to are in a `.env` file. This file is encrypted via `forge` utility. See https://github.com/jpegleg/dwarven-toolbox for more information.

Once the `.env` file is created and encrypted with `forge`, then a user can execute `dblade` to decrypt, load, and re-encrypt the `.env` and process files as declared in `evaluate.txt`.

Example `evaluate.txt`:

```
[process]
workspace/drafts/inputs_1.txt
workspace/drafts/inputs_2.txt
tmp/enforcement.txt
data_task_alpha.txt
more_files.md
```

Each file after the process stanza will be read. The logic on reading files for `dblade` is to read 20 lines, then keep reading after 20 until an empty line is encountered or we run out of data.
The amount read is a "chunk" which is sent as a prompt to Azure OpenAI JSON API.

Each run of `dblade` gets a transaction ID which is a UUIDv4. This transaction id is logged to STDOUT and used to name the output files from the Azure OpenAI API.

When `dblade` is run, we expect a user to supply the decryption password for `.env` interactively, then supply a password to encrypt `.env` with. This can be the same password or a new password.

Here is an example of running `dblade`:

```
$ dblade
[2023-08-11 01:09:44.590076121 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - Running dblade, reading password for .env decryption...
Password: 
[2023-08-11 01:09:46.466283283 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - Env decrypted for usage: .env
Password: 
[2023-08-11 01:09:48.355480695 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - Env loaded, reading password for re-encrypting .env...
[2023-08-11 01:09:48.355480695 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - Env encrypted for storage: .env
[2023-08-11 01:09:48.355617159 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - Opening evaluate.txt for files to process...
[2023-08-11 01:09:48.355638867 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - File to process: 1.txt
[2023-08-11 01:09:48.355645160 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - File to process: 2.txt
[2023-08-11 01:09:48.355710758 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - Calling AI API for chunk: {"prompt":"List types of squirrels in north america.","max_tokens":500} ...
[2023-08-11 01:10:02.617199647 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - Saved output from AI API to "review__1.txt_34e78380-9170-486f-a293-4833d411c0de"
[2023-08-11 01:10:02.617467124 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - Calling AI API for chunk: {"prompt":"List types of bats in north america.","max_tokens":500} ...
[2023-08-11 01:10:18.288379001 UTC INFO] 34e78380-9170-486f-a293-4833d411c0de - Saved output from AI API to "review__2.txt_34e78380-9170-486f-a293-4833d411c0de"
$
```

The value for `max_tokens` is hard-coded to 500 by default. This can be adjusted to also be read from `.env` or hard-coded to another value.

While the `dblade` program is designed to use Azure OpenAI, any API that takes a JSON POST can work, just adjust the code to match the need. 

The interactive password prompt can easily be replaced with reading the password or key from another mechanism if a version of `dblade` needs to run in a fully automated way, etc.
