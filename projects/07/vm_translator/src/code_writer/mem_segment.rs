use crate::parser::MemorySegment;
use super::{SP, LCL, ARG, THIS, THAT, TEMP_START_ADDR};

pub struct MemSegmentCodeGenerator {
    filename: String,
}

impl MemSegmentCodeGenerator {
    pub fn build(filename: String) -> Self {
        Self { filename }
    }

    pub fn generate_push_commands(
        &mut self, segment: MemorySegment,
        index: u16
    ) -> Vec<String> {
        match segment {
            MemorySegment::Local | MemorySegment::Argument | MemorySegment::This | MemorySegment::That => {
                let segment = match segment {
                    MemorySegment::Local => LCL,
                    MemorySegment::Argument => ARG,
                    MemorySegment::This => THIS,
                    MemorySegment::That => THAT,
                    _ => panic!(),
                };
                let mut commands = Vec::new();

                // addr = segment + i
                if index > 0 {
                    commands.extend(vec![
                        format!("@{index}"),
                        "D=A".to_string(),
                        format!("@{segment}"),
                        "A=D+M".to_string(),
                    ]);
                } else {
                    commands.extend(vec![
                        format!("@{segment}"),
                        "A=M".to_string(),
                    ]);
                }

                commands.extend(vec![
                    "D=M".to_string(),

                    // *SP = *addr
                    format!("@{SP}"),
                    "A=M".to_string(),
                    "M=D".to_string(),

                    // SP++
                    format!("@{SP}"),
                    "M=M+1".to_string(),
                ]);

                commands
            },
            MemorySegment::Constant => {
                vec![
                    // *SP = constant
                    format!("@{index}"),
                    "D=A".to_string(),
                    format!("@{SP}"),
                    "A=M".to_string(),
                    "M=D".to_string(),

                    // SP++
                    format!("@{SP}"),
                    "M=M+1".to_string(),
                ]
            },
            MemorySegment::Static => {
                let static_name = self.get_static_name(index);

                vec![
                    // *SP = variable
                    format!("@{static_name}"),
                    "D=M".to_string(),
                    format!("@{SP}"),
                    "A=M".to_string(),
                    "M=D".to_string(),

                    // SP++
                    format!("@{SP}"),
                    "M=M+1".to_string(),
                ]
            },
            MemorySegment::Pointer => {
                let pointer_addr = self.get_pointer_addr_name(index);

                vec![
                    // *SP = THIS/THAT
                    format!("@{pointer_addr}"),
                    "D=M".to_string(),
                    format!("@{SP}"),
                    "A=M".to_string(),
                    "M=D".to_string(),

                    // SP++
                    format!("@{SP}"),
                    "M=M+1".to_string(),
                ]
            },
            MemorySegment::Temp => {
                let addr = self.get_tmp_addr(index);

                vec![
                    // temp + i
                    format!("@{addr}"),
                    "D=M".to_string(),

                    // *SP = temp + i
                    format!("@{SP}"),
                    "A=M".to_string(),
                    "M=D".to_string(),

                    // SP++
                    format!("@{SP}"),
                    "M=M+1".to_string(),
                ]
            },
        }
    }

    pub fn generate_pop_commands(
        &mut self, segment: MemorySegment,
        index: u16
    ) -> Vec<String> {
        match segment {
            MemorySegment::Local | MemorySegment::Argument | MemorySegment::This | MemorySegment::That => {
                let segment = match segment {
                    MemorySegment::Local => LCL,
                    MemorySegment::Argument => ARG,
                    MemorySegment::This => THIS,
                    MemorySegment::That => THAT,
                    _ => panic!(),
                };

                let mut commands = Vec::new();

                // addr = segment + i
                if index > 0 {
                    commands.extend(vec![
                        format!("@{index}"),
                        "D=A".to_string(),
                        format!("@{segment}"),
                        "D=D+M".to_string(),
                    ]);
                } else {
                    commands.extend(vec![
                        format!("@{segment}"),
                        "D=M".to_string(),
                    ]);
                }

                commands.extend(vec![
                    // temp = addr
                    format!("@{}", self.get_tmp_addr(0)),
                    "M=D".to_string(),

                    // SP--
                    format!("@{SP}"),
                    "M=M-1".to_string(),

                    // *addr = *SP
                    "A=M".to_string(),
                    "D=M".to_string(),
                    format!("@{}", self.get_tmp_addr(0)),
                    "A=M".to_string(),
                    "M=D".to_string(),
                ]);

                commands
            },
            MemorySegment::Static => {
                let static_name = self.get_static_name(index);

                vec![
                    // SP--
                    format!("@{SP}"),
                    "M=M-1".to_string(),

                    // variable = *SP
                    "A=M".to_string(),
                    "D=M".to_string(),
                    format!("@{static_name}"),
                    "M=D".to_string(),
                ]
            },
            MemorySegment::Pointer => {
                let pointer_addr = self.get_pointer_addr_name(index);

                vec![
                    // SP--
                    format!("@{SP}"),
                    "M=M-1".to_string(),

                    // THIS/THAT = *SP
                    "A=M".to_string(),
                    "D=M".to_string(),
                    format!("@{pointer_addr}"),
                    "M=D".to_string(),
                ]
            },
            MemorySegment::Temp => {
                let addr = self.get_tmp_addr(index);

                vec![
                    // SP--
                    format!("@{SP}"),
                    "M=M-1".to_string(),

                    // temp + i = *SP
                    "A=M".to_string(),
                    "D=M".to_string(),
                    format!("@{addr}"),
                    "M=D".to_string(),
                ]
            },
            _ => panic!(),
        }
    }

    fn get_static_name(&self, index: u16) -> String {
        format!("{}.{}", self.filename, index)
    }

    fn get_pointer_addr_name(&self, index: u16) -> &'static str {
        match index {
            0 => THIS,
            1 => THAT,
            _ => panic!("index of pointer segment must be 0 or 1"),
        }
    }

    fn get_tmp_addr(&self, index: u16) -> u16 {
        TEMP_START_ADDR + index
    }
}
