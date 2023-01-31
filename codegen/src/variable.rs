use ast::{Expression, Type, TypeEnum};

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
            }
            Expression::String(string) => {
                println!("  # -- init string start");
                match type_ {
                    Type::Array { type_, .. } => match type_.as_ref() {
                        Type::Primitive(t) => match t {
                            TypeEnum::Char => {
                                // TODO:
                                // let asciis = string
                                //     .chars()
                                //     .map(|c| format!("0{:b}", c as u8))
                                //     .collect::<Vec<String>>();
                                // let reversed = asciis.into_iter().rev().collect::<Vec<String>>();
                                // let i = isize::from_str_radix(&reversed.join(""), 2).unwrap();
                                // self.gen_init_lval(*offset + 1);
                                // println!("  pop rax");
                                // println!("  mov [rax], {i}");
                            }
                            _ => panic!("Invalid type: {:?}.", type_),
                        },
                        _ => panic!("Invalid type: {:?}.", type_),
                    },
                    Type::Pointer(t) => match t.as_ref() {
                        Type::Primitive(t) => match t {
                            TypeEnum::Char => {
                                let label = format!(".LC{}", self.str_lits.len());
                                self.str_lits.push(AsmStringLiteral {
                                    label: label.clone(),
                                    value: string.clone(),
                                });
                                self.gen_init_lval(*offset);
                                println!("  pop rax");
                                println!("  mov qword ptr [rax], offset flat:{label}");
                            }
                            _ => panic!("Invalid type: {:?}.", type_),
                        },
                        _ => panic!("Invalid type: {:?}.", type_),
                    },
                    _ => panic!("Invalid type: {:?}.", type_),
                }
            }
            _ => {
                self.gen_expr(expr);
                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi");
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
