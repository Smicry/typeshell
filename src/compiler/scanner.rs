use super::types::{character_codes, syntax_kind, SyntaxKind};
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
#[derive(Debug, Clone, PartialEq)]
pub struct Scanner<'a> {
    text: &'a [u8],
    pos: usize,       // Current position (end position of text of current token)
    len: usize,       // Length of text
    start_pos: usize, // Start position of whitespace before current token
    token_pos: usize, // Start position of text of current token
    token: SyntaxKind,
    token_value: String,
    preceding_line_break: bool,
}

impl<'a> Scanner<'a> {
    pub fn create_scanner(text: &'a str) -> Self {
        return Scanner {
            text: text.as_bytes(),
            pos: 0,
            len: text.len(),
            start_pos: 0,
            token_pos: 0,
            token: SyntaxKind::Unknown,
            token_value: "".to_string(),
            preceding_line_break: false,
        };
    }

    pub fn get_start_pos(&self) -> usize {
        return self.start_pos;
    }

    pub fn get_text_pos(&self) -> usize {
        return self.pos;
    }

    pub fn get_token(&self) -> SyntaxKind {
        return self.token;
    }

    pub fn get_token_pos(&self) -> usize {
        return self.token_pos;
    }

    pub fn get_token_text(&self) -> String {
        return self.sub_str(self.token_pos, self.pos);
    }

    pub fn get_token_value(&self) -> &str {
        return self.token_value.as_str();
    }

    pub fn has_preceding_line_break(&self) -> bool {
        return self.preceding_line_break;
    }

    pub fn is_identifier(&self) -> bool {
        if self.token == SyntaxKind::Identifier || self.token as usize > syntax_kind::LAST_RESERVED_WORD as usize {
            return true;
        }
        return false;
    }

    pub fn is_reserved_word(&self) -> bool {
        if self.token as usize >= syntax_kind::FIRST_RESERVED_WORD as usize
            && self.token as usize <= syntax_kind::LAST_RESERVED_WORD as usize
        {
            return true;
        }
        return false;
    }

