use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Arithmetic {
    Add, Sub, Neg,
    Eq, Gt, Lt,
    And, Or, Not,
}

#[derive(Debug)]
pub enum MemorySegment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug)]
pub enum Command {
    Arithmetic(Arithmetic),
    Push{ segment: MemorySegment, index: u16 },
    Pop{ segment: MemorySegment, index: u16 },
}

#[derive(Debug)]
pub struct Line {
    pub command: Command,
    pub line: String,
}

pub fn parse(vm_path: &str) -> Result<impl Iterator<Item=Line>, std::io::Error> {
    // open file
    let f = File::open(vm_path)?;
    let f = BufReader::new(f);

    // file read stream
    let iterator = f.lines()
        .map(handle_line_or_panic)
        .filter(is_not_white_space)
        .map(parse_line);

    Ok(iterator)
}

fn handle_line_or_panic(line: Result<String, std::io::Error>) -> String {
    match line {
        Ok(s) => s,
        Err(error) => panic!("error: {error}"),
    }
}

fn is_not_white_space(line: &String) -> bool {
    !remove_comment(line.to_string()).trim().is_empty()
}

fn parse_line(line: String) -> Line {
    let line_without_comment_trim = remove_comment(line.clone()).trim();
    let command = parse_command(line.clone());

    if command.is_err() {
        panic!("{:?}", command.unwrap_err());
    }
    let command = command.unwrap();

    Line { command, line }
}

fn parse_command(line: String) -> Result<Command, &'static str> {
    
    if is_push_pop(&line) {
        return parse_push_pop(&line);
    }

    if let Some(command) = try_parse_arithmetic(&line) {
        return Ok(command);
    }

    Err("unknown command")
}

fn is_push_pop(line: &str) -> bool {
    line.starts_with("push") || line.starts_with("pop")
}

fn parse_push_pop(line: &str) -> Result<Command, &'static str> {
    lazy_static! {
        static ref PUSH_POP_RE: Regex = Regex::new(r"^(push|pop)\s+(\w+)\s+(\d+)$").unwrap();
    }

    let captured = PUSH_POP_RE.captures(line);
    if captured.is_none() {
        return Err("can not parse line to push/pop command");
    }
    let captured = captured.unwrap();

    let segment = match &captured[2] {
        "local" => Ok(MemorySegment::Local),
        "argument" => Ok(MemorySegment::Argument),
        "this" => Ok(MemorySegment::This),
        "that" => Ok(MemorySegment::That),
        "constant" => Ok(MemorySegment::Constant),
        "static" => Ok(MemorySegment::Static),
        "pointer" => Ok(MemorySegment::Pointer),
        "temp" => Ok(MemorySegment::Temp),
        _ => Err("can not parse memory segment of push/pop command"),
    }?;

    let index = u16::from_str_radix(&captured[3], 10)
        .or_else(|_| Err("can not parse index of push/pop command"))?;

    match &captured[1] {
        "push" => Ok(Command::Push { segment, index }),
        "pop" => {
            if let MemorySegment::Constant = segment {
                Err("constant segment can not be a pop command")
            } else {
                Ok(Command::Pop { segment, index })
            }
        },
        _ => Err("can not parse line to push/pop command"),
    }
}

fn remove_comment(line: String) -> String {
    if let Some((line, _)) = line.split_once("//") {
        line.to_string()
    } else {
        line
    }
}

fn try_parse_arithmetic(line: &str) -> Option<Command> {
    match line {
        "add" => Some(Command::Arithmetic(Arithmetic::Add)),
        "sub" => Some(Command::Arithmetic(Arithmetic::Sub)),
        "neg" => Some(Command::Arithmetic(Arithmetic::Neg)),
        "eq" => Some(Command::Arithmetic(Arithmetic::Eq)),
        "gt" => Some(Command::Arithmetic(Arithmetic::Gt)),
        "lt" => Some(Command::Arithmetic(Arithmetic::Lt)),
        "and" => Some(Command::Arithmetic(Arithmetic::And)),
        "or" => Some(Command::Arithmetic(Arithmetic::Or)),
        "not" => Some(Command::Arithmetic(Arithmetic::Not)),
        _ => None,
    }
}
