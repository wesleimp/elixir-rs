#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: String,
    pub ty: TokenType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Atom,
    Boolean,
    Char,
    Comma,
    Comment,
    Identifier,
    NewLine,
    Number,
    Operator,
    Quote,
    WhiteSpace,

    // Identifiers
    Alias,
    And,
    Break,
    Cond,
    Def,
    Defmacro,
    Defmodule,
    Do,
    Doc,
    Else,
    ElseIf,
    End,
    False,
    For,
    If,
    Import,
    In,
    ModuleDoc,
    Nil,
    Not,
    Or,
    Require,
    Spec,
    Then,
    True,
    Type,
    Unless,
    Use,
    Var,

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // [
    RightBrace,   // ]
    LeftBracket,  // {
    RightBracket, // }
    Percent,      // %
}

impl Token {
    pub fn new(kind: TokenType, value: String) -> Token {
        Token { ty: kind, value }
    }

    pub fn ty(&self) -> TokenType {
        self.ty.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl TokenType {
    pub fn is_atom(&self) -> bool {
        matches!(self, TokenType::Atom)
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, TokenType::Boolean)
    }

    pub fn is_char(&self) -> bool {
        matches!(self, TokenType::Char)
    }

    pub fn is_comment(&self) -> bool {
        matches!(self, TokenType::Comment)
    }

    pub fn is_comma(&self) -> bool {
        matches!(self, TokenType::Comma)
    }

    pub fn is_delimiter(&self) -> bool {
        matches!(
            self,
            TokenType::LeftParen
                | TokenType::RightParen
                | TokenType::LeftBrace
                | TokenType::RightBrace
                | TokenType::LeftBracket
                | TokenType::RightBracket
                | TokenType::Percent
        )
    }

    pub fn is_identifier(&self) -> bool {
        matches!(
            self,
            TokenType::Alias
                | TokenType::And
                | TokenType::Break
                | TokenType::Cond
                | TokenType::Def
                | TokenType::Defmacro
                | TokenType::Defmodule
                | TokenType::Do
                | TokenType::Doc
                | TokenType::Else
                | TokenType::ElseIf
                | TokenType::End
                | TokenType::False
                | TokenType::For
                | TokenType::If
                | TokenType::Import
                | TokenType::In
                | TokenType::ModuleDoc
                | TokenType::Nil
                | TokenType::Not
                | TokenType::Or
                | TokenType::Require
                | TokenType::Spec
                | TokenType::Then
                | TokenType::True
                | TokenType::Type
                | TokenType::Unless
                | TokenType::Use
                | TokenType::Var
                | TokenType::Identifier
        )
    }

    pub fn is_newline(&self) -> bool {
        matches!(self, TokenType::NewLine)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, TokenType::Number)
    }

    pub fn is_operator(&self) -> bool {
        matches!(self, TokenType::Operator)
    }

    pub fn is_quote(&self) -> bool {
        matches!(self, TokenType::Quote)
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self, TokenType::WhiteSpace)
    }
}
