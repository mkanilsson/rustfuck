use crate::ast::AST;

use std::sync::atomic::{AtomicUsize, Ordering};

static TEMPLATE_START: &'static str = include_str!("../resources/asm/start.S");
static TEMPLATE_END: &'static str = include_str!("../resources/asm/end.S");

static TEMPLATE_RIGHT: &'static str = include_str!("../resources/asm/right.S");
static TEMPLATE_LEFT: &'static str = include_str!("../resources/asm/left.S");
static TEMPLATE_ADD: &'static str = include_str!("../resources/asm/add.S");
static TEMPLATE_SUBTRACT: &'static str = include_str!("../resources/asm/subtract.S");

static TEMPLATE_PRINT_CHAR: &'static str = include_str!("../resources/asm/putchar.S");
static TEMPLATE_GET_CHAR: &'static str = include_str!("../resources/asm/getchar.S");

static TEMPLATE_LOOP_END: &'static str = include_str!("../resources/asm/loop.S");

static GLOBAL_LABEL_COUNT: AtomicUsize = AtomicUsize::new(0);

fn gen_label(hint: &str) -> String {
    let counter = GLOBAL_LABEL_COUNT.fetch_add(1, Ordering::SeqCst);
    format!("{hint}_{counter}")
}

pub fn codegen(ast: AST, optimized: bool) -> String {
    let mut content = TEMPLATE_START.to_owned();

    match ast {
        AST::Root(statments) => {
            let generated = codegen_statments(statments, optimized);
            content = format!("{content}\n{generated}");
        }
        _ => panic!("Expected AST::Root"),
    }

    content + TEMPLATE_END
}

fn codegen_statments(statments: Vec<AST>, optimized: bool) -> String {
    statments
        .iter()
        .map(|statment| codegen_statment(statment, optimized) + "\n")
        .collect()
}

fn codegen_statment(statment: &AST, optimized: bool) -> String {
    match statment {
        AST::Right(times) => codegen_right(*times, optimized),
        AST::Left(times) => codegen_left(*times, optimized),
        AST::Add(times) => codegen_add(*times, optimized),
        AST::Subtract(times) => codegen_subtract(*times, optimized),
        AST::PrintChar => codegen_print_char().to_owned(),
        AST::GetChar => codegen_get_char().to_owned(),
        AST::Loop(statments) => codegen_loop(statments, optimized),
        _ => unreachable!(),
    }
}

fn codegen_numeric(asm: &str, times: usize, optimized: bool) -> String {
    if optimized {
        asm.to_owned().replace("{TIMES}", &times.to_string())
    } else {
        let mut content = "".to_string();
        let one = asm.to_owned().replace("{TIMES}", "1");

        for _ in 0..times {
            content = format!("{content}\n{one}");
        }

        content
    }
}

// TODO: Loop over x times if optimization is disabled
fn codegen_right(times: usize, optimized: bool) -> String {
    codegen_numeric(TEMPLATE_RIGHT, times, optimized)
}

fn codegen_left(times: usize, optimized: bool) -> String {
    codegen_numeric(TEMPLATE_LEFT, times, optimized)
}

fn codegen_add(times: usize, optimized: bool) -> String {
    codegen_numeric(TEMPLATE_ADD, times, optimized)
}

fn codegen_subtract(times: usize, optimized: bool) -> String {
    codegen_numeric(TEMPLATE_SUBTRACT, times, optimized)
}

fn codegen_print_char() -> &'static str {
    TEMPLATE_PRINT_CHAR
}

fn codegen_get_char() -> &'static str {
    TEMPLATE_GET_CHAR
}

fn codegen_loop(statments: &Vec<AST>, optimized: bool) -> String {
    let body_label = gen_label("loop_body");
    let condition_label = gen_label("loop_condition");

    let mut content = format!("        jmp .{condition_label}\n.{body_label}:");

    for stmt in statments {
        let generated = codegen_statment(stmt, optimized);
        content = format!("{content}\n{generated}");
    }

    let end = TEMPLATE_LOOP_END
        .to_string()
        .replace("{LABEL_COND}", &condition_label)
        .replace("{LABEL_BODY}", &body_label);

    format!("{content}\n{end}")
}
