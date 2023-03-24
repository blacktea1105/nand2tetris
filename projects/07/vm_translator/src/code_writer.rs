use std::fs::File;
use std::io::{LineWriter, Write};
use crate::parser::{Arithmetic, MemorySegment};
use self::mem_segment::MemSegmentCodeGenerator;

pub mod arithmetic;
pub mod mem_segment;

const SP: &'static str = "SP";
const LCL: &'static str = "LCL";
const ARG: &'static str = "ARG";
const THIS: &'static str = "THIS";
const THAT: &'static str = "THAT";
const TEMP_START_ADDR: u16 = 5;

pub struct IncreasedIdLabel {
    filename: String,
    label_id: u16,
}

impl IncreasedIdLabel {
    fn build(filename: String) -> Self {
        Self { filename, label_id: 0 }
    }

    fn generate_increased_id_labels<const N: usize>(&mut self, mut labels: [String; N]) -> [String; N] {
        let label_id = self.label_id;
        self.label_id += 1;

        for label in labels.iter_mut() {
            *label = format!("{}_{}_{}", self.filename.clone(), label, label_id);
        }

        labels
    }
}

pub struct CodeWriter {
    line_writer: LineWriter<File>,
    increased_id_label: IncreasedIdLabel,
    mem_segment_code_generator: MemSegmentCodeGenerator,
}

impl CodeWriter {
    pub fn open(path: &str, filename_without_ext: String) -> std::io::Result<Self> {
        let file = File::create(path)?;
        let line_writer = LineWriter::new(file);
        let increased_id_label = IncreasedIdLabel::build(filename_without_ext.clone());
        let mem_segment_code_generator = MemSegmentCodeGenerator::build(filename_without_ext.clone());

        Ok(Self { line_writer, increased_id_label, mem_segment_code_generator })
    }

    pub fn write_arithmetic(&mut self, arithmetic: Arithmetic) {
        for command in self::arithmetic::code_generator(arithmetic, &mut self.increased_id_label) {
            self.write_line(command);
        }
    }

    pub fn write_push(&mut self, segment: MemorySegment, index: u16) {
        for command in self.mem_segment_code_generator.generate_push_commands(segment, index) {
            self.write_line(command);
        }
    }

    pub fn write_pop(&mut self, segment: MemorySegment, index: u16) {
        for command in self.mem_segment_code_generator.generate_pop_commands(segment, index) {
            self.write_line(command);
        }
    }

    pub fn write_comment(&mut self, comment: &str) {
        self.write_line(format!("// {comment}"));
    }

    fn write_line(&mut self, line: String) {
        self.line_writer.write_all(format!("{line}\n").as_bytes())
            .expect("write file error!");
    }
}
