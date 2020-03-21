use woody::compiler::scanner::Scanner;
fn main() {
    let s = Scanner::new("");
    println!("{:?}", s.text.get(s.pos));
}
