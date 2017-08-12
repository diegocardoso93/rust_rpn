
pub fn rpn_to_infix_equation(expr: &str) -> String {
    let tokens_collection = expr.split_whitespace();
    let tokens: Vec<&str> = tokens_collection.collect();
    let mut n2;
    let mut n1;
    let mut stack = vec!["".to_string()];
    for token in tokens {
        if ["+", "-", "*", "/"].contains(&token) {
            n1 = stack.pop().unwrap().to_string();
            n2 = stack.pop().unwrap().to_string();
            let formatted_equation = &format!("({}{}{})", n1, token, n2);
            stack.push(formatted_equation.to_owned() );
        } else {
            stack.push(token.to_string());
        }
    }
    return stack.pop().unwrap();
}