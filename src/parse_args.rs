use std::env::args;

pub struct AsmArgs {
    pub debug_files: bool,
    pub filepath: String,
    pub copy_to_clipboard: bool,
    pub output_file: String,
}

pub fn parse_args() -> AsmArgs {
    let args = args();
    let mut debug_files = false;
    let mut filepath = String::new();
    let mut copy_to_clipboard = false;
    let mut args = args.skip(1);
    let mut output_file = String::new();
    while args.len() > 0 {
        let Some(arg) = args.next() else {break};

        match arg.as_str() {
            "-d" => {
                debug_files = true;
            }
            "-c" => {
                copy_to_clipboard = true;
            }
            "-h" => {
                println!("Usage: assembler [[-d] | [-h] | [-c]] <file> [-o <output_file>]
    -d: Create a debug file
    -h: Print this help message
    -c: Copy the output to the clipboard
    -o <output_file>: The resulting file. If not specified, the output will be printed to the console
    file: The file to assemble");
                std::process::exit(0);
            }
            "-o" => {
                let Some(ofile) = args.next() else {
                    println!("No output file specified");
                    std::process::exit(1);
                };
                if output_file != "" {
                    println!("Output file already specified");
                    std::process::exit(1);
                }
                output_file.push_str(&ofile);
            }
            _ => {
                if filepath == "" {
                    filepath.push_str(&arg);
                } else {
                    println!("Unknown argument: {}", arg);
                    std::process::exit(1);
                }
            }
        }
    }
    return AsmArgs {
        debug_files: debug_files,
        filepath: filepath,
        copy_to_clipboard: copy_to_clipboard,
        output_file: output_file,
    };
}
