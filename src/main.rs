
mod rpn;

fn main() {
    let expr = "2 3 4 + /";
    let infix_equation = rpn::rpn_to_infix_equation(expr);
    println!("{}", infix_equation);
}
