mod ast;
mod codegen;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;

use std::{env, fs, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Expeced 1 arguments");
    }

    let file_path = &args[1];
    let lexer = Lexer::new_from_path(file_path);

    // let tokens: Vec<_> = lexer.into_iter().collect();
    // println!("{:#?}", tokens);

    let ast = Parser::parse(lexer);
    // println!("{:#?}", ast);

    let asm = codegen::codegen(ast);
    let filename = "a";
    save(filename, &asm);
    compile(filename);
}

fn save(filename: &str, data: &str) {
    fs::write(format!("output/{filename}.S"), data).expect("Failed to write to file");
}

fn compile(filename: &str) {
    let source = format!("output/{filename}.S");
    let object = format!("output/{filename}.o");
    let executable = format!("output/{filename}");

    println!("Running `as`...");
    let output = Command::new("as")
        .args(&[source, "-o".into(), object.clone()])
        .output()
        .expect("Failed to run `as`. Make sure it's installed.");

    if !output.status.success() {
        println!("`as` failed...");
        String::from_utf8(output.stderr)
            .unwrap()
            .lines()
            .for_each(|line| eprintln!("{line}"));

        return;
    }
    println!("`as` succeed...");

    println!("Running `gcc`...");
    let output = Command::new("gcc")
        .args(&[object, "-o".into(), executable])
        .output()
        .expect("Failed to run `gcc`. Make sure it's installed.");

    if !output.status.success() {
        println!("`gcc` failed...");
        String::from_utf8(output.stderr)
            .unwrap()
            .lines()
            .for_each(|line| eprintln!("{line}"));

        return;
    }
    println!("`gcc` succeed...");
}
