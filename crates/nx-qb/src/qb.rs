use encoding_rs::WINDOWS_1252;
use nx_common::{Readable, Writable};

use crate::{Error, symbols::SymbolTable, token::Token};

// XXX (ellie): organize imports
// pub mod nodearray;

#[derive(Debug)]
#[allow(unused)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Qb {
    pub tokens: Vec<Token>,
    pub symbol_table: SymbolTable,
}

impl Readable for Qb {
    type Error = Error;
    type ReadContext = ();

    fn read(reader: &mut impl nx_common::Reader, _: &mut ()) -> Result<Self, Error> {
        let mut tokens = Vec::new();
        let mut symbol_table = SymbolTable::new();

        while {
            // do: get token
            match Token::read(reader)? {
                Token::Terminator => false,
                token => {
                    if let Token::SymbolDef(checksum, name) = &token {
                        let (s, _, _) = WINDOWS_1252.decode(&name);
                        symbol_table.add(*checksum, s.to_string());
                    }
                    tokens.push(token);
                    true
                }
            }

            // while token is not terminator
        } {}

        Ok(Qb {
            tokens,
            symbol_table,
        })
    }
}

impl Writable for Qb {
    type Error = Error;
    type WriteContext = ();

    fn write(&self, writer: &mut impl nx_common::Writer, _: &mut ()) -> Result<(), Error> {
        for token in self.tokens.iter() {
            token.write(writer)?;
        }
        Token::Terminator.write(writer)?;
        Ok(())
    }
}

impl Qb {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();
        for token in self.tokens.iter() {
            token.to_string(&mut buffer, &self.symbol_table);
        }
        buffer
    }
}
