use crate::ast::{
    BinaryOperator, Expression, ForStatement, FunctionDefinition, IfStatement, InitDeclaration,
    Program, Statement, WhileStatement,
};

// entry
pub(crate) fn gen(node: Program) {
    let codegen = CodeGenerator::new(node);
    codegen.gen();
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
    fn gen(&self) {
        for stmt in self.ast.statements.iter() {
            self.gen_stmt(stmt);
        }
    }

    fn gen_stmt(&self, node: &Statement) {
        match node {
            Statement::If(if_stmt) => self.gen_if(if_stmt),
            Statement::While(while_stmt) => self.gen_while(while_stmt),
            Statement::For(for_stmt) => self.gen_for(for_stmt),
            Statement::Block(stmts) => self.gen_stmts(stmts),
            Statement::Expression(expr) => self.gen_expr(expr),
            Statement::Return(expr) => self.gen_return(expr),
            Statement::FunctionDefinition(function_def) => {
                self.gen_function_definition(function_def)
            }
            Statement::InitDeclaration(init_decl) => self.gen_init_declaration(init_decl),
            _ => todo!(),
        }
    }

    fn gen_stmts(&self, stmts: &[Statement]) {
        for stmt in stmts.iter() {
            self.gen_stmt(stmt);
        }
    }

    fn gen_if(&self, if_stmt: &IfStatement) {
        println!("# -- start if");
        match if_stmt.alternative.as_ref() {
            Some(alternative) => {
                let label_else = format!(".Lelse{}", rand::random::<u32>());
                let label_end = format!(".Lend{}", rand::random::<u32>());
                self.gen_expr(&if_stmt.condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {label_else}");
                self.gen_stmt(&*if_stmt.consequence);
                println!("  jmp {label_end}");
                println!("{label_else}:");
                self.gen_stmt(&*alternative);
                println!("{label_end}:");
            }
            None => {
                let label = format!(".Lend{}", rand::random::<u32>());
                self.gen_expr(&if_stmt.condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {}", label);
                self.gen_stmt(&*if_stmt.consequence);
                println!("{label}:");
            }
        }
        println!("# -- end if");
        println!("");
    }

    fn gen_while(&self, while_stmt: &WhileStatement) {
        println!("# -- start while");
        let label_begin = format!(".Lbegin{}", rand::random::<u32>());
        let label_end = format!(".Lend{}", rand::random::<u32>());
        println!("{label_begin}:");
        self.gen_expr(&while_stmt.condition);
        println!("  pop rax");
        println!("  cmp rax, 0");
        println!("  je {label_end}");
        self.gen_stmt(&*while_stmt.body);
        println!("  jmp {label_begin}");
        println!("{label_end}:");
        println!("# -- end while");
        println!("");
    }

    fn gen_for(&self, for_stmt: &ForStatement) {
        println!("# -- start for");
        let label_begin = format!(".Lbegin{}", rand::random::<u32>());
        let label_end = format!(".Lend{}", rand::random::<u32>());

        // init
        match for_stmt.init.as_ref() {
            Some(init) => self.gen_stmt(init),
            None => {}
        }
        println!("{label_begin}:");

        // condition and jump
        match for_stmt.condition.as_ref() {
            Some(ref condition) => {
                self.gen_expr(condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {label_end}");
            }
            None => {}
        }

        // body
        self.gen_stmt(for_stmt.body.as_ref());

        // update
        match for_stmt.post.as_ref() {
            Some(update) => self.gen_stmt(update),
            None => {}
        }

        println!("  jmp {label_begin}");
        println!("{label_end}:");
        println!("# -- end for");
        println!("");
    }

    fn gen_return(&self, node: &Expression) {
        println!("  # -- return");
        self.gen_expr(node);
        println!("  # epilogue");
        println!("  pop rax");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
        println!("");
    }

    fn gen_function_definition(&self, function_def: &FunctionDefinition) {
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
        self.gen_stmts(&function_def.body);
        println!("");
    }

    fn gen_init_declaration(&self, init_decl: &InitDeclaration) {
        println!("  # -- init declaration");
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

    fn gen_expr(&self, node: &Expression) {
        match node {
            Expression::Integer(int) => {
                println!("  push {}", int);
            }
            Expression::LocalVariable { .. } => {
                self.gen_lval(node);
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
            }
            Expression::Call(call) => {
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
                    BinaryOperator::Plus => {
                        self.gen_expr(&*bin.lhs);
                        self.gen_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  add rax, rdi");
                    }
                    BinaryOperator::Minus => {
                        self.gen_expr(&*bin.lhs);
                        self.gen_expr(&*bin.rhs);
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  sub rax, rdi");
                    }
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
                        self.gen_lval(&*bin.lhs);
                        println!("  # --right");
                        self.gen_expr(&*bin.rhs);
                        println!("  # --assignment");
                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  mov [rax], rdi");
                        println!("  push rdi");
                        println!("  # --end assignment");
                        println!("");
                    } //
                      // _ => {
                      //     panic!("Invalid binary operator: {:?}", bin.op);
                      // }
                }
                println!("  push rax");
            }
            _ => {
                panic!("Invalid node: {:?}", node);
            }
        }
    }

    fn gen_lval(&self, node: &Expression) {
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

    fn gen_init_lval(&self, offset: i32) {
        println!("  mov rax, rbp");
        println!("  sub rax, {offset}");
        println!("  push rax");
        println!("");
    }
}
