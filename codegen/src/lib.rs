use ast::{Program, Statement};

mod branch;
mod expression;
mod function;
mod loop_;
mod variable;

// entry
pub fn codegen(ast: Program) {
    let mut generator = CodeGenerator::new(ast);
    generator.codegen();
}

struct AsmStringLiteral {
    label: String,
    value: String,
}

struct CodeGenerator {
    ast: Program,
    str_lits: Vec<AsmStringLiteral>,
}

impl CodeGenerator {
    fn new(ast: Program) -> Self {
        Self {
            ast,
            str_lits: vec![],
        }
    }
}

impl CodeGenerator {
    fn codegen(&mut self) {
        println!("  .intel_syntax noprefix");
        println!("  .global main");
        println!("");
        println!("  .text");
        for stmt in self.ast.statements.clone().iter() {
            self.gen_stmt(stmt);
        }

        println!("  .data");
        self.gen_str_lits();
    }

    fn gen_stmts(&mut self, stmts: &[Statement]) {
        for stmt in stmts.iter() {
            self.gen_stmt(stmt);
        }
    }

    fn gen_stmt(&mut self, node: &Statement) {
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
            Statement::InitDeclaration {
                name,
                offset,
                type_,
                init,
            } => self.gen_init_declaration(name, offset, type_, init),
        }
    }
}
