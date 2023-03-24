use std::env;
use std::path::Path;
use vm_translator::{
    Config,
    code_writer::CodeWriter,
    parser::{self, Command},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap();
    let (asm_path, filename_without_ext) = get_asm_path(&config.vm_path);
    let mut code_writer = CodeWriter::open(&asm_path, filename_without_ext)?;

    parser::parse(&config.vm_path)?
        .for_each(|line| {
            // write comment
            if config.output_with_comment {
                code_writer.write_comment(&line.line);
            }

            match line.command {
                // arithmetic
                Command::Arithmetic(arithmetic) => code_writer.write_arithmetic(arithmetic),

                // push/pop
                Command::Push{ segment, index } => code_writer.write_push(segment, index),
                Command::Pop{ segment, index } => code_writer.write_pop(segment, index),
            }
        });

    Ok(())
}

fn get_asm_path(vm_path: &str) -> (String, String) {
    let vm_path = Path::new(vm_path);
    let filename_without_ext = vm_path.file_stem().expect("get file stem failed!").to_str().expect("convert file stem (OS str) failed!").to_string();

    let asm_path = vm_path.parent()
        .expect("get parent dir failed!")
        .clone()
        .join(format!("{}.asm", filename_without_ext))
        .to_str()
        .expect("asm path convert to str failed!")
        .to_string();

    (asm_path, filename_without_ext)
    
}
