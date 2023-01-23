use ast::{
    BinaryOperator, Expression, FunctionDefinition, InitDeclaration, Program, Statement, Type,
    UnaryOperator,
};

mod branch;
mod loop_;

// entry
pub fn compile(input: String) {
    let compiler = Compiler::new(input);
    compiler.compile();
}

struct Compiler {
    ast: Program,
}
impl Compiler {
    fn new(input: String) -> Self {
        let ast = match parse::parse(input) {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        Self { ast }
    }
}
impl Compiler {
    fn compile(&self) {
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("");
        for stmt in self.ast.statements.iter() {
            self.compile_stmt(stmt);
        }
    }

    fn compile_stmt(&self, node: &Statement) {
        match node {
            Statement::If(if_stmt) => self.compile_if(if_stmt),
            Statement::While(while_stmt) => self.compile_while(while_stmt),
            Statement::For(for_stmt) => self.compile_for(for_stmt),
            Statement::Block(stmts) => self.compile_stmts(stmts),
            Statement::Expression(expr) => self.compile_expr(expr),
            Statement::Return(expr) => self.compile_return(expr),
            Statement::FunctionDefinition(function_def) => {
                self.compile_function_definition(function_def)
            }
            Statement::InitDeclaration(init_decl) => self.compile_init_declaration(init_decl),
        }
    }

    fn compile_stmts(&self, stmts: &[Statement]) {
        for stmt in stmts.iter() {
            self.compile_stmt(stmt);
        }
    }

    fn compile_return(&self, node: &Expression) {
        println!("  # -- return");
        self.compile_expr(node);
        println!("  # epilogue");
        println!("  pop rax");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
        println!("");
    }

    fn compile_function_definition(&self, function_def: &FunctionDefinition) {
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

    fn compile_init_declaration(&self, init_decl: &InitDeclaration) {
        println!("  # -- init declaration {}", init_decl.name);
        self.compile_init_lval(init_decl.offset);
        match init_decl.init {
            Some(ref init) => self.compile_expr(init),
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

    fn compile_expr(&self, node: &Expression) {
        match node {
            Expression::Integer(int) => {
                println!("  push {}", int);
            }
            Expression::Unary(unary) => match unary.op {
                UnaryOperator::Minus => {
                    self.compile_expr(&*unary.expr);
                    println!("  pop rax");
                    println!("  neg rax");
                }
                UnaryOperator::Reference => {
                    self.compile_lval(&*unary.expr);
                }
                UnaryOperator::Dereference => {
                    self.compile_expr(&*unary.expr);
                    println!("  pop rax");
                    println!("  mov rax, [rax]");
                    println!("  push rax");
                }
            },
            Expression::LocalVariable { .. } => {
                self.compile_lval(node);
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
                    self.compile_expr(arg);
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
                                self.compile_lval(&*bin.lhs);
                                self.compile_expr(&*bin.rhs);
                                println!("  pop rax");
                                println!("  pop rdi");
                                println!("  imul rax, {}", type_.size());
                                println!("  add rax, rdi");
                            }
                            _ => {
                                self.compile_expr(&*bin.lhs);
                                self.compile_expr(&*bin.rhs);
                                println!("  pop rdi");
                                println!("  pop rax");
                                println!("  add rax, rdi");
                            }
                        },
                        _ => {
                            self.compile_expr(&*bin.lhs);
                            self.compile_expr(&*bin.rhs);
                            println!("  pop rdi");
                            println!("  pop rax");
                            println!("  add rax, rdi");
                        }
                    },
                    BinaryOperator::Minus => match &*bin.lhs {
                        Expression::LocalVariable { type_, .. } => match type_ {
                            Type::Pointer(_) => {
                                self.compile_lval(&*bin.lhs);
                                self.compile_expr(&*bin.rhs);
                                println!("  pop rax");
                                println!("  pop rdi");
                                println!("  imul rax, {}", type_.size());
                                println!("  add rdi, rax");
                                println!("  mov rax, rdi");
                            }
                            _ => {
                                self.compile_expr(&*bin.lhs);
                                self.compile_expr(&*bin.rhs);
                                println!("  pop rdi");
                                println!("  pop rax");
                                println!("  sub rax, rdi");
                            }
                        },
                        _ => {
                            self.compile_expr(&*bin.lhs);
                            self.compile_expr(&*bin.rhs);
                            println!("  pop rdi");
                            println!("  pop rax");
                            println!("  sub rax, rdi");
                        }
                    },
                    BinaryOperator::Asterisk => {
                        self.compile_expr(&*bin.lhs);
                        self.compile_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  imul rax, rdi");
                    }
                    BinaryOperator::Slash => {
                        self.compile_expr(&*bin.lhs);
                        self.compile_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cqo");
                        println!("  idiv rdi");
                    }
                    BinaryOperator::Lt => {
                        self.compile_expr(&*bin.lhs);
                        self.compile_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::LtEq => {
                        self.compile_expr(&*bin.lhs);
                        self.compile_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::Eq => {
                        self.compile_expr(&*bin.lhs);
                        self.compile_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::NotEq => {
                        self.compile_expr(&*bin.lhs);
                        self.compile_expr(&*bin.rhs);
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
                                    self.compile_expr(&*u.expr);
                                }
                                _ => {
                                    panic!("Invalid node: {:?}.\nleft node is not var on assignment expression.", u);
                                }
                            },
                            Expression::LocalVariable { .. } => {
                                self.compile_lval(&*bin.lhs);
                            }
                            _ => {
                                panic!("Invalid node: {:?}.\nleft node is not var on assignment expression.", bin.lhs);
                            }
                        }

                        println!("  # --right");
                        self.compile_expr(&*bin.rhs);
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
        }
    }

    fn compile_lval(&self, node: &Expression) {
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

    fn compile_init_lval(&self, offset: usize) {
        println!("  mov rax, rbp");
        println!("  sub rax, {offset}");
        println!("  push rax");
        println!("");
    }
}
