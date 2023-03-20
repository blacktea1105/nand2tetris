pub mod code;
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
        // filter white space
        .filter(line_with_num_not_white_space)
        .for_each(|LineWithAddr { line, addr }| {
            // set label address in the symbol table
            if let Line::Label { name } = &line {
                symbol_table.set_label(name.clone(), addr.unwrap());
            }
        });

    // open hack file
    let hack_path = get_hack_path(file_path);
    let hack_file = File::create(hack_path)?;
    let mut hack_file = LineWriter::new(hack_file);

    // second pass
    line_with_machine_addr(parser::parse(file_path)?)
        // filter white space
        .filter(line_with_num_not_white_space)
        // filter label
        .filter(line_with_num_not_label)
        .for_each(|LineWithAddr { line, addr: _ }| {

            // set variable address
            if let Line::ACommand { ref symbol } = line {
                if try_convert_numeric_addr(&symbol).is_none() {
                    if symbol_table.get_addr(&symbol).is_none() {
                        symbol_table.set_var_memory(symbol.clone());
                    }
                }
            }

            let machine_code = match line {
                Line::ACommand { ref symbol } => {
                    let addr: Addr = if let Some(addr) = try_convert_numeric_addr(&symbol) {
                        addr
                    } else {
                        *symbol_table.get_addr(&symbol).unwrap()
                    };

                    code::decode_a_command(addr)
                },
                Line::CCommand { dest, comp, jump } => {
                    code::decode_c_command(dest, comp, jump)
                },
                _ => panic!("get machine code failed!"),
            };

            // write machine code to hack file
            hack_file.write_all(format!("{machine_code}\n").as_bytes()).expect("write hack file error!");
        });

    Ok(())
}

struct LineWithAddr {
    pub line: Line,
    pub addr: Option<Addr>,
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

fn line_with_machine_addr(iterator: impl Iterator<Item=Line>) -> impl Iterator<Item=LineWithAddr> {
    
    iterator.scan(0_u16, |machine_addr, line| {
        let current_addr = *machine_addr;

        // next addr
        match &line {
            Line::Label { .. } | Line::WhiteSpace => (),
            _ => {
                *machine_addr += 1;
            },
        };

        let addr = if let Line::WhiteSpace = line {
            None
        } else {
            Some(current_addr.into())
        };

        Some(LineWithAddr { line, addr })
    })
}

fn line_with_num_not_white_space(LineWithAddr { line, addr: _ }: &LineWithAddr) -> bool {
    if let Line::WhiteSpace = line {
        false
    } else {
        true
    }
}

fn line_with_num_not_label(LineWithAddr { line, addr: _ }: &LineWithAddr) -> bool {
    if let Line::Label { .. } = line {
        false
    } else {
        true
    }
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
