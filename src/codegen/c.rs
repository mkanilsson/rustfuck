use super::Codegen;
use crate::ast::AST;

static TEMPLATE_START: &'static str = include_str!("../../resources/c/start.c");
static TEMPLATE_END: &'static str = include_str!("../../resources/c/end.c");

static TEMPLATE_RIGHT: &'static str = include_str!("../../resources/c/right.c");
static TEMPLATE_LEFT: &'static str = include_str!("../../resources/c/left.c");
static TEMPLATE_ADD: &'static str = include_str!("../../resources/c/add.c");
static TEMPLATE_SUBTRACT: &'static str = include_str!("../../resources/c/subtract.c");

static TEMPLATE_PRINT_CHAR: &'static str = include_str!("../../resources/c/putchar.c");
static TEMPLATE_GET_CHAR: &'static str = include_str!("../../resources/c/getchar.c");

static TEMPLATE_LOOP: &'static str = include_str!("../../resources/c/loop.c");

pub struct CCodeGenerator {}

impl CCodeGenerator {
    fn codegen_statments(statments: Vec<AST>, optimized: bool) -> String {
        statments
            .iter()
            .map(|statment| Self::codegen_statment(statment, optimized) + "\n")
            .collect()
    }

    fn codegen_statment(statment: &AST, optimized: bool) -> String {
        match statment {
            AST::Right(times) => Self::codegen_right(*times, optimized),
            AST::Left(times) => Self::codegen_left(*times, optimized),
            AST::Add(times) => Self::codegen_add(*times, optimized),
            AST::Subtract(times) => Self::codegen_subtract(*times, optimized),
            AST::PrintChar => Self::codegen_print_char().to_owned(),
            AST::GetChar => Self::codegen_get_char().to_owned(),
            AST::Loop(statments) => Self::codegen_loop(statments, optimized),
            _ => unreachable!(),
        }
    }

    fn codegen_numeric(c: &str, times: usize, optimized: bool) -> String {
        if optimized {
            c.to_owned().replace("{TIMES}", &times.to_string())
        } else {
            let mut content = "".to_string();
            let one = c.to_owned().replace("{TIMES}", "1");

            for _ in 0..times {
                content = format!("{content}\n{one}");
            }

            content
        }
    }

    // TODO: Loop over x times if optimization is disabled
    fn codegen_right(times: usize, optimized: bool) -> String {
        Self::codegen_numeric(TEMPLATE_RIGHT, times, optimized)
    }

    fn codegen_left(times: usize, optimized: bool) -> String {
        Self::codegen_numeric(TEMPLATE_LEFT, times, optimized)
    }

    fn codegen_add(times: usize, optimized: bool) -> String {
        Self::codegen_numeric(TEMPLATE_ADD, times, optimized)
    }

    fn codegen_subtract(times: usize, optimized: bool) -> String {
        Self::codegen_numeric(TEMPLATE_SUBTRACT, times, optimized)
    }

    fn codegen_print_char() -> &'static str {
        TEMPLATE_PRINT_CHAR
    }

    fn codegen_get_char() -> &'static str {
        TEMPLATE_GET_CHAR
    }

    fn codegen_loop(statments: &Vec<AST>, optimized: bool) -> String {
        let mut body = "".to_string();

        for stmt in statments {
            let generated = Self::codegen_statment(stmt, optimized);
            body = format!("{body}\n{generated}");
        }

        TEMPLATE_LOOP.to_string().replace("{ BODY }", &body)
    }
}

impl Codegen for CCodeGenerator {
    fn codegen(ast: AST, optimized: bool) -> String {
        let mut content = TEMPLATE_START.to_owned();

        match ast {
            AST::Root(statments) => {
                let generated = Self::codegen_statments(statments, optimized);
                content = format!("{content}\n{generated}");
            }
            _ => panic!("Expected AST::Root"),
        }

        content + TEMPLATE_END
    }
}
