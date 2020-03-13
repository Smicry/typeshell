use super::token::SyntaxKind;
pub struct Lexer<'a> {
    pub chunk: &'a str,      // source code
    pub chunk_name: &'a str, // source name
    pub line: u32,           // current line number
    pub next_token: String,
    pub next_token_kind: SyntaxKind,
    pub next_token_line: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(chunk: &'a str, chunk_name: &'a str) -> Self {
        return Lexer {
            chunk,
            chunk_name,
            line: 1,
            next_token: "".to_string(),
            next_token_kind: SyntaxKind::Unknown,
            next_token_line: 0,
        };
    }

    pub fn line(&self) -> u32 {
        return self.line;
    }

    pub fn look_ahead(&mut self) -> SyntaxKind {
        if self.next_token_line > 0 {
            return self.next_token_kind;
        }
        let current_line = self.line;
        let (line, kind, token) = self.next_token();
        self.line = current_line;
        self.next_token_line = line;
        self.next_token_kind = kind;
        self.next_token = token;
        return kind;
    }

    // todo
    // next_token -> (line,kind,token)
    pub fn next_token(&self) -> (u32, SyntaxKind, String) {
        return (0, SyntaxKind::Unknown, "".to_string());
    }
}
