use woody::compiler::scanner::Scanner;
fn main() {
    let mut s = Scanner::new("你好!");
    s.scan();
    println!("{:?}", s);
}
