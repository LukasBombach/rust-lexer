#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Eq(&'a str),
    Unknown(&'a str),
}

pub fn tokenize(input: &str)  -> impl Iterator<Item = Token> {
    let mut chars = input.chars();
    std::iter::from_fn(move || {
            match chars.next() {
                Some('=') => Some(Token::Eq(&input[0..1])),
                None => None,
                _ => Some(Token::Unknown(&input[0..1])),
            }
    })
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn it_tokenizes_an_empty_str() {
        let mut tokens = tokenize("");
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn it_tokenizes_eq() {
        let input = "=";
        let mut tokens = tokenize(input);
        assert_eq!(tokens.next(), Some(Token::Eq(&input[0..1])));
        assert_eq!(tokens.next(), None);
    }
}
