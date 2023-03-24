pub mod code_writer;
pub mod parser;

#[derive(Debug)]
pub struct Config {
    pub vm_path: String,
    pub output_with_comment: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Error: vm file path (arg 1) should be specificed!");
        }

        let vm_path = args[1].clone();
        let output_with_comment = if args.len() >= 3 {
            args[2] == "1"
        } else {
            false
        };

        Ok(Self { vm_path, output_with_comment })
    }
}
