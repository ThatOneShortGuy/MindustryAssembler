# Mindustry Assembler
The assembler doesn't do much other that make the jump commands dynamically calculated. It is meant for use with the [Mindustry Compiler](TBD).

## Installation
Make sure you have Rust installed (with Cargo). If you don't, you can install it [here](https://www.rust-lang.org/tools/install).
Run the following commands in your terminal:
```sh
git clone https://github.com/ThatOneShortGuy/MindustryAssembler.git
cd MindustryAssembler
cargo build --release
```

Or you can download the binary from the [releases](https://github.com/ThatOneShortGuy/MindustryAssembler/releases) page.

## Usage
Almost like normal Mindustry code, but with a few differences:
1. All code points must have whitespace in front of them after the newline
```mindustry
# Comments are fine
        jump 21 always x false # They can be at the end of a line
        op add x x 1
```
2. There are labels. They start with an &. They are used to calculate the location of a line. They do not need to be followed by an instruction, but they can be.
```mindustry
        set i 0
# &loop will be computed correctly even when there are comments
&loop   print "Hello, World!" # Here's a comment that doesn't matter
# Oh look, another comment
        printflush message1
        op add i i 1
        jump &loop lessThan i 10
```
`&loop` will be computed and replaced in the jump command with the line number of the `&loop` label (1).

## Running the assembler
```sh
assembler [-d] [-h] [-c] <file> [-o <output_file>]
```
- `-d`: Create a debug file (shows the line numbers of each label). The file is named `<file>.sym`
- `-h`: Print this help message
- `-c`: Copy the output to the clipboard
- `file`: The file to assemble
- `-o <output_file>`: The resulting file. If not specified, the output will be printed to the console

Order of the arguments is not important.