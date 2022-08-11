// V8 Tokens
// https://source.chromium.org/chromium/v8/v8.git/+/edf3dab4660ed6273e5d46bd2b0eae9f3210157d:src/token.h

// V8 Scanner
// https://source.chromium.org/chromium/v8/v8.git/+/edf3dab4660ed6273e5d46bd2b0eae9f3210157d:src/scanner.cc;l=422

// MDN Lexical Grammar
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar

// ESTree Specs
// https://github.com/estree/estree

mod token;

use std::str::CharIndices;
use std::iter::Peekable;
use token::Token;
use token::Token::*;

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    let mut reader = input.char_indices().peekable();

    std::iter::from_fn(move || {

        if let Some(next) = reader.next() {

            let (start, c) = next;
            let end = start + 1;

            return match c {
                '='  => from_eq(input, start, &mut reader),
                '(' => Some(Lparen(&input[start..end])),
                ')' => Some(Rparen(&input[start..end])),
                '[' => Some(Lbrack(&input[start..end])),
                ']' => Some(Rbrack(&input[start..end])),
                '{' => Some(Lbrace(&input[start..end])),
                '}' => Some(Rbrace(&input[start..end])),
                ':' => Some(Colon(&input[start..end])),
                ';' => Some(Semicolon(&input[start..end])),
                '.' => Some(Period(&input[start..end])),
                '?' => Some(Conditional(&input[start..end])),
                _ => Some(Unknown(&input[start..end])),
            }

        }

        None
    })
}

fn from_eq<'a>(input: &'a str, start: usize, reader: &mut Peekable<CharIndices>) -> Option<Token<'a>> {
    if let Some(next) = reader.peek() {

        let (i, c) = next;

        return match c {
            '>'  => {
                let v = Some(Arrow(&input[start..(i + 1)]));
                reader.next();
                return v;
            },
            _ => Some(Assign(&input[start..*i])),
        }
    }
    
    Some(Assign(&input[start..(start + 1)]))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_tokenizes_an_empty_str() {
        let mut tokens = tokenize("");
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn it_tokenizes_a_single_letter_token() {
        let input = "=";
        let mut tokens = tokenize(input);
        assert_eq!(Some(Assign(&input[0..1])), tokens.next());
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn it_tokenizes_a_two_letter_token() {
        let input = "=>";
        let mut tokens = tokenize(input);
        assert_eq!(Some(Arrow(&input[0..2])), tokens.next());
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn it_tokenizes_consecutive_single_letter_tokens() {
        let input = "()=";
        let mut tokens = tokenize(input);
        assert_eq!(Some(Lparen(&input[0..1])), tokens.next());
        assert_eq!(Some(Rparen(&input[1..2])), tokens.next());
        assert_eq!(Some(Assign(&input[2..3])), tokens.next());
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn it_tokenizes_a_token_following_a_two_letter_token() {
        let input = "=>()";
        let mut tokens = tokenize(input);
        assert_eq!(Some(Arrow(&input[0..2])), tokens.next());
        assert_eq!(Some(Lparen(&input[2..3])), tokens.next());
        assert_eq!(Some(Rparen(&input[3..4])), tokens.next());
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn it_tokenizes_x() {
        let input = "=()";
        let mut tokens = tokenize(input);
        assert_eq!(Some(Assign(&input[0..1])), tokens.next());
        assert_eq!(Some(Lparen(&input[1..2])), tokens.next());
        assert_eq!(Some(Rparen(&input[2..3])), tokens.next());
        assert_eq!(None, tokens.next());
    }
}
