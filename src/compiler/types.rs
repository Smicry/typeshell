pub struct TextRange {
    pub pos: usize,
    pub end: usize,
}

// token > SyntaxKind.Identifer => token is a keyword
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
pub mod syntax_kind {
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

pub enum NodeFlags {
    Export = 0x00000001,          // Declarations
    Ambient = 0x00000002,         // Declarations
    QuestionMark = 0x00000004,    // Parameter/Property/Method
    Rest = 0x00000008,            // Parameter
    Public = 0x00000010,          // Property/Method
    Private = 0x00000020,         // Property/Method
    Static = 0x00000040,          // Property/Method
    MultiLine = 0x00000080,       // Multi-line array or object literal
    Synthetic = 0x00000100,       // Synthetic node (for full fidelity)
    DeclarationFile = 0x00000200, // Node is a .d.ts file
}

pub mod node_flags {
    use super::NodeFlags;
    pub const MODIFIER: usize = NodeFlags::Export as usize
        | NodeFlags::Ambient as usize
        | NodeFlags::Public as usize
        | NodeFlags::Private as usize
        | NodeFlags::Static as usize;
}

pub struct Node {
    pub text_range: TextRange,
    pub kind: SyntaxKind,
    pub id: usize,           // Unique id (used to look up NodeLinks)
    //pub parent: Node,        // Parent node (initialized by binding)
    //pub symbol: Symbol,      // Symbol declared by node (initialized by binding)
    //pub locals: SymbolTable, // Locals associated with node (initialized by binding)
    //pub nextContainer: Node, // Next container in declaration order (initialized by binding)
    //pub localSymbol: Symbol, // Local symbol declared by node (initialized by binding only for exported nodes)
}

pub mod character_codes {
    pub const NULL_CHARACTER: u8 = 0x00;
    pub const MAX_ASCII_CHARACTER: u8 = 0x7F;

    pub const LINE_FEED: u8 = 0x0A; // \n
    pub const CARRIAGE_RETURN: u8 = 0x0D; // \r
    pub const NEXT_LINE: u8 = 0x0085;

    pub const SPACE: u8 = 0x0020; // " "
    pub const NON_BREAKING_SPACE: u8 = 0x00A0; //

    pub const UNDERLINE: u8 = 0x5F; // _
    pub const DOLLAR: u8 = 0x24; // $

    // number
    pub const _0: u8 = 0x30;
    pub const _1: u8 = 0x31;
    pub const _2: u8 = 0x32;
    pub const _3: u8 = 0x33;
    pub const _4: u8 = 0x34;
    pub const _5: u8 = 0x35;
    pub const _6: u8 = 0x36;
    pub const _7: u8 = 0x37;
    pub const _8: u8 = 0x38;
    pub const _9: u8 = 0x39;

    // lowercase
    pub const _A: u8 = 0x61;
    pub const _B: u8 = 0x62;
    pub const _C: u8 = 0x63;
    pub const _D: u8 = 0x64;
    pub const _E: u8 = 0x65;
    pub const _F: u8 = 0x66;
    pub const _G: u8 = 0x67;
    pub const _H: u8 = 0x68;
    pub const _I: u8 = 0x69;
    pub const _J: u8 = 0x6A;
    pub const _K: u8 = 0x6B;
    pub const _L: u8 = 0x6C;
    pub const _M: u8 = 0x6D;
    pub const _N: u8 = 0x6E;
    pub const _O: u8 = 0x6F;
    pub const _P: u8 = 0x70;
    pub const _Q: u8 = 0x71;
    pub const _R: u8 = 0x72;
    pub const _S: u8 = 0x73;
    pub const _T: u8 = 0x74;
    pub const _U: u8 = 0x75;
    pub const _V: u8 = 0x76;
    pub const _W: u8 = 0x77;
    pub const _X: u8 = 0x78;
    pub const _Y: u8 = 0x79;
    pub const _Z: u8 = 0x7A;

    // uppercase
    pub const A: u8 = 0x41;
    pub const B: u8 = 0x42;
    pub const C: u8 = 0x43;
    pub const D: u8 = 0x44;
    pub const E: u8 = 0x45;
    pub const F: u8 = 0x46;
    pub const G: u8 = 0x47;
    pub const H: u8 = 0x48;
    pub const I: u8 = 0x49;
    pub const J: u8 = 0x4A;
    pub const K: u8 = 0x4B;
    pub const L: u8 = 0x4C;
    pub const M: u8 = 0x4D;
    pub const N: u8 = 0x4E;
    pub const O: u8 = 0x4F;
    pub const P: u8 = 0x50;
    pub const Q: u8 = 0x51;
    pub const R: u8 = 0x52;
    pub const S: u8 = 0x53;
    pub const T: u8 = 0x54;
    pub const U: u8 = 0x55;
    pub const V: u8 = 0x56;
    pub const W: u8 = 0x57;
    pub const X: u8 = 0x58;
    pub const Y: u8 = 0x59;
    pub const Z: u8 = 0x5a;

    pub const AMPERSAND: u8 = 0x26; // &
    pub const ASTERISK: u8 = 0x2A; // *
    pub const AT: u8 = 0x40; // @
    pub const BACKSLASH: u8 = 0x5C; // \
    pub const BAR: u8 = 0x7C; // |
    pub const CARET: u8 = 0x5E; // ^
    pub const CLOSE_BRACE: u8 = 0x7D; // }
    pub const CLOSE_BRACKET: u8 = 0x5D; // ]
    pub const CLOSE_PAREN: u8 = 0x29; // )
    pub const COLON: u8 = 0x3A; // :
    pub const COMMA: u8 = 0x2C; // ,
    pub const DOT: u8 = 0x2E; // .
    pub const DOUBLE_QUOTE: u8 = 0x22; // "
    pub const EQUALS: u8 = 0x3D; // =
    pub const EXCLAMATION: u8 = 0x21; // !
    pub const GREATER_THAN: u8 = 0x3E; // >
    pub const LESS_THAN: u8 = 0x3C; // <
    pub const MINUS: u8 = 0x2D; // -
    pub const OPEN_BRACE: u8 = 0x7B; // {
    pub const OPEN_BRACKET: u8 = 0x5B; // [
    pub const OPEN_PAREN: u8 = 0x28; // (
    pub const PERCENT: u8 = 0x25; // %
    pub const PLUS: u8 = 0x2B; // +
    pub const QUESTION: u8 = 0x3F; // ?
    pub const SEMICOLON: u8 = 0x3B; // ;
    pub const SINGLE_QUOTE: u8 = 0x27; // '
    pub const SLASH: u8 = 0x2F; // /
    pub const TILDE: u8 = 0x7E; // ~

    pub const BACKSPACE: u8 = 0x08; // \b
    pub const FORM_FEED: u8 = 0x0C; // \f
    pub const TAB: u8 = 0x09; // \t
    pub const VERTICAL_TAB: u8 = 0x0B; // \v
}
