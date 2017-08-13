
mod rpn;

fn main() {
    let expr1 = "9 3 2 + /";
    let infix_equation1 = rpn::rpn_to_infix_equation(expr1);
    println!("expr1 converted: {}", infix_equation1);
    let expr2 = "x y ^ 5 2 * 4 * / 10 +";
    let infix_equation2 = rpn::rpn_to_infix_equation(expr2);
    println!("expr2 converted: {}", infix_equation2);
}
