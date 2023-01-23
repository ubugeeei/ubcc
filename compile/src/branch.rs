use ast::{IfStatement, WhileStatement};

use crate::Compiler;

impl Compiler {
    pub(super) fn compile_if(&self, if_stmt: &IfStatement) {
        println!("# -- start if");
        match if_stmt.alternative.as_ref() {
            Some(alternative) => {
                let label_else = format!(".Lelse{}", rand::random::<u32>());
                let label_end = format!(".Lend{}", rand::random::<u32>());
                self.compile_expr(&if_stmt.condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {label_else}");
                self.compile_stmt(&*if_stmt.consequence);
                println!("  jmp {label_end}");
                println!("{label_else}:");
                self.compile_stmt(&*alternative);
                println!("{label_end}:");
            }
            None => {
                let label = format!(".Lend{}", rand::random::<u32>());
                self.compile_expr(&if_stmt.condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {}", label);
                self.compile_stmt(&*if_stmt.consequence);
                println!("{label}:");
            }
        }
        println!("# -- end if");
        println!("");
    }

    pub(super) fn compile_while(&self, while_stmt: &WhileStatement) {
        println!("# -- start while");
        let label_begin = format!(".Lbegin{}", rand::random::<u32>());
        let label_end = format!(".Lend{}", rand::random::<u32>());
        println!("{label_begin}:");
        self.compile_expr(&while_stmt.condition);
        println!("  pop rax");
        println!("  cmp rax, 0");
        println!("  je {label_end}");
        self.compile_stmt(&*while_stmt.body);
        println!("  jmp {label_begin}");
        println!("{label_end}:");
        println!("# -- end while");
        println!("");
    }
}
