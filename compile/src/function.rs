use ast::{FunctionDefinition, Expression};

use crate::Compiler;

impl Compiler {
    pub(super) fn compile_function_definition(&self, function_def: &FunctionDefinition) {
        println!("# ====== function definition ======");
        println!("{}:", function_def.name);
        println!("  # prologue");
        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("");
        println!("  # arguments");
        let registers = ["rdi", "rsi", "rdx", "rcx", "r8d", "r9d"];
        for (i, arg) in function_def.arguments.iter().enumerate() {
            let offset = match arg {
                Expression::LocalVariable { offset, .. } => offset,
                _ => panic!("invalid argument"),
            };
            println!("  mov [rbp-{}], {}", offset, registers[i]);
        }
        if function_def.arguments.len() == 0 {
            println!("    # --");
        }
        println!("");

        println!("  # body");
        self.compile_stmts(&function_def.body);
        println!("");
    }

    pub(super) fn compile_return(&self, node: &Expression) {
        println!("  # -- return");
        self.compile_expr(node);
        println!("  # epilogue");
        println!("  pop rax");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
        println!("");
    }
}
