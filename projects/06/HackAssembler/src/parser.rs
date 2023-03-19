use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Line {
    WhiteSpace,
    ACommand { symbol: String },
    CCommand { dest: Option<String>, comp: String, jump: Option<String> },
    Label { name: String },
}

pub fn parse(filename: &str) -> Result<impl Iterator<Item=Line>, std::io::Error> {
    let f = File::open(filename)?;
    let f = BufReader::new(f);

    let iterator = f.lines()
        .map(handle_line_or_panic)
        .map(parse_line);
    Ok(iterator)
}

fn handle_line_or_panic(line: Result<String, std::io::Error>) -> String {
    match line {
        Ok(s) => s,
        Err(error) => panic!("error: {error}"),
    }
}

fn parse_line(line: String) -> Line {
    // white space
    if let Some(white_space) = try_convert_white_space(&line) {
        return white_space;
    }

    // remove inline comment
    let line = if let Some((line, _)) = line.split_once("//") {
        line.trim().to_string()
    } else {
        line
    };

    // A Command
    if let Some(a_command) = try_convert_a_command(&line) {
        return a_command;
    }

    // Label
    if let Some(label) = try_convert_label(&line) {
        return label;
    }

    // C Command
    if let Some(c_command) = try_convert_c_command(&line) {
        return c_command;
    }

    panic!("no match line!");
}

pub fn try_convert_white_space(text: &str) -> Option<Line> {
    lazy_static! {
        static ref WHITE_SPACE_RE: Regex = Regex::new(r"^(\s)*$").unwrap();
        static ref LINE_COMMENT_RE: Regex = Regex::new(r"^(\s)*//").unwrap();
    }

    if WHITE_SPACE_RE.is_match(text) || LINE_COMMENT_RE.is_match(text) {
        Some(Line::WhiteSpace)
    } else {
        None
    }
}

pub fn try_convert_a_command(text: &str) -> Option<Line> {
    lazy_static! {
        static ref A_COMMAND_RE: Regex = Regex::new(r"^\s*@(\S*)\s*$").unwrap();
    }

    if let Some(cap) = A_COMMAND_RE.captures(text) {
        let symbol = cap[1].to_string();
        Some(Line::ACommand { symbol })
    } else {
        None
    }
}

pub fn try_convert_label(text: &str) -> Option<Line> {
    lazy_static! {
        static ref LABEL_RE: Regex = Regex::new(r"^\s*\((\S*)\)\s*$").unwrap();
    }

    if let Some(cap) = LABEL_RE.captures(text) {
        let name = cap[1].to_string();
        Some(Line::Label { name })
    } else {
        None
    }
}

pub fn try_convert_c_command(text: &str) -> Option<Line> {
    let text = text.split_whitespace().collect::<String>();
    if text.is_empty() {
        return None;
    }

    // split by ';'
    let (dest_comp, jump): (String, Option<String>) = match text.split_once(";") {
        None => (text, None),
        Some((dest_comp, jump)) => (dest_comp.trim().to_string(), Some(jump.trim().to_string())),
    };

    // split by '='
    let (dest, comp): (Option<String>, String) = match dest_comp.split_once("=") {
        None => (None, dest_comp.trim().to_string()),
        Some((dest, comp)) => (Some(dest.trim().to_string()), comp.trim().to_string()),
    };

    Some(Line::CCommand {  dest, comp, jump })
}
