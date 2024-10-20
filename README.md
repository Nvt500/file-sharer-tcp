
# File Sharer Tcp

Share files with rust tcp.

Made to share files from desktop across devices.

# Usage

Place files in ```/files``` folder where you are in cmd.

```text
C:\wherever-you-want
├──file-sharer-tcp.exe
├──files
└──...
```

Then bind to whatever ip and you can download at said address.

```text
tcp-file-sharer 127.0.0.1
```

Or you can specify the path to the folder you want to share.

```text
tcp-file-sharer 127.0.0.1 C:\Users\user\Downloads
```

# Commands
```
File sharer through tcp. Connect to ip on any device and download files from /files or inputted path.

Usage: file-sharer-tcp <COMMANDS>

Commands:
    <ip>                Serves /files at <ip>.
    <ip> <path>         Serves folder at <path> at <ip>.
    help | -h | --help  Prints this message.

```