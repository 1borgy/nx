use std::{fmt::Debug, io::Write};

use nx_common::Reader;

use crate::Error;
use crate::component::{Component, Kind};
use crate::id::Id;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Structure(Vec<Component>);

impl Structure {
    pub fn new(symbols: Vec<Component>) -> Self {
        Self(symbols)
    }

    pub fn read(reader: &mut impl Reader) -> Result<Self, Error> {
        let mut symbols = vec![];

        while {
            // do:
            // Read symbol from the reader
            let symbol = Component::read(reader)?;
            let kind = symbol.kind;

            // while:
            // The symbol is not none
            match kind {
                Kind::None => false,
                _ => {
                    // Only push the symbol if it is non-none
                    symbols.push(symbol);
                    true
                }
            }
        } {}

        Ok(Self::new(symbols))
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        for symbol in &self.0 {
            symbol.write(writer)?;
        }

        // Each structure is terminated with a none symbol
        Component::none().write(writer)?;

        Ok(())
    }

    pub fn raw_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        self.write(&mut bytes)?;

        Ok(bytes)
    }

    pub fn get(&self, id: Id) -> Option<&Component> {
        self.0.iter().find(|symbol| symbol.id == id)
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut Component> {
        self.0.iter_mut().find(|symbol| symbol.id == id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, symbol: Component) -> Option<Component> {
        match self.get_mut(symbol.id) {
            Some(existing) => {
                let ret = existing.clone();
                *existing = symbol;
                Some(ret)
            }
            None => {
                self.0.push(symbol);
                None
            }
        }
    }

    pub fn remove(&mut self, id: Id) {
        self.0.retain(|symbol| symbol.id != id);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Component> {
        self.0.iter()
    }
}

impl FromIterator<Component> for Structure {
    fn from_iter<T: IntoIterator<Item = Component>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}
