use ast::{Expression, Statement};
use helper::rand::rand;

use crate::CodeGenerator;

impl CodeGenerator {
    pub(super) fn gen_for(
        &mut self,
        init: &Option<Box<Statement>>,
        condition: &Option<Expression>,
        post: &Option<Box<Statement>>,
        body: &Box<Statement>,
    ) {
        println!("# -- start for");
        let label_begin = format!(".Lbegin{}", rand());
        let label_end = format!(".Lend{}", rand());

        // init
        match init {
            Some(init) => self.gen_stmt(init),
            None => {}
        }
        println!("{label_begin}:");

        // condition and jump
        match condition {
            Some(ref condition) => {
                self.gen_expr(condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {label_end}");
            }
            None => {}
        }

        // body
        self.gen_stmt(body);

        // update
        match post {
            Some(update) => self.gen_stmt(update),
            None => {}
        }

        println!("  jmp {label_begin}");
        println!("{label_end}:");
        println!("# -- end for");
        println!("");
    }

    // TODO: while
}
