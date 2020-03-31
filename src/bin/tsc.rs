use woody::compiler::scanner::Scanner;
fn main() {
    let mut s = Scanner::new("123");
    s.scan();
    println!("{:?}", s);
}