    pub fn rescan_greater_token(&mut self) -> SyntaxKind {
        if self.token == SyntaxKind::GreaterThanToken {
            if self.compare_code(self.pos, character_codes::GREATER_THAN) {
                if self.compare_code(self.pos + 1, character_codes::GREATER_THAN) {
                    if self.compare_code(self.pos + 2, character_codes::EQUALS) {
                        self.pos += 3;
                        self.token = SyntaxKind::GreaterThanGreaterThanGreaterThanEqualsToken;
                        return self.token;
                    }
                    self.pos += 2;
                    self.token = SyntaxKind::GreaterThanGreaterThanGreaterThanToken;
                    return self.token;
                }
                if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                    self.pos += 2;
                    self.token = SyntaxKind::GreaterThanGreaterThanEqualsToken;
                    return self.token;
                }
                self.pos += 1;
                self.token = SyntaxKind::GreaterThanGreaterThanToken;
                return self.token;
            }
            if self.compare_code(self.pos, character_codes::EQUALS) {
                self.pos += 1;
                self.token = SyntaxKind::GreaterThanEqualsToken;
                return self.token;
            }
        }
        return self.token;
    }

    pub fn rescan_slash_token(&mut self) -> SyntaxKind {
        if self.token == SyntaxKind::SlashToken || self.token == SyntaxKind::SlashEqualsToken {
            let mut p = self.token_pos + 1;
            let mut in_escape = false;
            let mut in_character_class = false;

            while let Some(&ch) = self.text.get(p) {
                // Line breaks are not permissible in the middle of a RegExp.
                if Scanner::is_line_break(ch) {
                    return self.token;
                }
                if in_escape {
                    // Parsing an escape character;
                    // reset the flag and just advance to the next char.
                    in_escape = false;
                } else if ch == character_codes::SLASH && !in_character_class {
                    // A slash within a character class is permissible,
                    // but in general it signals the end of the regexp literal.
                    break;
                } else if ch == character_codes::OPEN_BRACKET {
                    in_character_class = true;
                } else if ch == character_codes::BACKSLASH {
                    in_escape = true;
                } else if ch == character_codes::CLOSE_BRACKET {
                    in_character_class = false;
                }
                p += 1;
            }
            p += 1;
            while let Some(&current) = self.text.get(p) {
                if !Scanner::is_identifier_part(current) {
                    break;
                }
                p += 1;
            }
            self.pos = p;
            self.token_value = self.sub_str(self.token_pos, self.pos);
            self.token = SyntaxKind::RegularExpressionLiteral;
        }
        return self.token;
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
                    character_codes::LINE_FEED | character_codes::CARRIAGE_RETURN => {
                        self.preceding_line_break = true;
                        self.pos += 1;
                    }
                    character_codes::TAB
                    | character_codes::VERTICAL_TAB
                    | character_codes::FORM_FEED
                    | character_codes::SPACE => {
                        self.pos += 1;
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
                    character_codes::DOUBLE_QUOTE | character_codes::SINGLE_QUOTE => {
                        self.token_value = self.scan_string(ch);
                        self.token = SyntaxKind::StringLiteral;
                        return self.token;
                    }
                    character_codes::PERCENT => {
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            self.pos += 2;
                            self.token = SyntaxKind::PercentEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::PercentToken;
                        return self.token;
                    }
                    character_codes::AMPERSAND => {
                        if self.compare_code(self.pos + 1, character_codes::AMPERSAND) {
                            self.pos += 2;
                            self.token = SyntaxKind::AmpersandAmpersandToken;
                            return self.token;
                        }
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            self.pos += 2;
                            self.token = SyntaxKind::AmpersandEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::AmpersandToken;
                        return self.token;
                    }
                    character_codes::OPEN_PAREN => {
                        self.pos += 1;
                        self.token = SyntaxKind::OpenParenToken;
                        return self.token;
                    }
                    character_codes::CLOSE_PAREN => {
                        self.pos += 1;
                        self.token = SyntaxKind::CloseParenToken;
                        return self.token;
                    }
                    character_codes::ASTERISK => {
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            self.pos += 2;
                            self.token = SyntaxKind::AsteriskEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::AsteriskToken;
                        return self.token;
                    }
                    character_codes::PLUS => {
                        if self.compare_code(self.pos + 1, character_codes::PLUS) {
                            self.pos += 2;
                            self.token = SyntaxKind::PlusPlusToken;
                            return self.token;
                        }
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            self.pos += 2;
                            self.token = SyntaxKind::PlusEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::PlusToken;
                        return self.token;
                    }
                    character_codes::COMMA => {
                        self.pos += 1;
                        self.token = SyntaxKind::CommaToken;
                        return self.token;
                    }
                    character_codes::MINUS => {
                        if self.compare_code(self.pos + 1, character_codes::MINUS) {
                            self.pos += 2;
                            self.token = SyntaxKind::MinusMinusToken;
                            return self.token;
                        }
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            self.pos += 2;
                            self.token = SyntaxKind::MinusEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::MinusToken;
                        return self.token;
                    }
                    character_codes::DOT => {
                        if self.is_digit(self.pos + 1) {
                            self.token_value = self.scan_number();
                            self.token = SyntaxKind::NumericLiteral;
                            return self.token;
                        }
                        if self.compare_code(self.pos + 1, character_codes::DOT)
                            && self.compare_code(self.pos + 2, character_codes::DOT)
                        {
                            self.pos += 3;
                            self.token = SyntaxKind::DotDotDotToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::DotToken;
                        return self.token;
                    }
                    character_codes::SLASH => {
                        // Single-line comment
                        if self.compare_code(self.pos + 1, character_codes::SLASH) {
                            self.pos += 2;
                            while let Some(&current) = self.text.get(self.pos) {
                                if Scanner::is_line_break(current) {
                                    break;
                                }
                                self.pos += 1;
                            }
                            continue;
                        }
                        // Multi-line comment
                        if self.compare_code(self.pos + 1, character_codes::ASTERISK) {
                            self.pos += 2;
                            let mut comment_closed = false;
                            while let Some(&current) = self.text.get(self.pos) {
                                if current == character_codes::ASTERISK
                                    && self.compare_code(self.pos + 1, character_codes::SLASH)
                                {
                                    self.pos += 2;
                                    comment_closed = true;
                                    break;
                                }
                                if Scanner::is_line_break(current) {
                                    self.preceding_line_break = true;
                                }
                                self.pos += 1;
                            }
                            if !comment_closed {
                                println!("'*/' expected.")
                            }
                            continue;
                        }
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            self.pos += 2;
                            self.token = SyntaxKind::SlashEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::SlashToken;
                        return self.token;
                    }
                    // TODO:deal Hex and Octal
                    character_codes::_0
                    | character_codes::_1
                    | character_codes::_2
                    | character_codes::_3
                    | character_codes::_4
                    | character_codes::_5
                    | character_codes::_6
                    | character_codes::_7
                    | character_codes::_8
                    | character_codes::_9 => {
                        self.token_value = self.scan_number();
                        self.token = SyntaxKind::NumericLiteral;
                        return self.token;
                    }
                    character_codes::COLON => {
                        self.pos += 1;
                        self.token = SyntaxKind::ColonToken;
                        return self.token;
                    }
                    character_codes::SEMICOLON => {
                        self.pos += 1;
                        self.token = SyntaxKind::SemicolonToken;
                        return self.token;
                    }
                    character_codes::LESS_THAN => {
                        if self.compare_code(self.pos + 1, character_codes::LESS_THAN) {
                            if self.compare_code(self.pos + 2, character_codes::EQUALS) {
                                self.pos += 3;
                                self.token = SyntaxKind::LessThanLessThanEqualsToken;
                                return self.token;
                            }
                            self.pos += 2;
                            self.token = SyntaxKind::LessThanLessThanToken;
                            return self.token;
                        }
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            self.pos += 2;
                            self.token = SyntaxKind::LessThanEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::LessThanToken;
                        return self.token;
                    }
                    character_codes::EQUALS => {
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            if self.compare_code(self.pos + 2, character_codes::EQUALS) {
                                self.pos += 3;
                                self.token = SyntaxKind::EqualsEqualsEqualsToken;
                                return self.token;
                            }
                            self.pos += 2;
                            self.token = SyntaxKind::EqualsEqualsToken;
                            return self.token;
                        }
                        if self.compare_code(self.pos + 1, character_codes::GREATER_THAN) {
                            self.pos += 2;
                            self.token = SyntaxKind::EqualsGreaterThanToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::EqualsToken;
                        return self.token;
                    }
                    character_codes::GREATER_THAN => {
                        self.pos += 1;
                        self.token = SyntaxKind::GreaterThanToken;
                        return self.token;
                    }
                    character_codes::QUESTION => {
                        self.pos += 1;
                        self.token = SyntaxKind::QuestionToken;
                        return self.token;
                    }
                    character_codes::OPEN_BRACKET => {
                        self.pos += 1;
                        self.token = SyntaxKind::OpenBracketToken;
                        return self.token;
                    }
                    character_codes::CLOSE_BRACKET => {
                        self.pos += 1;
                        self.token = SyntaxKind::CloseBracketToken;
                        return self.token;
                    }
                    character_codes::CARET => {
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            self.pos += 2;
                            self.token = SyntaxKind::CaretEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::CaretToken;
                        return self.token;
                    }
                    character_codes::OPEN_BRACE => {
                        self.pos += 1;
                        self.token = SyntaxKind::OpenBraceToken;
                        return self.token;
                    }
                    character_codes::BAR => {
                        if self.compare_code(self.pos + 1, character_codes::BAR) {
                            self.pos += 2;
                            self.token = SyntaxKind::BarBarToken;
                            return self.token;
                        }
                        if self.compare_code(self.pos + 1, character_codes::EQUALS) {
                            self.pos += 2;
                            self.token = SyntaxKind::BarEqualsToken;
                            return self.token;
                        }
                        self.pos += 1;
                        self.token = SyntaxKind::BarToken;
                        return self.token;
                    }
                    character_codes::CLOSE_BRACE => {
                        self.pos += 1;
                        self.token = SyntaxKind::CloseBraceToken;
                        return self.token;
                    }
                    character_codes::TILDE => {
                        self.pos += 1;
                        self.token = SyntaxKind::TildeToken;
                        return self.token;
                    }
                    character_codes::BACKSLASH => {
                        // TODO:deal Unicode
                        println!("Invalid character.");
                        self.pos += 1;
                        self.token = SyntaxKind::Unknown;
                        return self.token;
                    }
                    default => {
                        if Scanner::is_identifier_start(default) {
                            self.pos += 1;
                            while let Some(&current) = self.text.get(self.pos) {
                                if !Scanner::is_identifier_part(current) {
                                    break;
                                }
                                self.pos += 1;
                            }
                            self.token_value = self.sub_str(self.token_pos, self.pos);

                            if default == character_codes::SLASH {
                                // TODO:tokenValue += scanIdentifierParts();
                            }
                            return self.get_identifier_token();
                        } else if Scanner::is_white_space(default) {
                            self.pos += 1;
                            continue;
                        } else if Scanner::is_line_break(default) {
                            self.preceding_line_break = true;
                            self.pos += 1;
                            continue;
                        }
                        println!("Invalid character.");
                        self.pos += 1;
                        self.token = SyntaxKind::Unknown;
                        return self.token;
                    }
                },
            }
        }
    }

    pub fn set_text(&mut self, text: &'a str) {
        self.text = text.as_bytes();
        self.set_text_pos(0)
    }

    pub fn set_text_pos(&mut self, pos: usize) {
        self.pos = pos;
        self.start_pos = pos;
        self.token_pos = pos;
        self.token = SyntaxKind::Unknown;
        self.token_value = String::new();
        self.preceding_line_break = false;
    }

    pub fn get_identifier_token(&mut self) -> SyntaxKind {
        let temp_len = self.token_value.len();
        if temp_len >= 2 && temp_len <= 11 {
            if let Some(ch) = self.token_value.bytes().nth(0) {
                if ch >= character_codes::_A && ch <= character_codes::_Z {
                    if let Some(&token) = TEXT_TO_TOKEN.get(self.token_value.as_str()) {
                        self.token = token;
                        return self.token;
                    }
                }
            }
        }
        self.token = SyntaxKind::Identifier;
        return self.token;
    }

    fn scan_number(&mut self) -> String {
        let start = self.pos;
        while self.is_digit(self.pos) {
            self.pos += 1;
        }
        if self.compare_code(self.pos, character_codes::DOT) {
            self.pos += 1;
            while self.is_digit(self.pos) {
                self.pos += 1;
            }
        }
        let end = self.pos;
        // TODO:deal scientific notation
        if self.compare_code(self.pos, character_codes::E) || self.compare_code(self.pos, character_codes::_E) {}
        return self.sub_str(start, end);
    }

    fn scan_string(&mut self, quote: u8) -> String {
        self.pos += 1;
        let result: String;
        let start = self.pos;
        loop {
            match self.text.get(self.pos) {
                None => {
                    result = self.sub_str(start, self.pos);
                    println!("Unexpected end of text.");
                    break;
                }
                Some(&current) => {
                    if current == quote {
                        result = self.sub_str(start, self.pos);
                        self.pos += 1;
                        break;
                    }
                    // TODO:deal backslash
                    if current == character_codes::BACKSLASH {}
                    if Scanner::is_line_break(current) {
                        result = self.sub_str(start, self.pos);
                        println!("Unterminated string literal.");
                        break;
                    }
                    self.pos += 1;
                }
            }
        }
        return result;
    }

    fn is_identifier_start(ch: u8) -> bool {
        // TODO: || ch > character_codes::MAX_ASCII_CHARACTER && isUnicodeIdentifierStart()
        return ch >= character_codes::A && ch <= character_codes::Z
            || ch >= character_codes::_A && ch <= character_codes::_Z
            || ch == character_codes::DOLLAR
            || ch == character_codes::UNDERLINE;
    }

    fn is_identifier_part(ch: u8) -> bool {
        // TODO: || ch > character_codes::MAX_ASCII_CHARACTER && isUnicodeIdentifierPart()
        return ch >= character_codes::A && ch <= character_codes::Z
            || ch >= character_codes::_A && ch <= character_codes::_Z
            || ch >= character_codes::_0 && ch <= character_codes::_9
            || ch == character_codes::DOLLAR
            || ch == character_codes::UNDERLINE;
    }

    fn is_digit(&self, pos: usize) -> bool {
        match self.text.get(pos) {
            Some(&next) => next >= character_codes::_0 && next <= character_codes::_9,
            None => false,
        }
    }

    #[allow(dead_code)]
    fn is_octal_digit(&self, pos: usize) -> bool {
        match self.text.get(pos) {
            Some(&next) => next >= character_codes::_0 && next <= character_codes::_7,
            None => false,
        }
    }

    fn is_white_space(ch: u8) -> bool {
        return ch == character_codes::SPACE
            || ch == character_codes::TAB
            || ch == character_codes::VERTICAL_TAB
            || ch == character_codes::FORM_FEED
            || ch == character_codes::NON_BREAKING_SPACE;
    }

    fn is_line_break(ch: u8) -> bool {
        match ch {
            character_codes::LINE_FEED | character_codes::CARRIAGE_RETURN | character_codes::NEXT_LINE => true,
            _ => false,
        }
    }

    fn sub_str(&self, start_pos: usize, end_pos: usize) -> String {
        if start_pos >= end_pos {
            return String::new();
        }
        let mut result = String::new();
        let mut pos = start_pos;
        while let Some(&current) = self.text.get(pos) {
            if pos >= end_pos {
                break;
            }
            result.push(current as char);
            pos += 1;
        }
        return result;
    }

    fn compare_code(&self, pos: usize, code: u8) -> bool {
        match self.text.get(pos) {
            Some(&ch) => ch == code,
            None => false,
        }
    }
}
