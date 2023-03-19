use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Addr(u16);

impl Addr {
    pub fn new(value: u16) -> Self {
        if value > 0b01111111_11111111 {
            panic!("addr value must in 15-bit");
        }

        Self(value)
    }
}

impl From<u16> for Addr {
    fn from(item: u16) -> Self {
        Self::new(item)
    }
}

impl From<Addr> for u16 {
    fn from(item: Addr) -> Self {
        item.0
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, Addr>,
    var_memory_point: Addr,
}

impl SymbolTable {
    pub fn new<const N: usize>(init: [(String, Addr); N], var_memory_base: Addr) -> Self {
        let table = HashMap::from(init);

        Self {
            table,
            var_memory_point: var_memory_base,
        }
    }

    pub fn set_label(&mut self, name: String, addr: Addr) {
        self.table.entry(name).or_insert(addr);
    }

    pub fn get_addr(&self, symbol: &str) -> Option<&Addr> {
        self.table.get(symbol)
    }

    pub fn set_var_memory(&mut self, symbol: String) -> &Addr {
        self.table.entry(symbol)
            .or_insert_with(|| {
                let cur_point: u16 = self.var_memory_point.into();

                self.var_memory_point = (cur_point + 1).into();

                cur_point.into()
            })
    }
}
