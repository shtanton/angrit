# angrit
Lots of specific little commands designed around inter-process communication. Early development, will either be abandoned or will have loads of breaking changes.

They make heavy use of jsonrpc over stdio and are designed around being called from python scripts.

Currently only one is implemented

# poll
Used for
- Starting a process
- Polling the status of the process
- Displaying the logged statuses with a name for each
- Exporting these as JSON

I use this for running an audio recording process and polling and displaying the current timestamp to automatically cut out parts later on.
