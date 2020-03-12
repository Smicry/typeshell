use std::collections::HashMap;

#[derive(Debug)]
pub enum SyntaxKind {
    AnyKeyword,                                   // any
    Booleankeyword,                               // boolean
    BreakKeyword,                                 // break
    CaseKeyword,                                  // case
    CatchKeyword,                                 // catch
    ClassKeyword,                                 // class
    ContinueKeyword,                              // continue
    ConstKeyword,                                 // const
    ConstructorKeyword,                           // constructor
    DebuggerKeyword,                              // debugger
    DeclareKeyword,                               // declare
    DefaultKeyword,                               // default
    DeleteKeyword,                                // delete
    DoKeyword,                                    // do
    ElseKeyword,                                  // else
    EnumKeyword,                                  // enum
    ExportKeyword,                                // export
    ExtendsKeyword,                               // extends
    FalseKeyword,                                 // false
    FinallyKeyword,                               // finally
    ForKeyword,                                   // for
    FunctionKeyword,                              // function
    GetKeyword,                                   // get
    IfKeyword,                                    // if
    ImplementsKeyword,                            // implements
    ImportKeyword,                                // import
    InKeyword,                                    // in
    InstanceofKeyword,                            // instanceof
    InterfaceKeyword,                             // interface
    LetKeyword,                                   // let
    ModuleKeyword,                                // module
    NewKeyword,                                   // new
    NullKeyword,                                  // null
    NumberKeyword,                                // number
    PackageKeyword,                               // package
    PrivateKeyword,                               // private
    ProtectedKeyword,                             // protected
    PublicKeyword,                                // public
    RequireKeyword,                               // require
    ReturnKeyword,                                // return
    SetKeyword,                                   // set
    StaticKeyword,                                // static
    StringKeyword,                                // string
    SuperKeyword,                                 // super
    SwitchKeyword,                                // switch
    ThisKeyword,                                  // this
    ThrowKeyword,                                 // throw
    TrueKeyword,                                  // true
    TryKeyword,                                   // try
    TypeofKeyword,                                // typeof
    VarKeyword,                                   // var
    VoidKeyword,                                  // void
    WhileKeyword,                                 // while
    WithKeyword,                                  // with
    YieldKeyword,                                 // yield
    OpenbraceToken,                               // {
    ClosebraceToken,                              // }
    OpenparenToken,                               // (
    CloseparenToken,                              // )
    OpenbracketToken,                             // [
    ClosebracketToken,                            // ]
    DotToken,                                     // .
    DotdotdotToken,                               // ...
    SemicolonToken,                               // ;
    CommaToken,                                   // ,
    LessthanToken,                                // <
    GreaterthanToken,                             // >
    LessthanequalsToken,                          // <=
    GreaterthanequalsToken,                       // >=
    EqualsequalsToken,                            // ==
    ExclamationequalsToken,                       // !=
    EqualsequalsequalsToken,                      // ===
    ExclamationequalsequalsToken,                 // !==
    EqualsgreaterthanToken,                       // =>
    PlusToken,                                    // +
    MinusToken,                                   // -
    AsteriskToken,                                // *
    SlashToken,                                   // /
    PercentToken,                                 // %
    PlusplusToken,                                // ++
    MinusminusToken,                              // --
    LessthanlessthanToken,                        // <<
    GreaterthangreaterthanToken,                  // >>
    GreaterthangreaterthangreaterthanToken,       // >>>
    AmpersandToken,                               // &
    BarToken,                                     // |
    CaretToken,                                   // ^
    ExclamationToken,                             // !
    TildeToken,                                   // ~
    AmpersandampersandToken,                      // &&
    BarbarToken,                                  // ||
    QuestionToken,                                // ?
    ColonToken,                                   // :
    EqualsToken,                                  // =
    PlusequalsToken,                              // +=
    MinusequalsToken,                             // -=
    AsteriskequalsToken,                          // *=
    SlashequalsToken,                             // /=
    PercentequalsToken,                           // %=
    LessthanlessthanequalsToken,                  // <<=
    GreaterthangreaterthanequalsToken,            // >>=
    GreaterthangreaterthangreaterthanequalsToken, // >>>=
    AmpersandequalsToken,                         // &=
    BarequalsToken,                               // |=
    CaretequalsToken,                             // ^=
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, SyntaxKind> = {
        let mut m = HashMap::new();
        m.insert("any", SyntaxKind::AnyKeyword);
        m.insert("boolean", SyntaxKind::Booleankeyword);
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
        m.insert("instanceof", SyntaxKind::InstanceofKeyword);
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
        m.insert("typeof", SyntaxKind::TypeofKeyword);
        m.insert("var", SyntaxKind::VarKeyword);
        m.insert("void", SyntaxKind::VoidKeyword);
        m.insert("while", SyntaxKind::WhileKeyword);
        m.insert("with", SyntaxKind::WithKeyword);
        m.insert("yield", SyntaxKind::YieldKeyword);
        m
    };
}
