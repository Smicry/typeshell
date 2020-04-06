use woody::compiler::scanner::Scanner;
fn main() {
    let mut s = Scanner::new("var let");
    s.scan();
    println!("{:?}", s);
}
