use crate::lexer::Lexer;

mod lexer;

fn main() {
    "<layout:[8,8]#[|-.]/>;"
        .tokens()
        .for_each(|token| println!("{:?}", token));
}
