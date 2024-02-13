use clipboard::{ClipboardContext, ClipboardProvider};

pub mod parse_args;
pub mod parser;

fn main() {
    let args = parse_args::parse_args();
    let content = std::fs::read_to_string(&args.filepath).expect("Could not read file");
    let tokens = parser::tokenizer(&content);
    let symbol_table = parser::create_symbol_table(&tokens);
    if args.debug_files {
        parser::write_symbol_table(&symbol_table, format!("{}.sym", &args.filepath).as_str());
    }
    let output = parser::convert_tokens(&tokens, &symbol_table);
    if args.copy_to_clipboard {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(output.clone()).unwrap();
    }
    if args.output_file == "" {
        println!("{}", output);
    } else {
        std::fs::write(args.output_file, output).expect("Could not write to file");
    }

}
