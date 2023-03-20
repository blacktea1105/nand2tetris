use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::symbol_table::Addr;

lazy_static! {
    static ref DEST_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("M", "001"),
        ("D", "010"),
        ("MD", "011"),
        ("A", "100"),
        ("AM", "101"),
        ("AD", "110"),
        ("AMD", "111"),
    ]);

    static ref COMP_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("0", "0101010"),
        ("1", "0111111"),
        ("-1", "0111010"),
        ("D", "0001100"),
        ("A", "0110000"),
        ("M", "1110000"),
        ("!D", "0001111"),
        ("!A", "0110001"),
        ("!M", "1110001"),
        ("-D", "1001111"),
        ("-A", "0110011"),
        ("-M", "1110011"),
        ("D+1", "0011111"),
        ("A+1", "0110111"),
        ("M+1", "1110111"),
        ("D-1", "0001110"),
        ("A-1", "0110010"),
        ("M-1", "1110010"),
        ("D+A", "0000010"),
        ("D+M", "1000010"),
        ("D-A", "0010011"),
        ("D-M", "1010011"),
        ("A-D", "0000111"),
        ("M-D", "1000111"),
        ("D&A", "0000000"),
        ("D&M", "1000000"),
        ("D|A", "0010101"),
        ("D|M", "1010101"),
    ]);

    static ref JUMP_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("JGT", "001"),
        ("JEQ", "010"),
        ("JGE", "011"),
        ("JLT", "100"),
        ("JNE", "101"),
        ("JLE", "110"),
        ("JMP", "111"),
    ]);
}

pub fn decode_a_command(addr: Addr) -> String {
    format!("{:016b}", u16::from(addr))
}

pub fn decode_c_command(dest: Option<String>, comp: String, jump: Option<String>) -> String {
    let dest = if let Some(dest) = dest {
        DEST_MAP.get(&dest as &str).expect("decode dest({dest}) failed!")
    } else {
        "000"
    };

    let comp = COMP_MAP.get(&comp.as_str()).expect("decode comp({comp}) failed!");

    let jump = if let Some(jump) = jump {
        JUMP_MAP.get(&jump as &str).expect("decode jump({jump}) failed!")
    } else {
        "000"
    };

    format!("111{comp}{dest}{jump}")
}
