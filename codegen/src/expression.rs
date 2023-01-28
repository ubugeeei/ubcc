use ast::{BinaryOperator, Expression, Type, UnaryOperator};

use crate::CodeGenerator;

impl CodeGenerator {
    pub(super) fn gen_expr(&self, node: &Expression) {
        match node {
            Expression::Integer(int) => {
                println!("  push {}", int);
            }
            Expression::Unary(unary) => match unary.op {
                UnaryOperator::Minus => {
                    self.gen_expr(&*unary.expr);
                    println!("  pop rax");
                    println!("  neg rax");
                }
                UnaryOperator::Reference => {
                    self.gen_lval(&*unary.expr);
                }
                UnaryOperator::Dereference => {
                    self.gen_expr(&*unary.expr);
                    println!("  pop rax");
                    println!("  mov rax, [rax]");
                    println!("  push rax");
                }
            },
            Expression::LocalVariable { .. } => {
                self.gen_lval(node);
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
            }
            Expression::Call(call) => {
                if call.callee_name == "sizeof" {
                    match &call.arguments[0] {
                        Expression::LocalVariable { type_, .. } => {
                            println!("  push {}", type_.size());
                        }
                        Expression::Integer(_) | Expression::Binary(_) => {
                            println!("  push 8");
                        }
                        Expression::Unary(u) => match u.op {
                            UnaryOperator::Reference => {
                                println!("  push 8");
                            }
                            UnaryOperator::Dereference => match &*u.expr {
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
                if call.arguments.len() > registers.len() {
                    panic!("too many arguments");
                }
                for (i, arg) in call.arguments.iter().enumerate() {
                    self.gen_expr(arg);
                    println!("  pop {}", registers[i]);
                }
                println!("  mov rax, 0x0");
                println!("  call {}", call.callee_name);
                println!("  push rax");
            }
            Expression::Binary(bin) => {
                match bin.op {
                    BinaryOperator::Plus => match &*bin.lhs {
                        Expression::LocalVariable { type_, .. } => match type_ {
                            Type::Pointer(_) => {
                                self.gen_lval(&*bin.lhs);
                                self.gen_expr(&*bin.rhs);
                                println!("  pop rax");
                                println!("  pop rdi");
                                println!("  imul rax, {}", type_.size());
                                println!("  add rax, rdi");
                            }
                            _ => {
                                self.gen_expr(&*bin.lhs);
                                self.gen_expr(&*bin.rhs);
                                println!("  pop rdi");
                                println!("  pop rax");
                                println!("  add rax, rdi");
                            }
                        },
                        _ => {
                            self.gen_expr(&*bin.lhs);
                            self.gen_expr(&*bin.rhs);
                            println!("  pop rdi");
                            println!("  pop rax");
                            println!("  add rax, rdi");
                        }
                    },
                    BinaryOperator::Minus => match &*bin.lhs {
                        Expression::LocalVariable { type_, .. } => match type_ {
                            Type::Pointer(_) => {
                                self.gen_lval(&*bin.lhs);
                                self.gen_expr(&*bin.rhs);
                                println!("  pop rax");
                                println!("  pop rdi");
                                println!("  imul rax, {}", type_.size());
                                println!("  add rdi, rax");
                                println!("  mov rax, rdi");
                            }
                            _ => {
                                self.gen_expr(&*bin.lhs);
                                self.gen_expr(&*bin.rhs);
                                println!("  pop rdi");
                                println!("  pop rax");
                                println!("  sub rax, rdi");
                            }
                        },
                        _ => {
                            self.gen_expr(&*bin.lhs);
                            self.gen_expr(&*bin.rhs);
                            println!("  pop rdi");
                            println!("  pop rax");
                            println!("  sub rax, rdi");
                        }
                    },
                    BinaryOperator::Asterisk => {
                        self.gen_expr(&*bin.lhs);
                        self.gen_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  imul rax, rdi");
                    }
                    BinaryOperator::Slash => {
                        self.gen_expr(&*bin.lhs);
                        self.gen_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cqo");
                        println!("  idiv rdi");
                    }
                    BinaryOperator::Lt => {
                        self.gen_expr(&*bin.lhs);
                        self.gen_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::LtEq => {
                        self.gen_expr(&*bin.lhs);
                        self.gen_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::Eq => {
                        self.gen_expr(&*bin.lhs);
                        self.gen_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::NotEq => {
                        self.gen_expr(&*bin.lhs);
                        self.gen_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  setne al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::Assignment => {
                        println!("  # --start assignment");

                        println!("  # --left");
                        match &*bin.lhs {
                            Expression::Unary(u) => match u.op {
                                UnaryOperator::Dereference => {
                                    self.gen_expr(&*u.expr);
                                }
                                _ => {
                                    panic!("Invalid node: {:?}.\nleft node is not var on assignment expression.", u);
                                }
                            },
                            Expression::LocalVariable { .. } => {
                                self.gen_lval(&*bin.lhs);
                            }
                            _ => {
                                panic!("Invalid node: {:?}.\nleft node is not var on assignment expression.", bin.lhs);
                            }
                        }

                        println!("  # --right");
                        self.gen_expr(&*bin.rhs);
                        println!("  # --assignment");
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  mov [rax], rdi");
                        println!("  push rdi");
                        println!("  # --end assignment");
                        println!("");
                    }
                }
                println!("  push rax");
            }
            _ => todo!(),
        }
    }
}
