use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace,
    FnKw,
    VarKw,
    Ident,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comment,
    Error,
    Root,
    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
    VariableDef,
    VariableRef,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::FnKw => Self::FnKw,
            TokenKind::VarKw => Self::VarKw,
            TokenKind::Ident => Self::Ident,
            TokenKind::Number => Self::Number,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::Comment => Self::Comment,
            TokenKind::Error => Self::Error,
        }
    }
}

pub type SyntaxNode = rowan::SyntaxNode<GlueShellLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GlueShellLanguage {}

impl rowan::Language for GlueShellLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
