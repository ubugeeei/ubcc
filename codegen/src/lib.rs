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
            Statement::If {
                condition,
                consequence,
                alternative,
            } => self.gen_if(condition, consequence, alternative),
            Statement::While { condition, body } => self.gen_while(condition, body),
            Statement::For {
                init,
                condition,
                post,
                body,
            } => self.gen_for(init, condition, post, body),
            Statement::Block(stmts) => self.gen_stmts(stmts),
            Statement::Expression(expr) => self.gen_expr(expr),
            Statement::Return(expr) => self.gen_return(expr),
            Statement::FunctionDefinition {
                name,
                arguments,
                body,
            } => self.gen_function_definition(name, arguments, body),
            Statement::InitDeclaration(init_decl) => self.gen_init_declaration(init_decl),
        }
    }
}
