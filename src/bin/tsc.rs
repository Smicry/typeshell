use woody::compiler::scanner::Scanner;
fn main() {
    let mut s = Scanner::new("\"a\"");
    s.scan();
    println!("{:?}", s);
}
