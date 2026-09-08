use crate::{
    Qb,
    qb::{parser::Error::UnexpectedToken, token::Token},
};

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("unexpected token: {0:?}")]
    UnexpectedToken(Token),

    #[error("expected {0:?}, received {1:?}")]
    ExpectedButReceived(Token, Token),

    #[error("unexpected EOF")]
    UnexpectedEof,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Node {
    File(Vec<Node>), // Root node
    Script { name: String, nodes: Vec<Node> },
    If(Vec<Node>),
    Structure(Vec<Node>),
    Array(Vec<Node>),
    Parenthesized(Vec<Node>),
    Global(String),
    Symbol(String),
    Assignment(Box<Node>, Box<Node>),
    Token(Token), // A simple token that isn't a larger block construct
}

impl Node {
    fn parse_file(qb: &Qb) -> Result<Self, Error> {
        let mut nodes = Vec::new();
        let mut pos = 0;

        while pos < qb.tokens.len() {
            let (node, new_pos) = Self::parse_any(qb, pos)?;
            pos = new_pos;
            match node {
                Self::Token(Token::Newline) | Self::Token(Token::NewlineDebug(_)) => (),
                _ => nodes.push(node),
            }
        }

        Ok(Self::File(nodes))
    }

    /// Consume tokens until one matches the predicate.
    /// Returns a vec of all consumed tokens, including the one that matched the predicate.
    fn consume_until<P>(qb: &Qb, pos: &mut usize, mut predicate: P) -> Result<Vec<Node>, Error>
    where
        P: FnMut(&Token) -> bool,
    {
        let mut nodes = Vec::new();

        while {
            match qb.tokens.get(*pos) {
                Some(token) if predicate(token) => {
                    *pos += 1;
                    false
                }
                _ => true,
            }
        } {
            let (node, new_pos) = Self::parse_any(qb, *pos)?;
            *pos = new_pos;
            match node {
                Self::Token(Token::Newline) | Self::Token(Token::NewlineDebug(_)) => (),
                _ => nodes.push(node),
            }
        }

        Ok(nodes)
    }

    fn expect_token<P, R>(qb: &Qb, pos: &mut usize, mut predicate: P) -> Result<R, Error>
    where
        P: FnMut(&Token) -> Option<R>,
    {
        match qb.tokens.get(*pos) {
            Some(token) => {
                if let Some(ret) = predicate(token) {
                    *pos += 1;
                    Ok(ret)
                } else {
                    Err(Error::UnexpectedToken(token.clone()))
                }
            }
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_any(qb: &Qb, mut pos: usize) -> Result<(Self, usize), Error> {
        match qb.tokens.get(pos) {
            Some(token) => {
                pos += 1;
                match token {
                    Token::Script => Ok(Self::parse_script(qb, pos)?),
                    Token::If => Ok(Self::parse_if(qb, pos)?),
                    Token::LeftBrace => Ok(Self::parse_structure(qb, pos)?),
                    Token::LeftBracket => Ok(Self::parse_array(qb, pos)?),
                    Token::LeftParen => Ok(Self::parse_parens(qb, pos)?),
                    Token::Global => Ok(Self::parse_global(qb, pos)?),
                    Token::Symbol(checksum) => {
                        let slf = Node::Symbol(qb.symbol_table.get_or_hex(*checksum));
                        match Self::maybe_parse_assignment(qb, pos, &slf)? {
                            Some(n) => Ok(n),
                            None => Ok((slf, pos)),
                        }
                    }
                    Token::EndScript
                    | Token::EndIf
                    | Token::RightBrace
                    | Token::RightBracket
                    | Token::RightParen => Err(UnexpectedToken(token.clone())),
                    _ => Ok((Self::Token(token.clone()), pos)),
                }
            }
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_script(qb: &Qb, mut pos: usize) -> Result<(Self, usize), Error> {
        let checksum = Self::expect_token(qb, &mut pos, |token| match token {
            Token::Symbol(name) => Some(*name),
            _ => None,
        })?;
        let name = qb.symbol_table.get_or_hex(checksum);

        let nodes = Self::consume_until(qb, &mut pos, |token| match token {
            Token::EndScript => true,
            _ => false,
        })?;

        Ok((Self::Script { name, nodes }, pos))
    }

    fn parse_if(qb: &Qb, mut pos: usize) -> Result<(Self, usize), Error> {
        let nodes = Self::consume_until(qb, &mut pos, |token| match token {
            Token::EndIf => true,
            _ => false,
        })?;

        Ok((Self::If(nodes), pos))
    }

    fn parse_structure(qb: &Qb, mut pos: usize) -> Result<(Self, usize), Error> {
        let nodes = Self::consume_until(qb, &mut pos, |token| match token {
            Token::RightBrace => true,
            _ => false,
        })?;

        Ok((Self::Structure(nodes), pos))
    }

    fn parse_array(qb: &Qb, mut pos: usize) -> Result<(Self, usize), Error> {
        let nodes = Self::consume_until(qb, &mut pos, |token| match token {
            Token::RightBracket => true,
            _ => false,
        })?;

        Ok((Self::Array(nodes), pos))
    }

    fn parse_parens(qb: &Qb, mut pos: usize) -> Result<(Self, usize), Error> {
        let nodes = Self::consume_until(qb, &mut pos, |token| match token {
            Token::RightParen => true,
            _ => false,
        })?;

        Ok((Self::Parenthesized(nodes), pos))
    }

    fn parse_global(qb: &Qb, mut pos: usize) -> Result<(Self, usize), Error> {
        let name = Self::expect_token(qb, &mut pos, |token| match token {
            Token::Symbol(name) => Some(*name),
            _ => None,
        })?;

        Ok((Self::Global(qb.symbol_table.get_or_hex(name)), pos))
    }

    fn maybe_parse_assignment(
        qb: &Qb,
        mut pos: usize,
        lhs: &Self,
    ) -> Result<Option<(Self, usize)>, Error> {
        // XXX (ellie): needs to be context-dependent, as `=` is used for both assignment and equality
        match qb.tokens.get(pos) {
            Some(Token::Equals) => {
                pos += 1;

                match qb.tokens.get(pos) {
                    Some(Token::Newline) | Some(Token::NewlineDebug(_)) => {
                        pos += 1;
                    }
                    _ => (),
                }

                let (rhs, new_pos) = Self::parse_any(qb, pos)?;

                Ok(Some((
                    Self::Assignment(Box::new(lhs.clone()), Box::new(rhs)),
                    new_pos,
                )))
            }

            _ => Ok(None),
        }
    }
}

impl TryFrom<&Qb> for Node {
    type Error = Error;

    fn try_from(qb: &Qb) -> Result<Self, Error> {
        Self::parse_file(qb)
    }
}
