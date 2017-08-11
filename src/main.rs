
mod rpn;

fn main() {
    println!("Hello, world!");
    let expr = " 2 3 4 + 2 /";
    rpn::rpn_to_infix(expr);
}
