use brainfuck_compiler;
use clap::Parser;
use colored::*;
use std::io::Write;

enum CompileErrorTypes {
    InvalidLength,
    InvalidChar,
}
#[derive(Parser, Debug)]
#[command(name = "brainfuck_compiler")]
struct Cli {
    /// Input file
    input: String,

    /// Output file
    output: String,

    /// C Compiler
    #[arg(short, default_value = "gcc")]
    c: String,

    /// brainfuck tape length
    #[arg(short, default_value = "30000")]
    l: i32,
}
fn split_into_chunks(s: &str) -> Vec<String> {
    s.chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|c| c.iter().collect())
        .collect()
}
fn compile_error_print(msg: &str, err_type: CompileErrorTypes) {
    let ce = match err_type {
        CompileErrorTypes::InvalidLength => "Invalid length",
        CompileErrorTypes::InvalidChar => "Invalid character",
    }
    .red()
    .bold();
    let label = format!("{}{}{}", "CompileError[".red(), ce, "]".red());
    eprintln!("{}", format!("{} {}", label, msg));
}
fn main() {
    let cli = Cli::parse();
    let input_file = cli.input;
    let output_file = cli.output;
    let c_compiler = cli.c;
    let tape_length = cli.l;

    let mut eyefuck_code = std::fs::read_to_string(input_file).unwrap();

    if eyefuck_code.ends_with("\n") {
        eyefuck_code.pop();
    }
    if eyefuck_code.len() % 3 != 0 {
        compile_error_print("", CompileErrorTypes::InvalidLength);
        std::process::exit(1);
    }

    for (_, c) in eyefuck_code.chars().enumerate() {
        match c {
            'I' | 'l' => {}
            _ => {
                compile_error_print(c.to_string().as_str(), CompileErrorTypes::InvalidChar);
                std::process::exit(1);
            }
        }
    }

    let splited_code = split_into_chunks(&eyefuck_code);
    let mut bf_code = String::new();
    for i in splited_code.iter() {
        match i.as_str() {
            "III" => {
                bf_code.push_str(">");
            }
            "IIl" => {
                bf_code.push_str("<");
            }
            "IlI" => {
                bf_code.push_str("+");
            }
            "Ill" => {
                bf_code.push_str("-");
            }
            "lII" => {
                bf_code.push_str(".");
            }
            "lIl" => {
                bf_code.push_str(",");
            }
            "llI" => {
                bf_code.push_str("[");
            }
            "lll" => {
                bf_code.push_str("]");
            }
            _ => {
                compile_error_print(i.as_str(), CompileErrorTypes::InvalidChar);
            }
        }
    }
    let current_dir = std::env::current_dir().expect("Failed to get current path");
    let tmp_dir = current_dir.join("tmp");
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir).expect("Failed to create tmp directory");
    }
    let c_file_path = tmp_dir.join("output.c");
    let mut c_code_file = std::fs::File::create(&c_file_path).unwrap();
    c_code_file
        .write(brainfuck_compiler::compile_c(bf_code, tape_length).as_bytes())
        .unwrap();

    let output_path = tmp_dir.join("output");
    let command = std::process::Command::new(format!("{}", c_compiler))
        .arg(c_file_path)
        .arg("-o")
        .arg(&output_path)
        .status()
        .expect("Failed to execute gcc");
    if !command.success() {
        eprintln!("GCC Compilation failed: gcc exited with non-zero status code");
        std::process::exit(1);
    }

    let destination = current_dir.join(format!("{}", output_file));
    std::fs::copy(output_path, destination).expect("Failed to copy output binary");

    std::fs::remove_dir_all(tmp_dir).expect("Failed to delete tmp directory");
}
