pub mod parser;
pub mod symbol_table;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;
use parser::Line;
use symbol_table::{Addr, SymbolTable};

lazy_static! {
    static ref PRE_DEFINED_SYMBOLS: [(String, Addr); 23] = [
        ("R0".to_string(), 0.into()),
        ("R1".to_string(), 1.into()),
        ("R2".to_string(), 2.into()),
        ("R3".to_string(), 3.into()),
        ("R4".to_string(), 4.into()),
        ("R5".to_string(), 5.into()),
        ("R6".to_string(), 6.into()),
        ("R7".to_string(), 7.into()),
        ("R8".to_string(), 8.into()),
        ("R9".to_string(), 9.into()),
        ("R10".to_string(), 10.into()),
        ("R11".to_string(), 11.into()),
        ("R12".to_string(), 12.into()),
        ("R13".to_string(), 13.into()),
        ("R14".to_string(), 14.into()),
        ("R15".to_string(), 15.into()),
        ("SCREEN".to_string(), 16384.into()),
        ("KBD".to_string(), 24576.into()),
        ("SP".to_string(), 0.into()),
        ("LCL".to_string(), 1.into()),
        ("ARG".to_string(), 2.into()),
        ("THIS".to_string(), 3.into()),
        ("THAT".to_string(), 4.into()),
    ];

    static ref VAR_MEMORY_BASE: Addr = 16.into();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no asm file path argument");
    }

    let file_path = &args[1];

    // check file name is *.asm format
    if !is_asm_file(file_path) {
        panic!("{file_path} is not *.asm file");
    }

    let mut symbol_table = SymbolTable::new(PRE_DEFINED_SYMBOLS.clone(), VAR_MEMORY_BASE.clone());

    // first pass
    line_with_machine_addr(parser::parse(file_path)?)
        .for_each(|(line, machine_addr)| {
            if let Line::Label { name } = &line {
                symbol_table.set_label(name.clone(), machine_addr.into());
            }
        });

    // open hack file
    let hack_path = get_hack_path(file_path);
    let hack_file = File::create(hack_path)?;
    let mut hack_file = LineWriter::new(hack_file);

    // second pass
    line_with_machine_addr(parser::parse(file_path)?)
        .filter(|(line, machine_addr)| match line {
            Line::WhiteSpace | Line::Label { .. } => false,
            _ => true,
        })
        .for_each(|(line, machine_addr)| {

            let machine_code = match line {
                Line::ACommand { symbol } => {
                    let addr: Addr = if let Some(addr) = try_convert_numeric_addr(&symbol) {
                        addr
                    } else {
                        if let Some(&addr) = symbol_table.get_addr(&symbol) {
                            addr
                        } else {
                            *symbol_table.set_var_memory(symbol.clone())
                        }
                    };

                    decode_a_command(addr)
                },
                Line::CCommand { dest, comp, jump } => {
                    decode_c_command(dest, comp, jump)
                },
                _ => panic!("get machine code failed!"),
            };

            // write machine code to hack file
            hack_file.write_all(format!("{machine_code}\n").as_bytes());
        });

    Ok(())
}

fn is_asm_file(path: &str) -> bool {
    match Path::new(path).extension() {
        Some(ext) => {
            ext == "asm"
        },
        None => false,
    }
}

fn get_hack_path(asm_path: &str) -> String {
    let asm_path = Path::new(asm_path);

    asm_path.parent()
        .expect("get parent dir failed!")
        .clone()
        .join(format!("{}.hack", asm_path.file_stem().expect("get file stem failed!").to_str().expect("convert file stem (OS str) failed!")))
        .to_str()
        .expect("hack path convert to str failed!")
        .to_string()
    
}

fn line_with_machine_addr(iterator: impl Iterator<Item=Line>) -> impl Iterator<Item=(Line, u16)> {
    
    iterator.filter(|line| match line {
            Line:: WhiteSpace => false,
            _ => true,
        })
        .scan(0, |machine_addr, line| {
            let current_addr = *machine_addr;

            match &line {
                Line::Label { .. } => (),
                _ => {
                    *machine_addr += 1;
                },
            };
            // if ! let Line::Label { .. } = &line {
            // } else {
                
            // }

            Some((line, current_addr))
        })
}

fn try_convert_numeric_addr(symbol: &str) -> Option<Addr> {
    lazy_static! {
        static ref ADDR_RE: Regex = Regex::new(r"^\d+$").unwrap();
    }

    if ADDR_RE.is_match(&symbol) {
        Some(
            u16::from_str_radix(symbol, 10)
            .expect("parse addr failed!")
            .into()
        )
    } else {
        None
    }
}

fn decode_a_command(addr: Addr) -> String {
    format!("{:016b}", u16::from(addr))
}

fn decode_c_command(dest: Option<String>, comp: String, jump: Option<String>) -> String {
    let dest = if let Some(dest) = dest {
        match dest.as_str() {
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            _ => panic!("decode dest({dest}) failed!"),
        }
    } else {
        "000"
    };

    let comp = match comp.as_str() {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001111",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "1001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        _ => panic!("decode comp({comp}) failed!"),
    };

    let jump = if let Some(jump) = jump {
        match jump.as_str() {
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => panic!("decode jump({jump}) failed!"),
        }
    } else {
        "000"
    };

    format!("111{comp}{dest}{jump}")
}
