use std::collections::HashMap;

pub mod syntax_kind {
    pub static ANY_KEYWORD: u32 = 0; // any
    pub static BOOL_EANKEYWORD: u32 = 1; // boolean
    pub static BREAK_KEYWORD: u32 = 2; // break
    pub static CASE_KEYWORD: u32 = 3; // case
    pub static CATCH_KEYWORD: u32 = 4; // catch
    pub static CLASS_KEYWORD: u32 = 5; // class
    pub static CONTINUE_KEYWORD: u32 = 6; // continue
    pub static CONST_KEYWORD: u32 = 7; // const
    pub static CONSTRUCTOR_KEYWORD: u32 = 8; // constructor
    pub static DEBUGGER_KEYWORD: u32 = 9; // debugger
    pub static DECLARE_KEYWORD: u32 = 10; // declare
    pub static DEFAULT_KEYWORD: u32 = 11; // default
    pub static DELETE_KEYWORD: u32 = 12; // delete
    pub static DO_KEYWORD: u32 = 13; // do
    pub static ELSE_KEYWORD: u32 = 14; // else
    pub static ENUM_KEYWORD: u32 = 15; // enum
    pub static EXPORT_KEYWORD: u32 = 16; // export
    pub static EXTENDS_KEYWORD: u32 = 17; // extends
    pub static FALSE_KEYWORD: u32 = 18; // false
    pub static FINALLY_KEYWORD: u32 = 19; // finally
    pub static FOR_KEYWORD: u32 = 20; // for
    pub static FUNCTION_KEYWORD: u32 = 21; // function
    pub static GET_KEYWORD: u32 = 22; // get
    pub static IF_KEYWORD: u32 = 23; // if
    pub static IMPLEMENTS_KEYWORD: u32 = 24; // implements
    pub static IMPORT_KEYWORD: u32 = 25; // import
    pub static IN_KEYWORD: u32 = 26; // in
    pub static INSTANCEOF_KEYWORD: u32 = 27; // instanceof
    pub static INTERFACE_KEYWORD: u32 = 28; // interface
    pub static LET_KEYWORD: u32 = 29; // let
    pub static MODULE_KEYWORD: u32 = 30; // module
    pub static NEW_KEYWORD: u32 = 31; // new
    pub static NULL_KEYWORD: u32 = 32; // null
    pub static NUMBER_KEYWORD: u32 = 33; // number
    pub static PACKAGE_KEYWORD: u32 = 34; // package
    pub static PRIVATE_KEYWORD: u32 = 35; // private
    pub static PROTECTED_KEYWORD: u32 = 36; // protected
    pub static PUBLIC_KEYWORD: u32 = 37; // public
    pub static REQUIRE_KEYWORD: u32 = 38; // require
    pub static RETURN_KEYWORD: u32 = 39; // return
    pub static SET_KEYWORD: u32 = 40; // set
    pub static STATIC_KEYWORD: u32 = 41; // static
    pub static STRING_KEYWORD: u32 = 42; // string
    pub static SUPER_KEYWORD: u32 = 43; // super
    pub static SWITCH_KEYWORD: u32 = 44; // switch
    pub static THIS_KEYWORD: u32 = 45; // this
    pub static THROW_KEYWORD: u32 = 46; // throw
    pub static TRUE_KEYWORD: u32 = 47; // true
    pub static TRY_KEYWORD: u32 = 48; // try
    pub static TYPEOF_KEYWORD: u32 = 49; // typeof
    pub static VAR_KEYWORD: u32 = 50; // var
    pub static VOID_KEYWORD: u32 = 51; // void
    pub static WHILE_KEYWORD: u32 = 52; // while
    pub static WITH_KEYWORD: u32 = 53; // with
    pub static YIELD_KEYWORD: u32 = 54; // yield
    pub static OPENBRACE_TOKEN: u32 = 55; // {
    pub static CLOSEBRACE_TOKEN: u32 = 56; // }
    pub static OPENPAREN_TOKEN: u32 = 57; // (
    pub static CLOSEPAREN_TOKEN: u32 = 58; // )
    pub static OPENBRACKET_TOKEN: u32 = 59; // [
    pub static CLOSEBRACKET_TOKEN: u32 = 60; // ]
    pub static DOT_TOKEN: u32 = 61; // .
    pub static DOTDOTDOT_TOKEN: u32 = 62; // ...
    pub static SEMICOLON_TOKEN: u32 = 63; // ;
    pub static COMMA_TOKEN: u32 = 64; // ,
    pub static LESSTHAN_TOKEN: u32 = 65; // <
    pub static GREATERTHAN_TOKEN: u32 = 66; // >
    pub static LESSTHANEQUALS_TOKEN: u32 = 67; // <=
    pub static GREATERTHANEQUALS_TOKEN: u32 = 68; // >=
    pub static EQUALSEQUALS_TOKEN: u32 = 69; // ==
    pub static EXCLAMATIONEQUALS_TOKEN: u32 = 70; // !=
    pub static EQUALSEQUALSEQUALS_TOKEN: u32 = 71; // ===
    pub static EXCLAMATIONEQUALSEQUALS_TOKEN: u32 = 72; // !==
    pub static EQUALSGREATERTHAN_TOKEN: u32 = 73; // =>
    pub static PLUS_TOKEN: u32 = 74; // +
    pub static MINUS_TOKEN: u32 = 75; // -
    pub static ASTERISK_TOKEN: u32 = 76; // *
    pub static SLASH_TOKEN: u32 = 77; // /
    pub static PERCENT_TOKEN: u32 = 78; // %
    pub static PLUSPLUS_TOKEN: u32 = 79; // ++
    pub static MINUSMINUS_TOKEN: u32 = 80; // --
    pub static LESSTHANLESSTHAN_TOKEN: u32 = 81; // <<
    pub static GREATERTHANGREATERTHAN_TOKEN: u32 = 82; // >>
    pub static GREATERTHANGREATERTHANGREATERTHAN_TOKEN: u32 = 83; // >>>
    pub static AMPERSAND_TOKEN: u32 = 84; // &
    pub static BAR_TOKEN: u32 = 85; // |
    pub static CARET_TOKEN: u32 = 86; // ^
    pub static EXCLAMATION_TOKEN: u32 = 87; // !
    pub static TILDE_TOKEN: u32 = 88; // ~
    pub static AMPERSANDAMPERSAND_TOKEN: u32 = 89; // &&
    pub static BARBAR_TOKEN: u32 = 90; // ||
    pub static QUESTION_TOKEN: u32 = 91; // ?
    pub static COLON_TOKEN: u32 = 92; // :
    pub static EQUALS_TOKEN: u32 = 93; // =
    pub static PLUSEQUALS_TOKEN: u32 = 94; // +=
    pub static MINUSEQUALS_TOKEN: u32 = 95; // -=
    pub static ASTERISKEQUALS_TOKEN: u32 = 96; // *=
    pub static SLASHEQUALS_TOKEN: u32 = 97; // /=
    pub static PERCENTEQUALS_TOKEN: u32 = 98; // %=
    pub static LESSTHANLESSTHANEQUALS_TOKEN: u32 = 99; // <<=
    pub static GREATERTHANGREATERTHANEQUALS_TOKEN: u32 = 100; // >>=
    pub static GREATERTHANGREATERTHANGREATERTHANEQUALS_TOKEN: u32 = 101; // >>>=
    pub static AMPERSANDEQUALS_TOKEN: u32 = 102; // &=
    pub static BAREQUALS_TOKEN: u32 = 103; // |=
    pub static CARETEQUALS_TOKEN: u32 = 104; // ^=
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str,u32> = {
        let mut m = HashMap::new();
        m.insert("ANY_KEYWORD",0); // any
        m.insert("BOOL_EANKEYWORD",1); // boolean
        m.insert("BREAK_KEYWORD",2); // break
        m.insert("CASE_KEYWORD",3); // case
        m.insert("CATCH_KEYWORD",4); // catch
        m.insert("CLASS_KEYWORD",5); // class
        m.insert("CONTINUE_KEYWORD",6); // continue
        m.insert("CONST_KEYWORD",7); // const
        m.insert("CONSTRUCTOR_KEYWORD",8); // constructor
        m.insert("DEBUGGER_KEYWORD",9); // debugger
        m.insert("DECLARE_KEYWORD",10); // declare
        m.insert("DEFAULT_KEYWORD",11); // default
        m.insert("DELETE_KEYWORD",12); // delete
        m.insert("DO_KEYWORD",13); // do
        m.insert("ELSE_KEYWORD",14); // else
        m.insert("ENUM_KEYWORD",15); // enum
        m.insert("EXPORT_KEYWORD",16); // export
        m.insert("EXTENDS_KEYWORD",17); // extends
        m.insert("FALSE_KEYWORD",18); // false
        m.insert("FINALLY_KEYWORD",19); // finally
        m.insert("FOR_KEYWORD",20); // for
        m.insert("FUNCTION_KEYWORD",21); // function
        m.insert("GET_KEYWORD",22); // get
        m.insert("IF_KEYWORD",23); // if
        m.insert("IMPLEMENTS_KEYWORD",24); // implements
        m.insert("IMPORT_KEYWORD",25); // import
        m.insert("IN_KEYWORD",26); // in
        m.insert("INSTANCEOF_KEYWORD",27); // instanceof
        m.insert("INTERFACE_KEYWORD",28); // interface
        m.insert("LET_KEYWORD",29); // let
        m.insert("MODULE_KEYWORD",30); // module
        m.insert("NEW_KEYWORD",31); // new
        m.insert("NULL_KEYWORD",32); // null
        m.insert("NUMBER_KEYWORD",33); // number
        m.insert("PACKAGE_KEYWORD",34); // package
        m.insert("PRIVATE_KEYWORD",35); // private
        m.insert("PROTECTED_KEYWORD",36); // protected
        m.insert("PUBLIC_KEYWORD",37); // public
        m.insert("REQUIRE_KEYWORD",38); // require
        m.insert("RETURN_KEYWORD",39); // return
        m.insert("SET_KEYWORD",40); // set
        m.insert("STATIC_KEYWORD",41); // static
        m.insert("STRING_KEYWORD",42); // string
        m.insert("SUPER_KEYWORD",43); // super
        m.insert("SWITCH_KEYWORD",44); // switch
        m.insert("THIS_KEYWORD",45); // this
        m.insert("THROW_KEYWORD",46); // throw
        m.insert("TRUE_KEYWORD",47); // true
        m.insert("TRY_KEYWORD",48); // try
        m.insert("TYPEOF_KEYWORD",49); // typeof
        m.insert("VAR_KEYWORD",50); // var
        m.insert("VOID_KEYWORD",51); // void
        m.insert("WHILE_KEYWORD",52); // while
        m.insert("WITH_KEYWORD",53); // with
        m.insert("YIELD_KEYWORD",54); // yield
        m
    };
}
