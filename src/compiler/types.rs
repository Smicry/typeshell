#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SyntaxKind {
    Unknown,
    EndOfFileToken,
    // Literals
    NumericLiteral,
    StringLiteral,
    RegularExpressionLiteral,
    // Punctuation
    OpenBraceToken,
    CloseBraceToken,
    OpenParenToken,
    CloseParenToken,
    OpenBracketToken,
    CloseBracketToken,
    DotToken,
    DotDotDotToken,
    SemicolonToken,
    CommaToken,
    LessThanToken,
    GreaterThanToken,
    LessThanEqualsToken,
    GreaterThanEqualsToken,
    EqualsEqualsToken,
    ExclamationEqualsToken,
    EqualsEqualsEqualsToken,
    ExclamationEqualsEqualsToken,
    EqualsGreaterThanToken,
    PlusToken,
    MinusToken,
    AsteriskToken,
    SlashToken,
    PercentToken,
    PlusPlusToken,
    MinusMinusToken,
    LessThanLessThanToken,
    GreaterThanGreaterThanToken,
    GreaterThanGreaterThanGreaterThanToken,
    AmpersandToken,
    BarToken,
    CaretToken,
    ExclamationToken,
    TildeToken,
    AmpersandAmpersandToken,
    BarBarToken,
    QuestionToken,
    ColonToken,
    // Assignments
    EqualsToken,
    PlusEqualsToken,
    MinusEqualsToken,
    AsteriskEqualsToken,
    SlashEqualsToken,
    PercentEqualsToken,
    LessThanLessThanEqualsToken,
    GreaterThanGreaterThanEqualsToken,
    GreaterThanGreaterThanGreaterThanEqualsToken,
    AmpersandEqualsToken,
    BarEqualsToken,
    CaretEqualsToken,
    // Identifiers
    Identifier,
    // Reserved words
    BreakKeyword,
    CaseKeyword,
    CatchKeyword,
    ClassKeyword,
    ConstKeyword,
    ContinueKeyword,
    DebuggerKeyword,
    DefaultKeyword,
    DeleteKeyword,
    DoKeyword,
    ElseKeyword,
    EnumKeyword,
    ExportKeyword,
    ExtendsKeyword,
    FalseKeyword,
    FinallyKeyword,
    ForKeyword,
    FunctionKeyword,
    IfKeyword,
    ImportKeyword,
    InKeyword,
    InstanceOfKeyword,
    NewKeyword,
    NullKeyword,
    ReturnKeyword,
    SuperKeyword,
    SwitchKeyword,
    ThisKeyword,
    ThrowKeyword,
    TrueKeyword,
    TryKeyword,
    TypeOfKeyword,
    VarKeyword,
    VoidKeyword,
    WhileKeyword,
    WithKeyword,
    // Strict mode reserved words
    ImplementsKeyword,
    InterfaceKeyword,
    LetKeyword,
    PackageKeyword,
    PrivateKeyword,
    ProtectedKeyword,
    PublicKeyword,
    StaticKeyword,
    YieldKeyword,
    // TypeScript keywords
    AnyKeyword,
    BooleanKeyword,
    ConstructorKeyword,
    DeclareKeyword,
    GetKeyword,
    ModuleKeyword,
    RequireKeyword,
    NumberKeyword,
    SetKeyword,
    StringKeyword,
    // Parse tree nodes
    Missing,
    // Names
    QualifiedName,
    // Signature elements
    TypeParameter,
    Parameter,
    // TypeMember
    Property,
    Method,
    Constructor,
    GetAccessor,
    SetAccessor,
    CallSignature,
    ConstructSignature,
    IndexSignature,
    // Type
    TypeReference,
    TypeQuery,
    TypeLiteral,
    ArrayType,
    // Expression
    ArrayLiteral,
    ObjectLiteral,
    PropertyAssignment,
    PropertyAccess,
    IndexedAccess,
    CallExpression,
    NewExpression,
    TypeAssertion,
    ParenExpression,
    FunctionExpression,
    ArrowFunction,
    PrefixOperator,
    PostfixOperator,
    BinaryExpression,
    ConditionalExpression,
    OmittedExpression,
    // Element
    Block,
    VariableStatement,
    EmptyStatement,
    ExpressionStatement,
    IfStatement,
    DoStatement,
    WhileStatement,
    ForStatement,
    ForInStatement,
    ContinueStatement,
    BreakStatement,
    ReturnStatement,
    WithStatement,
    SwitchStatement,
    CaseClause,
    DefaultClause,
    LabelledStatement,
    ThrowStatement,
    TryStatement,
    TryBlock,
    CatchBlock,
    FinallyBlock,
    DebuggerStatement,
    VariableDeclaration,
    FunctionDeclaration,
    FunctionBlock,
    ClassDeclaration,
    InterfaceDeclaration,
    EnumDeclaration,
    ModuleDeclaration,
    ModuleBlock,
    ImportDeclaration,
    ExportAssignment,
    // Enum
    EnumMember,
    // Top-level nodes
    SourceFile,
    Program,
    // Synthesized list
    SyntaxList,
    // Enum value count
    Count,
}

