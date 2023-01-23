use ast::ForStatement;

use crate::Compiler;

impl Compiler {
    pub(super) fn compile_for(&self, for_stmt: &ForStatement) {
        println!("# -- start for");
        let label_begin = format!(".Lbegin{}", rand::random::<u32>());
        let label_end = format!(".Lend{}", rand::random::<u32>());

        // init
        match for_stmt.init.as_ref() {
            Some(init) => self.compile_stmt(init),
            None => {}
        }
        println!("{label_begin}:");

        // condition and jump
        match for_stmt.condition.as_ref() {
            Some(ref condition) => {
                self.compile_expr(condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {label_end}");
            }
            None => {}
        }

        // body
        self.compile_stmt(for_stmt.body.as_ref());

        // update
        match for_stmt.post.as_ref() {
            Some(update) => self.compile_stmt(update),
            None => {}
        }

        println!("  jmp {label_begin}");
        println!("{label_end}:");
        println!("# -- end for");
        println!("");
    }

    // TODO: while
}
