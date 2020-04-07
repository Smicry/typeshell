use std::path::Path;
use std::{env, fs};
use woody::compiler::scanner::Scanner;
use woody::compiler::types::SyntaxKind;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("missing file");
            return;
        }
    };
    let src = match fs::read_to_string(Path::new(file)) {
        Ok(src) => src,
        Err(e) => {
            println!("unable to read file: {}", e);
            return;
        }
    };
    let mut s = Scanner::new(src.as_str());
    while s.token != SyntaxKind::EndOfFileToken {
        s.scan();
        println!("在{}-{}发现标记{:?}", s.token_pos, s.pos, s.token);
    }
}
