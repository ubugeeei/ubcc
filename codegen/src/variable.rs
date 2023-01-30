use ast::{Expression, Type};

use crate::{AsmStringLiteral, CodeGenerator};

impl CodeGenerator {
    pub(super) fn gen_init_declaration(
        &mut self,
        name: &String,
        offset: &usize,
        type_: &Type,
        init: &Option<Expression>,
    ) {
        println!("  # -- init declaration {}", name);
        self.gen_init_lval(*offset);
        match init {
            Some(ref init) => self.gen_init_expr(init, offset, type_),
            None => {}
        }
    }

    pub(super) fn gen_lval(&self, node: &Expression) {
        match node {
            Expression::LocalVariable { offset, type_, .. } => {
                match type_ {
                    // cast to pointer
                    Type::Array { type_, size, .. } => {
                        println!("  mov rax, rbp");
                        println!(
                            "  sub rax, {}",
                            (*offset) - (*size as usize - 1) * (type_.size())
                        );
                        println!("  push rax");
                    }
                    _ => {
                        println!("  mov rax, rbp");
                        println!("  sub rax, {offset}");
                        println!("  push rax");
                    }
                }
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

    pub(super) fn gen_init_expr(&mut self, expr: &Expression, offset: &usize, type_: &Type) {
        match expr {
            Expression::Array { elements, .. } => {
                println!("  # -- init array start");
                let element_type = match type_ {
                    Type::Array { type_, .. } => type_,
                    _ => panic!("Invalid type: {:?}.", type_),
                };

                for (i, element) in elements.iter().enumerate() {
                    self.gen_init_lval(
                        ((*offset) - (type_.size() as usize)) + (i + 1) * element_type.size(),
                    );
                    self.gen_expr(element);
                    println!("  pop rdi");
                    println!("  pop rax");
                    println!("  mov [rax], rdi");
                    println!("  push rdi");
                }
                println!("  # -- init array end");
                println!("");
            }
            Expression::String { label, value, .. } => {
                self.str_lits.push(AsmStringLiteral {
                    label: label.to_owned(),
                    value: value.to_owned(),
                });

                let len = match type_ {
                    Type::Array { size, .. } => *size as usize,
                    _ => panic!("Invalid type: {:?}.", type_),
                };

                // TODO:
                println!("  mov rax, rbp");
                println!("  sub rax, {}", offset - (len - 1));
                println!("  push rax");

                println!("  pop rax");
                println!("  mov r8d, {label}");
                println!("  mov [rax], r8d");
                println!("  push rdi");
                println!("");
            }
            _ => {
                self.gen_expr(expr);
                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi");
                println!("");
            }
        }
    }

    pub(super) fn gen_str_lits(&self) {
        for lit in self.str_lits.iter() {
            println!("{}: .string \"{}\"", lit.label, lit.value);
            println!("");
        }
    }
}
