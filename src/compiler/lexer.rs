pub struct Lexer {
    pub chunk: String,      // source code
    pub chunk_name: String, // source name
    pub line: u32,          // current line number
    pub next_token: String,
    pub next_token_kind: u32,
    pub next_token_line: u32,
}
