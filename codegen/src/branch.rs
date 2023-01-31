use ast::{Expression, Statement};
use helper::rand::rand;

use crate::CodeGenerator;

impl CodeGenerator {
    pub(super) fn gen_if(
        &mut self,
        condition: &Expression,
        consequence: &Box<Statement>,
        alternative: &Option<Box<Statement>>,
    ) {
        println!("# -- start if");
        match alternative {
            Some(alternative) => {
                let label_else = format!(".Lelse{}", rand());
                let label_end = format!(".Lend{}", rand());
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
                let label = format!(".Lend{}", rand());
                self.gen_expr(condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {}", label);
                self.gen_stmt(consequence);
                println!("{label}:");
            }
        }
        println!("# -- end if");
    }

    pub(super) fn gen_while(&mut self, condition: &Expression, body: &Box<Statement>) {
        println!("# -- start while");
        let label_begin = format!(".Lbegin{}", rand());
        let label_end = format!(".Lend{}", rand());
        println!("{label_begin}:");
        self.gen_expr(condition);
        println!("  pop rax");
        println!("  cmp rax, 0");
        println!("  je {label_end}");
        self.gen_stmt(body);
        println!("  jmp {label_begin}");
        println!("{label_end}:");
        println!("# -- end while");
    }
}
