use crate::ast::{BinaryOperator, Expression, Program, Statement};

pub(crate) fn gen(node: Program) {
    for stmt in node.statements {
        match stmt {
            Statement::Expression(expr) => {
                gen_expr(expr);
            }
        }
    }

}

fn gen_expr(node: Expression) {
    match node {
        Expression::Integer(int) => {
            println!("  push {}", int);
        }
        Expression::Binary(bin) => {
            gen_expr(*bin.lhs);
            gen_expr(*bin.rhs);
            println!("  pop rdi");
            println!("  pop rax");
            match bin.op {
                BinaryOperator::Plus => {
                    println!("  add rax, rdi");
                }
                BinaryOperator::Minus => {
                    println!("  sub rax, rdi");
                }
                BinaryOperator::Asterisk => {
                    println!("  imul rax, rdi");
                }
                BinaryOperator::Slash => {
                    println!("  cqo");
                    println!("  idiv rdi");
                }
                BinaryOperator::Lt => {
                    println!("  cmp rax, rdi");
                    println!("  setl al");
                    println!("  movzb rax, al");
                }
                BinaryOperator::LtEq => {
                    println!("  cmp rax, rdi");
                    println!("  setle al");
                    println!("  movzb rax, al");
                }
                BinaryOperator::Eq => {
                    println!("  cmp rax, rdi");
                    println!("  sete al");
                    println!("  movzb rax, al");
                }
                BinaryOperator::NotEq => {
                    println!("  cmp rax, rdi");
                    println!("  setne al");
                    println!("  movzb rax, al");
                } // _ => {
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
