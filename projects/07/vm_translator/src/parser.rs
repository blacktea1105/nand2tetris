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

#[derive(Debug)]
struct CommandString {
    command: String,
    args: ArgsString,
}

#[derive(Debug)]
struct ArgsString {
    arg1: Option<String>,
    arg2: Option<String>,
}

impl CommandString {
    fn build(line: &str) -> Result<Self, &'static str> {
        lazy_static! {
            static ref COMMAND_RE: Regex = Regex::new(r"^(\w+)(?:\s+(\w+)(?:\s+(\d+))?)?$").unwrap();
        }

        let captured = COMMAND_RE.captures(&line);

        if captured.is_none() {
            return Err("parse command failed!");
        }
        let captured = captured.unwrap();

        if captured.len() < 2 {
            return Err("parse command failed!");
        }

        Ok(Self {
            command: captured[1].to_string(),
            args: ArgsString {
                arg1: captured.get(2).map(|s| s.as_str().into()),
                arg2: captured.get(3).map(|s| s.as_str().into()),
            },
        })
    }
}

pub fn parse(vm_path: &str) -> Result<impl Iterator<Item=Line>, std::io::Error> {
    // open file
    let f = File::open(vm_path)?;
    let f = BufReader::new(f);

    // file read stream
    let iterator = f.lines()
        .map(handle_result_ioerr_or_panic)
        .filter(is_not_white_space)
        .map(parse_line);

    Ok(iterator)
}

fn handle_result_ioerr_or_panic<T>(line: Result<T, std::io::Error>) -> T {
    match line {
        Ok(s) => s,
        Err(error) => panic!("error: {error}"),
    }
}

fn is_not_white_space(line: &String) -> bool {
    !remove_comment(line.to_string()).trim().is_empty()
}

fn parse_line(line: String) -> Line {
    let line_without_comment_trim = remove_comment(line.clone())
        .trim()
        .to_string();

    let command_string = CommandString::build(&line_without_comment_trim);
    if command_string.is_err() {
        panic!("{:?}", command_string.unwrap_err());
    }
    let command_string = command_string.unwrap();

    let command = parse_command(command_string);
    if command.is_err() {
        panic!("{:?}", command.unwrap_err());
    }
    let command = command.unwrap();

    Line { command, line }
}

fn remove_comment(line: String) -> String {
    if let Some((line, _)) = line.split_once("//") {
        line.to_string()
    } else {
        line
    }
}

fn parse_command(command_string: CommandString) -> Result<Command, &'static str> {
    match command_string.command.as_str() {
        // arithmetic
        "add" => valid_arithmetic_args(Arithmetic::Add, &command_string.args),
        "sub" => valid_arithmetic_args(Arithmetic::Sub, &command_string.args),
        "neg" => valid_arithmetic_args(Arithmetic::Neg, &command_string.args),
        "eq" => valid_arithmetic_args(Arithmetic::Eq, &command_string.args),
        "gt" => valid_arithmetic_args(Arithmetic::Gt, &command_string.args),
        "lt" => valid_arithmetic_args(Arithmetic::Lt, &command_string.args),
        "and" => valid_arithmetic_args(Arithmetic::And, &command_string.args),
        "or" => valid_arithmetic_args(Arithmetic::Or, &command_string.args),
        "not" => valid_arithmetic_args(Arithmetic::Not, &command_string.args),

        // push / pop
        "push" => parse_push(&command_string.args),
        "pop" => parse_pop(&command_string.args),

        _ => Err("unknown command"),
    }
}

fn valid_arithmetic_args(arithmetic: Arithmetic, args: &ArgsString) -> Result<Command, &'static str> {
    if args.arg1.is_none() && args.arg2.is_none() {
        Ok(Command::Arithmetic(arithmetic))
    } else {
        Err("arithmetic command could not have arguments")
    }
}

fn parse_push(args: &ArgsString) -> Result<Command, &'static str> {
    let (segment, index) = parse_segment_arguments(args)?;

    Ok(Command::Push { segment, index })
}

fn parse_pop(args: &ArgsString) -> Result<Command, &'static str> {
    let (segment, index) = parse_segment_arguments(args)?;

    if let MemorySegment::Constant = segment {
        return Err("constant segment can not be a pop command");
    }

    Ok(Command::Pop { segment, index })
}

fn parse_segment_arguments(args: &ArgsString) -> Result<(MemorySegment, u16), &'static str> {
    if args.arg1.is_none() {
        return Err("missing memory segment argument");
    }
    let segment = args.arg1.clone().unwrap();
    let segment = parse_segment(&segment)?;

    if args.arg2.is_none() {
        return Err("missing segment index argument");
    }
    let index = args.arg2.clone().unwrap();
    let index = parse_segment_index(&index)?;

    Ok((segment, index))
}

fn parse_segment(segment: &str) -> Result<MemorySegment, &'static str> {
    match segment {
        "local" => Ok(MemorySegment::Local),
        "argument" => Ok(MemorySegment::Argument),
        "this" => Ok(MemorySegment::This),
        "that" => Ok(MemorySegment::That),
        "constant" => Ok(MemorySegment::Constant),
        "static" => Ok(MemorySegment::Static),
        "pointer" => Ok(MemorySegment::Pointer),
        "temp" => Ok(MemorySegment::Temp),
        _ => Err("can not parse memory segment argument"),
    }
}

fn parse_segment_index(index: &str) -> Result<u16, &'static str> {
    u16::from_str_radix(index, 10)
        .or_else(|_| Err("can not parse segment index argument"))
}
