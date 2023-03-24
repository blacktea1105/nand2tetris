use crate::parser::Arithmetic;
use super::{SP, IncreasedIdLabel};

pub fn code_generator(
    arithmetic: Arithmetic,
    increased_id_label: &mut IncreasedIdLabel
) -> Vec<String> {
    match arithmetic {
        Arithmetic::Add => {
            pop_two_into_mreg_then_dreg().chain([
                "D=D+M".to_string(),

                format!("@{SP}"),
                "A=M-1".to_string(),
                "M=D".to_string(),
            ].into_iter())
            .collect()
        },
        Arithmetic::Sub => {
            pop_two_into_mreg_then_dreg().chain([
                "D=M-D".to_string(),

                format!("@{SP}"),
                "A=M-1".to_string(),
                "M=D".to_string(),
            ].into_iter())
            .collect()
        },
        Arithmetic::Neg => {
            vec![
                format!("@{SP}"),
                "A=M-1".to_string(),
                "M=-M".to_string(),
            ]
        },
        Arithmetic::Eq => {
            generate_ord("JEQ", increased_id_label).collect()
        },
        Arithmetic::Gt => {
            generate_ord("JGT", increased_id_label).collect()
        },
        Arithmetic::Lt => {
            generate_ord("JLT", increased_id_label).collect()
        },
        Arithmetic::And => {
            pop_two_into_mreg_then_dreg().chain([
                "D=D&M".to_string(),

                format!("@{SP}"),
                "A=M-1".to_string(),
                "M=D".to_string(),
            ].into_iter())
            .collect()
        },
        Arithmetic::Or => {
            pop_two_into_mreg_then_dreg().chain([
                "D=D|M".to_string(),

                format!("@{SP}"),
                "A=M-1".to_string(),
                "M=D".to_string(),
            ].into_iter())
            .collect()
        },
        Arithmetic::Not => {
            vec![
                format!("@{SP}"),
                "A=M-1".to_string(),
                "M=!M".to_string(),
            ]
        },
    }
}

// SP-- only once
fn pop_two_into_mreg_then_dreg() -> impl Iterator<Item=String> {
    [
        // SP--
        format!("@{SP}"),
        "M=M-1".to_string(),

        // D=stack1
        "A=M".to_string(),
        "D=M".to_string(),

        // M=stack2
        format!("@{SP}"),
        "A=M-1".to_string(),
    ].into_iter()
}

fn generate_ord(
    ord: &str,
    increased_id_label: &mut IncreasedIdLabel
) -> impl Iterator<Item=String> {
    let [ord_label, ord_end_label] = increased_id_label.generate_increased_id_labels([
        "LOGICAL_ORD".to_string(),
        "LOGICAL_ORD_END".to_string(),
    ]);

    pop_two_into_mreg_then_dreg().chain([
        // branching
        "D=M-D".to_string(),
        format!("@{ord_label}"),
        format!("D; {ord}"),

        // false
        format!("@{SP}"),
        "A=M-1".to_string(),
        "M=0".to_string(),
        format!("@{ord_end_label}"),
        "0; JMP".to_string(),

        // true
        format!("({ord_label})"),
        format!("@{SP}"),
        "A=M-1".to_string(),
        "M=-1".to_string(),

        // end branching
        format!("({ord_end_label})"),
    ].into_iter())
}
