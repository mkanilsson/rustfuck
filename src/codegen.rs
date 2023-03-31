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
        AST::Right => codegen_right().to_owned(),
        AST::Left => codegen_left().to_owned(),
        AST::Add => codegen_add().to_owned(),
        AST::Subtract => codegen_subtract().to_owned(),
        AST::PrintChar => codegen_print_char().to_owned(),
        AST::GetChar => codegen_get_char().to_owned(),
        AST::Loop(statments) => codegen_loop(statments),
        _ => unreachable!(),
    }
}

fn codegen_right() -> &'static str {
    TEMPLATE_RIGHT
}

fn codegen_left() -> &'static str {
    TEMPLATE_LEFT
}

fn codegen_add() -> &'static str {
    TEMPLATE_ADD
}

fn codegen_subtract() -> &'static str {
    TEMPLATE_SUBTRACT
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