// Markers
pub mod markers {
    use super::SyntaxKind;

    pub const FIRST_ASSIGNMENT: SyntaxKind = SyntaxKind::EqualsToken;
    pub const LAST_ASSIGNMENT: SyntaxKind = SyntaxKind::CaretEqualsToken;
    pub const FIRST_RESERVED_WORD: SyntaxKind = SyntaxKind::BreakKeyword;
    pub const LAST_RESERVED_WORD: SyntaxKind = SyntaxKind::WithKeyword;
    pub const FIRST_KEYWORD: SyntaxKind = SyntaxKind::BreakKeyword;
    pub const LAST_KEYWORD: SyntaxKind = SyntaxKind::StringKeyword;
    pub const FIRST_FUTURE_RESERVED_WORD: SyntaxKind = SyntaxKind::ImplementsKeyword;
    pub const LAST_FUTURE_RESERVED_WORD: SyntaxKind = SyntaxKind::YieldKeyword;
    pub const FIRST_TYPE_NODE: SyntaxKind = SyntaxKind::TypeReference;
    pub const LAST_TYPE_NODE: SyntaxKind = SyntaxKind::ArrayType;
    pub const FIRST_PUNCTUATION: SyntaxKind = SyntaxKind::OpenBraceToken;
    pub const LAST_PUNCTUATION: SyntaxKind = SyntaxKind::CaretEqualsToken;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CharacterCodes {
    NullCharacter = 0x00,
    MaxAsciiCharacter = 0x7F,

    LineFeed = 0x0A,       // \n
    CarriageReturn = 0x0D, // \r
    Space = 0x20,          // " "
    Underline = 0x5F,      // _
    Dollar = 0x24,         // $

    // number
    _0 = 0x30,
    _1 = 0x31,
    _2 = 0x32,
    _3 = 0x33,
    _4 = 0x34,
    _5 = 0x35,
    _6 = 0x36,
    _7 = 0x37,
    _8 = 0x38,
    _9 = 0x39,

    // lowercase
    _A = 0x61,
    _B = 0x62,
    _C = 0x63,
    _D = 0x64,
    _E = 0x65,
    _F = 0x66,
    _G = 0x67,
    _H = 0x68,
    _I = 0x69,
    _J = 0x6A,
    _K = 0x6B,
    _L = 0x6C,
    _M = 0x6D,
    _N = 0x6E,
    _O = 0x6F,
    _P = 0x70,
    _Q = 0x71,
    _R = 0x72,
    _S = 0x73,
    _T = 0x74,
    _U = 0x75,
    _V = 0x76,
    _W = 0x77,
    _X = 0x78,
    _Y = 0x79,
    _Z = 0x7A,

    // uppercase
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5a,

    Ampersand = 0x26,    // &
    Asterisk = 0x2A,     // *
    At = 0x40,           // @
    Backslash = 0x5C,    // \
    Bar = 0x7C,          // |
    Caret = 0x5E,        // ^
    CloseBrace = 0x7D,   // }
    CloseBracket = 0x5D, // ]
    CloseParen = 0x29,   // )
    Colon = 0x3A,        // :
    Comma = 0x2C,        // ,
    Dot = 0x2E,          // .
    DoubleQuote = 0x22,  // "
    Equals = 0x3D,       // =
    Exclamation = 0x21,  // !
    GreaterThan = 0x3E,  // >
    LessThan = 0x3C,     // <
    Minus = 0x2D,        // -
    OpenBrace = 0x7B,    // {
    OpenBracket = 0x5B,  // [
    OpenParen = 0x28,    // (
    Percent = 0x25,      // %
    Plus = 0x2B,         // +
    Question = 0x3F,     // ?
    Semicolon = 0x3B,    // ;
    SingleQuote = 0x27,  // '
    Slash = 0x2F,        // /
    Tilde = 0x7E,        // ~

    Backspace = 0x08,   // \b
    FormFeed = 0x0C,    // \f
    Tab = 0x09,         // \t
    VerticalTab = 0x0B, // \v
}

impl CharacterCodes {
    pub fn to_byte(&self) -> u8 {
        return *self as u8;
    }
}
