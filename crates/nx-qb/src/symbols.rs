use std::collections::BTreeMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolTable {
    symbols: BTreeMap<u32, String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, checksum: u32, name: String) {
        self.symbols.insert(checksum, name);
    }

    pub fn get(&self, checksum: u32) -> Option<&String> {
        self.symbols.get(&checksum)
    }

    pub fn get_or_hex(&self, checksum: u32) -> String {
        self.symbols
            .get(&checksum)
            .cloned()
            .unwrap_or(format!("{:#08x}", checksum))
    }
}
