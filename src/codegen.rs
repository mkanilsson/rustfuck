use crate::ast::AST;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

static TEMPLATE_START: &'static str = include_str!("../resources/asm/start.S");
static TEMPLATE_END: &'static str = include_str!("../resources/asm/end.S");

static TEMPLATE_RIGHT: &'static str = include_str!("../resources/asm/right.S");
static TEMPLATE_LEFT: &'static str = include_str!("../resources/asm/left.S");

static TEMPLATE_ADD: &'static str = include_str!("../resources/asm/add.S");
static TEMPLATE_SUBTRACT: &'static str = include_str!("../resources/asm/subtract.S");

static TEMPLATE_PRINT_CHAR: &'static str = include_str!("../resources/asm/putchar.S");
static TEMPLATE_GET_CHAR: &'static str = include_str!("../resources/asm/getchar.S");

static TEMPLATE_LOOP_END: &'static str = include_str!("../resources/asm/loop.S");

fn gen_label(hint: &str) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();

    format!("{hint}_{rand_string}")
}

pub fn codegen(ast: AST) -> String {
    let mut content = TEMPLATE_START.to_owned();

    match ast {
        AST::Root(statments) => {
            let generated = codegen_statments(statments);
            content = format!("{content}\n{generated}");
        }
        _ => panic!("Expected AST::Root"),
    }

    content + TEMPLATE_END
}

fn codegen_statments(statments: Vec<AST>) -> String {
    statments
        .iter()
        .map(|statment| codegen_statment(statment) + "\n")
        .collect()
}

fn codegen_statment(statment: &AST) -> String {
    match statment {
        AST::Right(times) => codegen_right(*times),
        AST::Left(times) => codegen_left(*times),
        AST::Add(times) => codegen_add(*times),
        AST::Subtract(times) => codegen_subtract(*times),
        AST::PrintChar => codegen_print_char().to_owned(),
        AST::GetChar => codegen_get_char().to_owned(),
        AST::Loop(statments) => codegen_loop(statments),
        _ => unreachable!(),
    }
}

// TODO: Loop over x times if optimization is disabled
fn codegen_right(times: usize) -> String {
    TEMPLATE_RIGHT
        .to_owned()
        .replace("{TIMES}", &times.to_string())
}

fn codegen_left(times: usize) -> String {
    TEMPLATE_LEFT
        .to_owned()
        .replace("{TIMES}", &times.to_string())
}

fn codegen_add(times: usize) -> String {
    TEMPLATE_ADD
        .to_owned()
        .replace("{TIMES}", &times.to_string())
}

fn codegen_subtract(times: usize) -> String {
    TEMPLATE_SUBTRACT
        .to_owned()
        .replace("{TIMES}", &times.to_string())
}

fn codegen_print_char() -> &'static str {
    TEMPLATE_PRINT_CHAR
}

fn codegen_get_char() -> &'static str {
    TEMPLATE_GET_CHAR
}

fn codegen_loop(statments: &Vec<AST>) -> String {
    let body_label = gen_label("loop_body");
    let condition_label = gen_label("loop_condition");

    let mut content = format!("        jmp .{condition_label}\n.{body_label}:");

    for stmt in statments {
        let generated = codegen_statment(stmt);
        content = format!("{content}\n{generated}");
    }

    let end = TEMPLATE_LOOP_END
        .to_string()
        .replace("{LABEL_COND}", &condition_label)
        .replace("{LABEL_BODY}", &body_label);

    format!("{content}\n{end}")
}
