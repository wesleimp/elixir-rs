use self::token::{Token, TokenKind};

mod token;

pub struct Lexer {
    cursor: usize,
    input: Vec<char>,
}

impl Lexer {
    pub fn new(string: &str) -> Lexer {
        Lexer {
            cursor: 0,
            input: string.chars().collect(),
        }
    }

    pub fn read_char(&mut self) -> Option<&char> {
        if let Some(ch) = self.input.get(self.cursor) {
            self.cursor += 1;
            return Some(ch);
        }

        None
    }

    fn read_while<F>(&mut self, mut func: F) -> Option<String>
    where
        F: FnMut(&char) -> bool,
    {
        let mut string = String::new();
        while let Some(next) = self.peek() {
            if !func(next) {
                break;
            }
            let ch = self.read_char()?;
            string.push(*ch)
        }

        if string.is_empty() {
            return None;
        }

        Some(string)
    }

    pub fn peek(&self) -> Option<&char> {
        self.input.get(self.cursor)
    }

    pub fn peek_ahead(&self, n: usize) -> Option<&char> {
        self.input.get(self.cursor + n)
    }

    fn is_done(&self) -> bool {
        self.cursor >= self.input.len()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let peek = self.peek()?;

        match peek {
            '#' => read_comment(self),
            ',' => read_comma(self),
            ':' => read_atom(self),
            '?' => read_char(self),
            ch if is_newline(ch) => read_with(self, TokenKind::NewLine),
            ch if is_quote(ch) => read_with(self, TokenKind::Quote),
            ch if is_delim(ch) => read_with(self, TokenKind::Delimiter),
            ch if is_operator(ch) => read_operator(self),
            ch if ch.is_numeric() => read_number(self),
            ch if is_identifier(ch) => read_identifier(self),
            ch if ch.is_whitespace() => read_whitespace(self),
            _ => None,
        }
    }
}

fn read_whitespace(lexer: &mut Lexer) -> Option<Token> {
    let value = lexer.read_while(&|ch: &char| ch.is_whitespace())?;
    Some(Token::new(TokenKind::WhiteSpace, value))
}

fn read_identifier(lexer: &mut Lexer) -> Option<Token> {
    let ident = lexer.read_while(is_identifier)?;

    if is_bool_literal(&ident) {
        return Some(Token::new(TokenKind::Boolean, ident));
    }
    return Some(Token::new(TokenKind::Identifier, ident));
}

fn is_identifier(ch: &char) -> bool {
    ((ch.is_alphanumeric() || is_extra_literal(ch)) || !ch.is_ascii_punctuation())
        && !ch.is_whitespace()
}

fn is_bool_literal(ident: &str) -> bool {
    ident.eq("true") || ident.eq("false") || ident.eq("nil")
}

fn read_number(lexer: &mut Lexer) -> Option<Token> {
    let value = lexer.read_while(is_number)?;
    Some(Token::new(TokenKind::Number, value))
}

fn read_operator(lexer: &mut Lexer) -> Option<Token> {
    let value = lexer.read_while(is_operator)?;
    Some(Token::new(TokenKind::Operator, value))
}

fn read_with(lexer: &mut Lexer, tok: TokenKind) -> Option<Token> {
    Some(Token::new(tok, lexer.read_char()?.to_string()))
}

fn read_char(lexer: &mut Lexer) -> Option<Token> {
    let value = lexer.read_while(is_char)?;
    Some(Token::new(TokenKind::Char, value))
}

fn read_atom(lexer: &mut Lexer) -> Option<Token> {
    let next = lexer.peek_ahead(1)?;
    if next.is_alphanumeric() || is_quote(next) {
        let value = lexer.read_while(is_atom)?;
        return Some(Token::new(TokenKind::Atom, value));
    }

    read_operator(lexer)
}

fn is_number(ch: &char) -> bool {
    ch.is_ascii_alphanumeric() || ch.eq(&'.')
}

fn is_operator(ch: &char) -> bool {
    ch.is_ascii_punctuation()
        && !ch.eq(&'`')
        && !ch.eq(&'_')
        && !ch.eq(&'@')
        && !ch.eq(&',')
        && !ch.eq(&';')
        && !ch.eq(&'#')
        || ch.eq(&':')
}

fn is_delim(ch: &char) -> bool {
    matches!(ch, '[' | ']' | '(' | ')' | '{' | '}' | '%')
}

fn is_char(ch: &char) -> bool {
    ch.eq(&'?') || ch.is_alphanumeric()
}

fn is_quote(ch: &char) -> bool {
    matches!(ch, '"' | '\'')
}

fn is_atom(ch: &char) -> bool {
    ch.eq(&':') || ch.eq(&'"') || is_extra_literal(ch) || ch.is_alphanumeric() || ch.is_whitespace()
}

fn is_extra_literal(ch: &char) -> bool {
    ch.eq(&'_')
        || ch.eq(&'@')
        || ch.eq(&'?')
        || ch.eq(&'!')
        || ch.eq(&'{')
        || ch.eq(&'%')
        || ch.eq(&'}')
        || ch.eq(&'.')
}

