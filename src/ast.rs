#[derive(Debug)]
pub enum AST {
    Root(Vec<AST>),
    Right,
    Left,
    Add,
    Subtract,
    PrintChar,
    GetChar,
    Loop(Vec<AST>),
}
