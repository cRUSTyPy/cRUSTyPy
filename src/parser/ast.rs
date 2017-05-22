#[derive(Clone)]
pub enum Statement {
    Symbol(Box<String>),
    Int(i64),
}

pub type Module = Vec<Statement>;
