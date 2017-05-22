/// contains all the parsing structures of ghvm
mod ast;
use self::ast::{Module, Statement};

peg_file! grammar("grammar.rustpeg");

#[cfg(test)]
mod test_grammar;

pub fn parse(body: &str) -> Result<ast::Module, String> {
    match grammar::module(body) {
        Ok(module) => Ok(module),
        Err(error) => Err(format!("{}", error))
    }
}
