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
[2023-08-10 04:20:16.433606609 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Running dblade, reading password for .env decryption...
Password: 
[2023-08-10 04:20:18.515443781 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Env decrypted for usage: .env
Password: 
[2023-08-10 04:20:20.348641165 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Env loaded, reading password for re-encrypting .env...
[2023-08-10 04:20:20.348641165 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Env encrypted for storage: .env
[2023-08-10 04:20:20.348677820 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Opening evaluate.txt for files to process...
[2023-08-10 04:20:20.348695787 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - processing: 1.txt
[2023-08-10 04:20:20.348698131 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - processing: 2.txt
[2023-08-10 04:20:20.348699813 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - processing: tryharder2.txt
[2023-08-10 04:20:20.348764841 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - processing: mesero.txt
[2023-08-10 04:20:20.348850279 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Calling AI API for chunk: {"prompt":"List types of squirrels in north america.","max_tokens":500} ...
[2023-08-10 04:20:32.513309460 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Saved output from AI API to "review__1.txt_dc445016-0206-47db-8f37-d13ca436c230"
[2023-08-10 04:20:32.513670197 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Calling AI API for chunk: {"prompt":"List types of bats in north america.","max_tokens":500} ...
[2023-08-10 04:20:47.820522063 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Saved output from AI API to "review__2.txt_dc445016-0206-47db-8f37-d13ca436c230"
[2023-08-10 04:20:47.820981613 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Calling AI API for chunk: {"prompt":"If you had $500/month, which cloud services would you purchase to run your IT infrastructure.\nThis is for a small admin team to manage. Best guess on prices and services available.\nAnswer only with valid UTF-8. Answer in a single, concise, paragraph.","max_tokens":500} ...
[2023-08-10 04:21:01.173047699 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Saved output from AI API to "review__tryharder2.txt_dc445016-0206-47db-8f37-d13ca436c230"
[2023-08-10 04:21:01.173344142 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Calling AI API for chunk: {"prompt":"What is the most difficult math problem? Answer with only valid UTF-8.","max_tokens":500} ...
[2023-08-10 04:21:14.342560257 UTC INFO] dc445016-0206-47db-8f37-d13ca436c230 - Saved output from AI API to "review__mesero.txt_dc445016-0206-47db-8f37-d13ca436c230"
```

The value for `max_tokens` is hard-coded to 500 by default. This can be adjusted to also be read from `.env` or hard-coded to another value.

While the `dblade` program is designed to use Azure OpenAI, any API that takes a JSON POST can work, just adjust the code to match the need. 

The interactive password prompt can easily be replaced with reading the password or key from another mechanism if a version of `dblade` needs to run in a fully automated way, etc.
