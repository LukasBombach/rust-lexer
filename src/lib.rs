// V8 Tokens
// https://source.chromium.org/chromium/v8/v8.git/+/edf3dab4660ed6273e5d46bd2b0eae9f3210157d:src/token.h

// V8 Scanner
// https://source.chromium.org/chromium/v8/v8.git/+/edf3dab4660ed6273e5d46bd2b0eae9f3210157d:src/scanner.cc;l=422

// MDN Lexical Grammar
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar

// ESTree Specs
// https://github.com/estree/estree

use std::str::CharIndices;

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

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    let mut chars = input.char_indices();

    std::iter::from_fn(move || {

        if let Some(next) = chars.next() {

            let (start, c) = next;
            let end = start + 1;

            println!("> tokenize {:#?}", c);

            return match c {
                '='  => from_eq(input, start, &mut chars),
                '(' => Some(Token::Lparen(&input[start..end])),
                ')' => Some(Token::Rparen(&input[start..end])),
                '[' => Some(Token::Lbrack(&input[start..end])),
                ']' => Some(Token::Rbrack(&input[start..end])),
                '{' => Some(Token::Lbrace(&input[start..end])),
                '}' => Some(Token::Rbrace(&input[start..end])),
                ':' => Some(Token::Colon(&input[start..end])),
                ';' => Some(Token::Semicolon(&input[start..end])),
                '.' => Some(Token::Period(&input[start..end])),
                '?' => Some(Token::Conditional(&input[start..end])),
                _ => Some(Token::Unknown(&input[start..end])),
            }

        }

        None
    })
}

fn from_eq<'a>(input: &'a str, start: usize, chars: &mut CharIndices) -> Option<Token<'a>> {
    if let Some(next) = chars.next() {

        let (i, c) = next;

        println!("> from_eq  {:#?}", c);

        return match c {
            '>'  => Some(Token::Arrow(&input[start..(i + 1)])),
            _ => Some(Token::Assign(&input[start..i])),
        }
    }
    
    Some(Token::Assign(&input[start..(start + 1)]))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn it_tokenizes_an_empty_str() {
        let mut tokens = tokenize("");
        assert_eq!(tokens.next(), None);
    }

    #[test]
    #[ignore]
    fn it_tokenizes_a_single_letter_token() {
        let input = "=";
        let mut tokens = tokenize(input);
        assert_eq!(tokens.next(), Some(Token::Assign(&input[0..1])));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    #[ignore]
    fn it_tokenizes_a_two_letter_token() {
        let input = "=>";
        let mut tokens = tokenize(input);
        assert_eq!(tokens.next(), Some(Token::Arrow(&input[0..2])));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    #[ignore]
    fn it_tokenizes_consecutive_single_letter_tokens() {
        let input = "()=";
        let mut tokens = tokenize(input);
        assert_eq!(tokens.next(), Some(Token::Lparen(&input[0..1])));
        assert_eq!(tokens.next(), Some(Token::Rparen(&input[1..2])));
        assert_eq!(tokens.next(), Some(Token::Assign(&input[2..3])));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn it_tokenizes_a_token_following_a_two_letter_token() {
        let input = "=>()";
        let mut tokens = tokenize(input);
        assert_eq!(tokens.next(), Some(Token::Arrow(&input[0..2])));
        assert_eq!(tokens.next(), Some(Token::Lparen(&input[2..3])));
        assert_eq!(tokens.next(), Some(Token::Rparen(&input[3..4])));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn it_tokenizes_x() {
        let input = "=()";
        let mut tokens = tokenize(input);
        assert_eq!(tokens.next(), Some(Token::Assign(&input[0..1])));
        assert_eq!(tokens.next(), Some(Token::Lparen(&input[1..2])));
        assert_eq!(tokens.next(), Some(Token::Rparen(&input[3..4])));
        assert_eq!(tokens.next(), None);
    }
}
