pub(crate) mod tokenize;
pub(crate) mod ast;
pub(crate) mod module;
pub(crate) mod assembly;
pub use assembly::Assembly;


pub fn parse(schema: &str) -> Assembly {
    let tokens = tokenize::tokenize(schema);
    let ast = ast::gen_ast(&tokens);
    let module = module::create_module(&ast);
    let assembly = assembly::generate(&module);
    return assembly;
}



