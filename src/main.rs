mod ast;
mod cli;
mod codegen;
mod lexer;
mod parser;
mod token;

use ast::AST;
use cli::Cli;
use lexer::Lexer;
use parser::Parser;

use codegen::{codegen, AssemblyCodeGenerator, CCodeGenerator};

use clap::Parser as CliParser;
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

#[derive(Debug)]
struct CompilationPaths {
    pub source_path: PathBuf,
    pub asm_path: PathBuf,
    pub c_path: PathBuf,
    pub object_path: PathBuf,
    pub output_path: PathBuf,
}

impl CompilationPaths {
    pub fn new(input_file: &str, output_file: &str, use_tmp: bool) -> Self {
        let source_path = Path::new(input_file).to_owned();
        let output_path = Path::new(output_file).to_owned();

        let output_path_no_extension = if use_tmp {
            let mut tmp_dir = std::env::temp_dir();

            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Why is your computers clock set to before 1970? This is on you!");

            let file_name = format!(
                "{}_{}",
                output_path.file_stem().unwrap().to_str().unwrap(),
                timestamp.as_secs()
            );

            tmp_dir.push(file_name);
            tmp_dir
        } else {
            let mut path = output_path.parent().unwrap().to_owned();
            path.push(output_path.file_name().unwrap());
            path
        };

        let mut asm_path = output_path_no_extension.clone();
        asm_path.set_extension("S");

        let mut c_path = output_path_no_extension.clone();
        c_path.set_extension("c");

        let mut object_path = output_path_no_extension.clone();
        object_path.set_extension("o");

        Self {
            source_path,
            asm_path,
            c_path,
            object_path,
            output_path,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.assembly && cli.c {
        panic!("Can't output both c and asm at the same time!");
    }

    let output_path = if let Some(path) = &cli.output_path {
        path.clone()
    } else {
        match (cli.assembly, cli.c) {
            (false, false) => "a.out",
            (true, false) => "a.S",
            (false, true) => "a.c",
            (true, true) => unreachable!(),
        }
        .to_string()
    };

    let compilation_paths = CompilationPaths::new(&cli.input_path, &output_path, !cli.keep_files);

    let source = fs::read_to_string(&compilation_paths.source_path).expect(&format!(
        "Failed to read source {}",
        compilation_paths.source_path.to_str().unwrap()
    ));

    let lexer = Lexer::new(source);
    let ast = Parser::parse(lexer);

    if cli.dump_ast {
        println!("{:#?}", ast);
        return;
    }

    if cli.assembly {
        handle_asm(ast, &compilation_paths, &cli);
        return;
    }

    handle_c(ast, &compilation_paths, &cli);
}

fn save(output_path: &Path, data: &str) {
    fs::write(output_path, data).expect("Failed to write asm file");
}

// ASM
//
fn handle_c(ast: AST, compilation_paths: &CompilationPaths, cli: &Cli) {
    let c = codegen::<CCodeGenerator>(ast, cli.optimizations);

    if stop_at_c(&compilation_paths.output_path, &cli) {
        save(&compilation_paths.output_path, &c); // Respect specified output path
        return;
    } else {
        save(&compilation_paths.c_path, &c);
    }

    compile_c(&compilation_paths, cli);

    // NOTE: Should the files be removed or should they stay in tmp?
    if !cli.keep_files {
        fs::remove_file(&compilation_paths.c_path).expect(&format!(
            "Failed to remove c file {}",
            compilation_paths.asm_path.to_str().unwrap()
        ));
    }
}

fn stop_at_c(output_path: &Path, cli: &Cli) -> bool {
    if cli.c {
        true
    } else {
        // if output file ends with .c or .C it will count as c
        if let Some(extension) = output_path.extension() {
            extension.to_ascii_lowercase() == OsStr::new("c")
        } else {
            false
        }
    }
}

fn compile_c(paths: &CompilationPaths, cli: &Cli) {
    let c_path = paths.c_path.to_str().unwrap();
    let executable_path = paths.output_path.to_str().unwrap();

    let mut args = vec![c_path, "-o", executable_path];
    if cli.optimizations {
        args.push("-O2");
    }

    print!("Running `gcc`... ");
    let output = Command::new("gcc")
        .args(&args)
        .output()
        .expect("Failed to run `gcc`. Make sure it's installed.");

    if !output.status.success() {
        println!("FAILED");
        String::from_utf8(output.stderr)
            .unwrap()
            .lines()
            .for_each(|line| eprintln!("{line}"));

        return;
    }
    println!("SUCCESS");
}

// ASM

fn handle_asm(ast: AST, compilation_paths: &CompilationPaths, cli: &Cli) {
    let asm = codegen::<AssemblyCodeGenerator>(ast, cli.optimizations);

    if stop_at_asm(&compilation_paths.output_path, &cli) {
        save(&compilation_paths.output_path, &asm); // Respect specified output path
        return;
    } else {
        save(&compilation_paths.asm_path, &asm);
    }

    compile_asm(&compilation_paths);

    // NOTE: Should the files be removed or should they stay in tmp?
    if !cli.keep_files {
        fs::remove_file(&compilation_paths.asm_path).expect(&format!(
            "Failed to remove asm file {}",
            compilation_paths.asm_path.to_str().unwrap()
        ));

        fs::remove_file(&compilation_paths.object_path).expect(&format!(
            "Failed to remove object file {}",
            compilation_paths.object_path.to_str().unwrap()
        ));
    }
}

fn stop_at_asm(output_path: &Path, cli: &Cli) -> bool {
    if cli.assembly {
        true
    } else {
        // if output file ends with .S or .s it will count as asm
        if let Some(extension) = output_path.extension() {
            extension.to_ascii_lowercase() == OsStr::new("s")
        } else {
            false
        }
    }
}

fn compile_asm(paths: &CompilationPaths) {
    let asm_path = paths.asm_path.to_str().unwrap();
    let obj_path = paths.object_path.to_str().unwrap();
    let executable_path = paths.output_path.to_str().unwrap();

    print!("Running `as`... ");
    let output = Command::new("as")
        .args(&[asm_path, "-o", obj_path])
        .output()
        .expect("Failed to run `as`. Make sure it's installed.");

    if !output.status.success() {
        println!("FAILED");
        String::from_utf8(output.stderr)
            .unwrap()
            .lines()
            .for_each(|line| eprintln!("{line}"));

        return;
    }
    println!("SUCCESS");

    print!("Running `gcc`... ");
    let output = Command::new("gcc")
        .args(&[obj_path, "-o", executable_path])
        .output()
        .expect("Failed to run `gcc`. Make sure it's installed.");

    if !output.status.success() {
        println!("FAILED");
        String::from_utf8(output.stderr)
            .unwrap()
            .lines()
            .for_each(|line| eprintln!("{line}"));

        return;
    }
    println!("SUCCESS");
}
