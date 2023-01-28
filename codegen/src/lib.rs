use ast::{Program, Statement};

mod branch;
mod expression;
mod function;
mod loop_;
mod variable;

// entry
pub fn codegen(ast: Program) {
    let generator = CodeGenerator::new(ast);
    generator.codegen();
}

struct CodeGenerator {
    ast: Program,
}

impl CodeGenerator {
    fn new(ast: Program) -> Self {
        Self { ast }
    }
}

impl CodeGenerator {
    fn codegen(&self) {
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("");
        for stmt in self.ast.statements.iter() {
            self.gen_stmt(stmt);
        }
    }

    fn gen_stmts(&self, stmts: &[Statement]) {
        for stmt in stmts.iter() {
            self.gen_stmt(stmt);
        }
    }

    fn gen_stmt(&self, node: &Statement) {
        match node {
            Statement::If(if_stmt) => self.gen_if(if_stmt),
            Statement::While(while_stmt) => self.gen_while(while_stmt),
            Statement::For(for_stmt) => self.gen_for(for_stmt),
            Statement::Block(stmts) => self.gen_stmts(stmts),
            Statement::Expression(expr) => self.gen_expr(expr),
            Statement::Return(expr) => self.gen_return(expr),
            Statement::FunctionDefinition(function_def) => {
                self.gen_function_definition(function_def)
            }
            Statement::InitDeclaration(init_decl) => self.gen_init_declaration(init_decl),
        }
    }
}
