use super::types::{character_codes, SyntaxKind};
use std::collections::HashMap;

lazy_static! {
    pub static ref TEXT_TO_TOKEN: HashMap<&'static str, SyntaxKind> = {
        let mut m = HashMap::new();
        m.insert("any", SyntaxKind::AnyKeyword);
        m.insert("boolean", SyntaxKind::BooleanKeyword);
        m.insert("break", SyntaxKind::BreakKeyword);
        m.insert("case", SyntaxKind::CaseKeyword);
        m.insert("catch", SyntaxKind::CatchKeyword);
        m.insert("class", SyntaxKind::ClassKeyword);
        m.insert("continue", SyntaxKind::ContinueKeyword);
        m.insert("const", SyntaxKind::ConstKeyword);
        m.insert("constructor", SyntaxKind::ConstructorKeyword);
        m.insert("debugger", SyntaxKind::DebuggerKeyword);
        m.insert("declare", SyntaxKind::DeclareKeyword);
        m.insert("default", SyntaxKind::DefaultKeyword);
        m.insert("delete", SyntaxKind::DeleteKeyword);
        m.insert("do", SyntaxKind::DoKeyword);
        m.insert("else", SyntaxKind::ElseKeyword);
        m.insert("enum", SyntaxKind::EnumKeyword);
        m.insert("export", SyntaxKind::ExportKeyword);
        m.insert("extends", SyntaxKind::ExtendsKeyword);
        m.insert("false", SyntaxKind::FalseKeyword);
        m.insert("finally", SyntaxKind::FinallyKeyword);
        m.insert("for", SyntaxKind::ForKeyword);
        m.insert("function", SyntaxKind::FunctionKeyword);
        m.insert("get", SyntaxKind::GetKeyword);
        m.insert("if", SyntaxKind::IfKeyword);
        m.insert("implements", SyntaxKind::ImplementsKeyword);
        m.insert("import", SyntaxKind::ImportKeyword);
        m.insert("in", SyntaxKind::InKeyword);
        m.insert("instanceof", SyntaxKind::InstanceOfKeyword);
        m.insert("interface", SyntaxKind::InterfaceKeyword);
        m.insert("let", SyntaxKind::LetKeyword);
        m.insert("module", SyntaxKind::ModuleKeyword);
        m.insert("new", SyntaxKind::NewKeyword);
        m.insert("null", SyntaxKind::NullKeyword);
        m.insert("number", SyntaxKind::NumberKeyword);
        m.insert("package", SyntaxKind::PackageKeyword);
        m.insert("private", SyntaxKind::PrivateKeyword);
        m.insert("protected", SyntaxKind::ProtectedKeyword);
        m.insert("public", SyntaxKind::PublicKeyword);
        m.insert("require", SyntaxKind::RequireKeyword);
        m.insert("return", SyntaxKind::ReturnKeyword);
        m.insert("set", SyntaxKind::SetKeyword);
        m.insert("static", SyntaxKind::StaticKeyword);
        m.insert("string", SyntaxKind::StringKeyword);
        m.insert("super", SyntaxKind::SuperKeyword);
        m.insert("switch", SyntaxKind::SwitchKeyword);
        m.insert("this", SyntaxKind::ThisKeyword);
        m.insert("throw", SyntaxKind::ThrowKeyword);
        m.insert("true", SyntaxKind::TrueKeyword);
        m.insert("try", SyntaxKind::TryKeyword);
        m.insert("typeof", SyntaxKind::TypeOfKeyword);
        m.insert("var", SyntaxKind::VarKeyword);
        m.insert("void", SyntaxKind::VoidKeyword);
        m.insert("while", SyntaxKind::WhileKeyword);
        m.insert("with", SyntaxKind::WithKeyword);
        m.insert("yield", SyntaxKind::YieldKeyword);
        m
    };
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scanner<'a> {
    pub text: &'a [u8],
    pub pos: usize,       // Current position (end position of text of current token)
    pub len: usize,       // Length of text
    pub start_pos: usize, // Start position of whitespace before current token
    pub token_pos: usize, // Start position of text of current token
    pub token: SyntaxKind,
    pub token_value: &'a str,
    pub preceding_line_break: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a str) -> Self {
        return Scanner {
            text: text.as_bytes(),
            pos: 0,
            len: text.len(),
            start_pos: 0,
            token_pos: 0,
            token: SyntaxKind::Unknown,
            token_value: "",
            preceding_line_break: false,
        };
    }

    pub fn scan(&mut self) -> SyntaxKind {
        self.start_pos = self.pos;
        self.preceding_line_break = false;
        loop {
            self.token_pos = self.pos;
            match self.text.get(self.pos) {
                // None means self.pos >= self.len
                None => {
                    self.token = SyntaxKind::EndOfFileToken;
                    return self.token;
                }
                Some(&ch) => match ch {
                    character_codes::LINE_FEED => {}
                    character_codes::CARRIAGE_RETURN => {
                        self.preceding_line_break = true;
                    }
                    character_codes::TAB => {}
                    character_codes::VERTICAL_TAB => {}
                    character_codes::FORM_FEED => {}
                    character_codes::SPACE => {
                        self.pos += 1;
                        continue;
                    }
                    character_codes::EXCLAMATION => {
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            if self.compare_code(self.pos + 2, character_codes::EQUALS) {
                                self.pos += 3;
                                self.token = SyntaxKind::ExclamationEqualsEqualsToken;
                                return self.token;
                            }
                            self.pos += 2;
                            self.token = SyntaxKind::ExclamationEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::ExclamationToken;
                        return self.token;
                    }
                    character_codes::DOUBLE_QUOTE => {}
                    character_codes::SINGLE_QUOTE => {
                        self.token_value = self.scan_string();
                        self.token = SyntaxKind::StringLiteral;
                        return self.token;
                    }
                    // default x
                    x if x >= character_codes::MAX_ASCII_CHARACTER => {}
                },
            }
        }
    }

    pub fn scan_string(&mut self) -> &'a str {
        return "";
    }

    pub fn compare_code(&self, pos: usize, code: u8) -> bool {
        match self.text.get(pos) {
            Some(&ch) => ch == code,
            None => false,
        }
    }

    pub fn set_text_pos(&mut self, pos: usize) {
        self.pos = pos;
        self.start_pos = pos;
        self.token_pos = pos;
        self.token = SyntaxKind::Unknown;
        self.token_value = "";
        self.preceding_line_break = false;
    }
}
