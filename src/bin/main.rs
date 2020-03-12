use woody::compiler::token::SyntaxKind;
use woody::compiler::token::KEYWORDS;
fn main() {
    println!("{:?}", SyntaxKind::AnyKeyword);
    println!("{:?}", KEYWORDS.get("any"));
}
