use ast::{Program, Statement};

mod branch;
mod expression;
mod function;
mod loop_;
mod variable;

// entry
pub fn compile(input: String) {
    let compiler = Compiler::new(input);
    compiler.compile();
}

struct Compiler {
    ast: Program,
}

impl Compiler {
    fn new(input: String) -> Self {
        let ast = match parse::parse(input) {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        Self { ast }
    }
}

impl Compiler {
    fn compile(&self) {
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("");
        for stmt in self.ast.statements.iter() {
            self.compile_stmt(stmt);
        }
    }

    fn compile_stmts(&self, stmts: &[Statement]) {
        for stmt in stmts.iter() {
            self.compile_stmt(stmt);
        }
    }

    fn compile_stmt(&self, node: &Statement) {
        match node {
            Statement::If(if_stmt) => self.compile_if(if_stmt),
            Statement::While(while_stmt) => self.compile_while(while_stmt),
            Statement::For(for_stmt) => self.compile_for(for_stmt),
            Statement::Block(stmts) => self.compile_stmts(stmts),
            Statement::Expression(expr) => self.compile_expr(expr),
            Statement::Return(expr) => self.compile_return(expr),
            Statement::FunctionDefinition(function_def) => {
                self.compile_function_definition(function_def)
            }
            Statement::InitDeclaration(init_decl) => self.compile_init_declaration(init_decl),
        }
    }
}
