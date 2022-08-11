#[derive(PartialEq, Debug)]
pub enum Token<'a> {

    // Punctuators
    Lparen(&'a str),         // (
    Rparen(&'a str),         // )
    Lbrack(&'a str),         // [
    Rbrack(&'a str),         // ]
    Lbrace(&'a str),         // {
    Rbrace(&'a str),         // }
    Colon(&'a str),          // :
    Semicolon(&'a str),      // ;
    Period(&'a str),         // .
    Conditional(&'a str),    // ?
    Inc(&'a str),            // ++
    Dec(&'a str),            // --
    Arrow(&'a str),          // =>

    // Assignment operators
    Assign(&'a str),         // =
    AssignBitOr(&'a str),    // |=
    AssignBitXor(&'a str),   // ^=
    AssignBitAnd(&'a str),   // &=
    AssignShl(&'a str),      // <<=
    AssignSar(&'a str),      // >>=
    AssignShr(&'a str),      // >>>=
    AssignAdd(&'a str),      // +=
    AssignSub(&'a str),      // -=
    AssignMul(&'a str),      // *=
    AssignDiv(&'a str),      // /=
    AssignMod(&'a str),      // %=
 
    // Reserved Words
    Await(&'a str),
    Break(&'a str),
    Case(&'a str),
    Catch(&'a str),
    Class(&'a str),
    Const(&'a str),
    Continue(&'a str),
    Debugger(&'a str),
    Default(&'a str),
    Delete(&'a str),
    Do(&'a str),
    Else(&'a str),
    Export(&'a str),
    Extends(&'a str),
    False(&'a str),
    Finally(&'a str),
    For(&'a str),
    Function(&'a str),
    If(&'a str),
    Import(&'a str),
    In(&'a str),
    Instanceof(&'a str),
    Let(&'a str),
    New(&'a str),
    Null(&'a str),
    Return(&'a str),
    Super(&'a str),
    Switch(&'a str),
    This(&'a str),
    Throw(&'a str),
    True(&'a str),
    Try(&'a str),
    Typeof(&'a str),
    Var(&'a str),
    Void(&'a str),
    While(&'a str),
    With(&'a str),
    Yield(&'a str),

    // When none other matches
    Unknown(&'a str),
}