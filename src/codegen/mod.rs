use crate::ast::AST;

pub mod asm;
pub mod c;

pub use asm::AssemblyCodeGenerator;
pub use c::CCodeGenerator;

pub trait Codegen {
    fn codegen(ast: AST, optimized: bool) -> String;
}

pub fn codegen<T: Codegen>(ast: AST, optimized: bool) -> String {
    T::codegen(ast, optimized)
}
