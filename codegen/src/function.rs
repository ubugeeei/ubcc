use ast::{Expression, Statement};

use crate::CodeGenerator;

impl CodeGenerator {
    pub(super) fn gen_function_definition(
        &mut self,
        name: &String,
        arguments: &Vec<Expression>, // Expression::LocalVariable
        body: &Vec<Statement>,
    ) {
        println!("# ====== function definition ======");
        println!("{}:", name);
        println!("  # prologue");
        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("");
        println!("  # arguments");
        let registers = ["rdi", "rsi", "rdx", "rcx", "r8d", "r9d"];
        for (i, arg) in arguments.iter().enumerate() {
            let offset = match arg {
                Expression::LocalVariable { offset, .. } => offset,
                _ => panic!("invalid argument"),
            };
            println!("  mov [rbp-{}], {}", offset, registers[i]);
        }
        if arguments.len() == 0 {
            println!("    # --");
        }
        println!("");

        println!("  # body");
        self.gen_stmts(body);
        println!("");
    }

    pub(super) fn gen_return(&mut self, node: &Expression) {
        println!("  # -- return");
        self.gen_expr(node);
        println!("  # epilogue");
        println!("  pop rax");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
        println!("");
    }
}