fn read_comma(lexer: &mut Lexer) -> Option<Token> {
    let value = lexer.read_char()?.to_string();
    Some(Token::new(TokenKind::Comma, value))
}

fn read_comment(lexer: &mut Lexer) -> Option<Token> {
    let comment = lexer.read_while(|ch| !is_newline(ch))?;
    Some(Token::new(token::TokenKind::Comment, comment))
}

fn is_newline(ch: &char) -> bool {
    matches!(ch, '\n' | '\t' | '\r')
}

#[cfg(test)]
mod comment {
    use super::*;

    #[test]
    fn shoud_read_comment() {
        let comment = "# this is a comment";
        let token = Lexer::new(comment).next().unwrap();
        assert!(token.kind().is_comment());
        assert_eq!(token.value, comment.to_string())
    }

    #[test]
    fn shoud_read_atom() {
        let atom = ":hello";
        let token = Lexer::new(atom).next().unwrap();
        assert!(token.kind().is_atom());
        assert_eq!(token.value, atom.to_string())
    }

    #[test]
    fn shoud_read_atom_with_quotes() {
        let atom = r#":"hello""#;
        let token = Lexer::new(atom).next().unwrap();
        assert!(token.kind().is_atom());
        assert_eq!(token.value, atom.to_string())
    }

    #[test]
    fn shoud_read_atom_with_quotes_and_whitespace() {
        let atom = r#":"foo bar""#;
        let token = Lexer::new(atom).next().unwrap();
        assert!(token.kind().is_atom());
        assert_eq!(token.value, atom.to_string())
    }

    #[test]
    fn shoud_read_char() {
        let value = "?á";
        let token = Lexer::new(value).next().unwrap();
        assert!(token.kind().is_char());
        assert_eq!(token.value, value.to_string())
    }

    #[test]
    fn shoud_read_new_line() {
        let value = "\n";
        let token = Lexer::new(value).next().unwrap();
        assert!(token.kind().is_newline());
        assert_eq!(token.value, value.to_string())
    }

    #[test]
    fn shoud_read_quotes() {
        let double = "\"";
        let token = Lexer::new(double).next().unwrap();
        assert!(token.kind().is_quote());
        assert_eq!(token.value, double.to_string());

        let single = "\'";
        let token = Lexer::new(single).next().unwrap();
        assert!(token.kind().is_quote());
        assert_eq!(token.value, single.to_string())
    }

    #[test]
    fn shoud_read_delimiters() {
        let delims = "[]{}()%";
        let mut lex = Lexer::new(delims);
        while !lex.is_done() {
            let token = lex.next().unwrap();
            assert!(token.kind().is_delimiter());
        }
    }

    #[test]
    fn should_read_operator() {
        let ops = r#"- + / ^ ^^^ &&& & \\\ * ** ! && <- || ||| == != =~ === !== < > <= >= |> <<< >>> <<~ ~>> <~ ~> <~> <|> +++ --- <> ++ -- => :: | // .. ."#;
        let mut lex = Lexer::new(ops);

        while !lex.is_done() {
            let token = lex.next().unwrap();
            let kind = token.kind();

            if kind.is_whitespace() || kind.is_newline() {
                continue;
            }

            assert!(token.kind().is_operator());
        }
    }

    #[test]
    fn should_read_int() {
        let int = "40";
        let token = Lexer::new(int).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.value(), int.to_string());
    }

    #[test]
    fn should_read_float() {
        let float = "11.45";
        let token = Lexer::new(float).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.value(), float.to_string());
    }

    #[test]
    fn should_read_sci_float() {
        let sci_f = "1.11e10";
        let token = Lexer::new(sci_f).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.value(), sci_f.to_string());
    }

    #[test]
    fn should_read_bin() {
        let bin = "0b1010";
        let token = Lexer::new(bin).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.value(), bin.to_string());
    }

    #[test]
    fn should_read_octal() {
        let oct = "0o17";
        let token = Lexer::new(oct).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.value(), oct.to_string());
    }

    #[test]
    fn should_read_hexa() {
        let hex = "0xFFF";
        let token = Lexer::new(hex).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.value(), hex.to_string());
    }

    #[test]
    fn should_read_identifier() {
        let value = "defmodule";
        let token = Lexer::new(value).next().unwrap();
        assert!(token.kind().is_identifier());
        assert_eq!(token.value(), value.to_string());
    }

    #[test]
    fn should_read_mod_identifier() {
        let value = "@spec";
        let token = Lexer::new(value).next().unwrap();
        assert!(token.kind().is_identifier());
        assert_eq!(token.value(), value.to_string());
    }

    #[test]
    fn should_read_ignored_identifier() {
        let value = "_ignored";
        let token = Lexer::new(value).next().unwrap();
        assert!(token.kind().is_identifier());
        assert_eq!(token.value(), value.to_string());
    }
}
