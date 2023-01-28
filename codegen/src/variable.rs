use ast::{Expression, InitDeclaration};

use crate::CodeGenerator;

impl CodeGenerator {
    pub(super) fn gen_init_declaration(&self, init_decl: &InitDeclaration) {
        println!("  # -- init declaration {}", init_decl.name);
        self.gen_init_lval(init_decl.offset);
        match init_decl.init {
            Some(ref init) => self.gen_expr(init),
            None => {
                println!("  push rax");
            }
        }
        println!("  pop rdi");
        println!("  pop rax");
        println!("  mov [rax], rdi");
        println!("  push rdi");
        println!("");
    }
    pub(super) fn gen_lval(&self, node: &Expression) {
        match node {
            Expression::LocalVariable { offset, .. } => {
                println!("  mov rax, rbp");
                println!("  sub rax, {offset}");
                println!("  push rax");
                println!("");
            }
            _ => {
                panic!(
                    "Invalid node: {:?}.\nleft node is not var on assignment expression.",
                    node
                );
            }
        }
    }

    pub(super) fn gen_init_lval(&self, offset: usize) {
        println!("  mov rax, rbp");
        println!("  sub rax, {offset}");
        println!("  push rax");
        println!("");
    }
}
