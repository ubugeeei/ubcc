use crate::ast::{BinaryOperator, Expression};

pub(crate) fn gen(node: Expression) {
    match node {
        Expression::Integer(int) => {
            println!("  push {}", int);
        }
        Expression::Binary(bin) => {
            gen(*bin.lhs);
            gen(*bin.rhs);
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
                }
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
