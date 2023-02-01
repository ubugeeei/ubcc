use ast::{BinaryOperator, Expression, Type, TypeEnum, UnaryOperator};

use crate::CodeGenerator;

impl CodeGenerator {
    pub(super) fn gen_expr(&mut self, node: &Expression) {
        match node {
            Expression::Integer(int) => {
                println!("  push {}", int);
            }
            Expression::Unary { expr, op } => match op {
                UnaryOperator::Minus => {
                    self.gen_expr(expr);
                    println!("  pop rax");
                    println!("  neg rax");
                }
                UnaryOperator::Reference => {
                    self.gen_lval(expr);
                }
                UnaryOperator::Dereference => {
                    self.gen_expr(expr);
                    println!("  pop rax");
                    match expr.as_ref() {
                        Expression::LocalVariable { type_, .. } => {
                            match type_ {
                                Type::Pointer(ptr) => {
                                    match ptr.as_ref() {
                                        Type::Primitive(pt) => {
                                            match pt {
                                                // TODO:
                                                TypeEnum::Int => println!("  mov rax, [rax]"),
                                                TypeEnum::Char => {
                                                    println!("  movzx rax, byte ptr [rax]")
                                                }
                                                // TODO:
                                                _ => println!("  mov rax, [rax]"),
                                            }
                                        }
                                        // TODO:
                                        _ => println!("  mov rax, [rax]"),
                                    }
                                }
                                Type::Array { .. } => println!("  mov rax, [rax]"),
                                _ => panic!("invalid dereference"),
                            }
                        }
                        _ => panic!("invalid dereference"),
                    }
                    println!("  push rax");
                }
            },
            Expression::LocalVariable { type_, .. } => match type_ {
                // cast to pointer
                Type::Array { .. } => self.gen_lval(node),
                _ => {
                    self.gen_lval(node);
                    println!("  pop rax");
                    println!("  mov rax, [rax]");
                    println!("  push rax");
                }
            },
            Expression::Call {
                callee_name,
                arguments,
            } => {
                if callee_name == "sizeof" {
                    match &arguments[0] {
                        Expression::LocalVariable { type_, .. } => {
                            println!("  push {}", type_.size());
                        }
                        Expression::Integer(_) | Expression::Binary { .. } => {
                            println!("  push 8");
                        }
                        Expression::Unary { expr, op } => match op {
                            UnaryOperator::Reference => {
                                println!("  push 8");
                            }
                            UnaryOperator::Dereference => match expr.as_ref() {
                                Expression::LocalVariable { type_, .. } => {
                                    println!("  push {}", type_.size());
                                }
                                _ => panic!("invalid sizeof"),
                            },
                            // TODO: judge type
                            _ => panic!("invalid sizeof"),
                        },
                        _ => panic!("invalid sizeof"),
                    }
                    return;
                }

                let registers = ["rdi", "rsi", "rdx", "rcx", "r8d", "r9d"];
                if arguments.len() > registers.len() {
                    panic!("too many arguments");
                }
                for (i, arg) in arguments.iter().enumerate() {
                    self.gen_expr(arg);
                    println!("  pop {}", registers[i]);
                }
                println!("  mov rax, 0x0");
                println!("  call {}", callee_name);
                println!("  push rax");
            }
            Expression::Index { expr, index } => {
                match expr.as_ref() {
                    Expression::LocalVariable { type_, .. } => match type_ {
                        Type::Array { type_, .. } => {
                            self.gen_expr(expr); // pointer
                            self.gen_expr(index);
                            println!("  pop rdi");
                            println!("  pop rax"); // pointer

                            // calc offset
                            println!("  imul rdi, {}", type_.size());
                            println!("  sub rax, rdi");

                            // load value from offset
                            println!("  mov rax, [rax]");
                            println!("  push rax");
                        }
                        Type::Pointer(_) => {
                            self.gen_expr(expr); // pointer
                            self.gen_expr(index);
                            println!("  pop rdi");
                            println!("  pop rax"); // pointer
                            println!("  add rax, rdi");

                            // TODO: judge type
                            // load value from offset
                            println!("  movzx eax, byte ptr [rax]");
                            println!("  push rax");
                        }
                        _ => panic!(
                            "Invalid node: {:?}.\nleft node is not array on index expression.",
                            expr
                        ),
                    },
                    _ => panic!(
                        "Invalid node: {:?}.\nleft node is not array on index expression.",
                        expr
                    ),
                }
            }
            Expression::Binary { lhs, op, rhs } => {
                match op {
                    BinaryOperator::Plus => match lhs.as_ref() {
                        Expression::LocalVariable { type_, .. } => match type_ {
                            Type::Pointer(_) => {
                                self.gen_expr(lhs);
                                self.gen_expr(rhs);
                                println!("  pop rdi");
                                println!("  pop rax");
                                println!("  imul rdi, {}", type_.size());
                                println!("  sub rax, rdi");
                            }
                            _ => {
                                self.gen_expr(lhs);
                                self.gen_expr(rhs);
                                println!("  pop rdi");
                                println!("  pop rax");
                                println!("  add rax, rdi");
                            }
                        },
                        _ => {
                            self.gen_expr(lhs);
                            self.gen_expr(rhs);
                            println!("  pop rdi");
                            println!("  pop rax");
                            println!("  add rax, rdi");
                        }
                    },
                    BinaryOperator::Minus => match lhs.as_ref() {
                        Expression::LocalVariable { type_, .. } => match type_ {
                            Type::Pointer(_) => {
                                self.gen_expr(lhs);
                                self.gen_expr(rhs);
                                println!("  pop rdi");
                                println!("  pop rax");
                                println!("  imul rdi, {}", type_.size());
                                println!("  add rax, rdi");
                            }
                            _ => {
                                self.gen_expr(lhs);
                                self.gen_expr(rhs);
                                println!("  pop rdi");
                                println!("  pop rax");
                                println!("  sub rax, rdi");
                            }
                        },
                        _ => {
                            self.gen_expr(lhs);
                            self.gen_expr(rhs);
                            println!("  pop rdi");
                            println!("  pop rax");
                            println!("  sub rax, rdi");
                        }
                    },
                    BinaryOperator::Asterisk => {
                        self.gen_expr(lhs);
                        self.gen_expr(rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  imul rax, rdi");
                    }
                    BinaryOperator::Slash => {
                        self.gen_expr(lhs);
                        self.gen_expr(rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cqo");
                        println!("  idiv rdi");
                    }
                    BinaryOperator::Lt => {
                        self.gen_expr(lhs);
                        self.gen_expr(rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::LtEq => {
                        self.gen_expr(lhs);
                        self.gen_expr(rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::Eq => {
                        self.gen_expr(lhs);
                        self.gen_expr(rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::NotEq => {
                        self.gen_expr(lhs);
                        self.gen_expr(rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  setne al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::Assignment => {
                        println!("  # --start assignment");

                        println!("  # --left");
                        match lhs.as_ref() {
                            Expression::Unary { expr, op } => match op {
                                UnaryOperator::Dereference => {
                                    self.gen_expr(expr);
                                }
                                _ => {
                                    panic!("Invalid node: {:?}.\nleft node is not var on assignment expression.", expr);
                                }
                            },
                            Expression::Index { expr, index } => {
                                self.gen_lval(expr);
                                self.gen_expr(index);
                                println!("  pop rdi");
                                println!("  pop rax");

                                // calc offset
                                let element_type = match expr.as_ref() {
                                    Expression::LocalVariable { type_, .. } => match type_ {
                                        Type::Array { type_, .. } => type_.as_ref(),
                                        _ => panic!("Invalid node: {:?}.\nleft node is not var on assignment expression.", expr),
                                    },
                                    _ => panic!("Invalid node: {:?}.\nleft node is not var on assignment expression.", expr),
                                };
                                println!("  add rdi, 1");
                                println!("  imul rdi, {}", element_type.size());
                                println!("  add rax, rdi");
                            }
                            Expression::LocalVariable { .. } => {
                                self.gen_lval(lhs);
                            }
                            _ => {
                                panic!("Invalid node: {:?}.\nleft node is not var on assignment expression.", lhs);
                            }
                        }

                        println!("  # --right");
                        self.gen_expr(rhs);
                        println!("  # --assignment");
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  mov [rax], rdi");
                        println!("  push rdi");
                        println!("  # --end assignment");
                    }
                }
                println!("  push rax");
            }
            _ => unreachable!(),
        }
    }
}
