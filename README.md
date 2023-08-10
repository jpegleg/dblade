# dblade ⚔️ 🟦 🧠

Dblade is a command line program for linux. THe default setup is for Azure OpenAI, however this code could be used for any JSON HTTPS/REST type API that takes POST requests with a JSON body.

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
