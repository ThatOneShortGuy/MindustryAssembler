# Mindustry Assembler
The assembler doesn't do much other that make the jump commands dynamically calculated. It is meant for use with the [Mindustry Compiler]().

## Usage
```sh
assembler [-d] [-h] [-c] <file> [-o <output_file>]
```
- `-d`: Create a debug file
- `-h`: Print this help message
- `-c`: Copy the output to the clipboard
- `file`: The file to assemble
- `-o <output_file>`: The resulting file. If not specified, the output will be printed to the console

Order of the arguments is not important.