use ast::{Expression, Statement};

use crate::CodeGenerator;

impl CodeGenerator {
    pub(super) fn gen_if(
        &self,
        condition: &Expression,
        consequence: &Box<Statement>,
        alternative: &Option<Box<Statement>>,
    ) {
        println!("# -- start if");
        match alternative {
            Some(alternative) => {
                let label_else = format!(".Lelse{}", rand::random::<u32>());
                let label_end = format!(".Lend{}", rand::random::<u32>());
                self.gen_expr(condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {label_else}");
                self.gen_stmt(consequence);
                println!("  jmp {label_end}");
                println!("{label_else}:");
                self.gen_stmt(alternative);
                println!("{label_end}:");
            }
            None => {
                let label = format!(".Lend{}", rand::random::<u32>());
                self.gen_expr(condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {}", label);
                self.gen_stmt(consequence);
                println!("{label}:");
            }
        }
        println!("# -- end if");
        println!("");
    }

    pub(super) fn gen_while(&self, condition: &Expression, body: &Box<Statement>) {
        println!("# -- start while");
        let label_begin = format!(".Lbegin{}", rand::random::<u32>());
        let label_end = format!(".Lend{}", rand::random::<u32>());
        println!("{label_begin}:");
        self.gen_expr(condition);
        println!("  pop rax");
        println!("  cmp rax, 0");
        println!("  je {label_end}");
        self.gen_stmt(body);
        println!("  jmp {label_begin}");
        println!("{label_end}:");
        println!("# -- end while");
        println!("");
    }
}
