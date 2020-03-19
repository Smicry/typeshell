use woody::compiler::scanner::Scanner;
use woody::compiler::types::CharacterCodes;
fn main() {
    let s = Scanner::new("/r");
    println!("{:?}", s);
    println!("{:?}", CharacterCodes::_0.to_byte());
}
