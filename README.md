# minutils
`minutils` is a portable Unix-like toolset that supplies lightweight implementations of popular commands.

## Commands
### echo
Print the supplied arguments to `stdout` as a space-seperated string.

Example: `minutils-echo hello world`

### mkdir
Create a directory and all of its parents, silently continuing if any directories already exist. Functions similarly to `mkdir -p` in other implementations.

Example: `minutils-mkdir ~/abc/xyz/`

### rm
Remove all specified files, failing if any do not already exist.

Example: `minutils-rm abc xyz tuv`

### cp
> [!NOTE]
> The `minutils` implementation of cp only accepts two arguments (extras are ignored). This means that traditional shell globbing (`cp xyz/* abc/`) does not work properly.

Copy the file specified in the first argument to the filename specified in the second argument. You cannot copy to a directory without specifying the destination filename.

Example: `minutils-cp abc xyz`

### ls
List all files in the supplied directory (including hidden ones) in alphabetical order. If no arguments are supplied, list the current directory.

Example: `minutils-ls ~/`

### touch
> [!NOTE]
> The traditional `touch` command changes access timestamps on files that already exist; this behavior is currently unimplemented in `minutils`.

If the file path specified in the first argument doesn't exist, create it; otherwise exit silently.

Example: `touch abc`

### minutils
Show the current version of minutils and its included binaries.

Example: `minutils`
