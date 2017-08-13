
pub fn rpn_to_infix_equation(expr: &str) -> String {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let mut n2: String;
    let mut n1: String;
    let mut stack: Vec<String> = vec!["".to_string()];
    for token in tokens {
        if ["+", "-", "*", "/", "^"].contains(&token) {
            n2 = stack.pop().unwrap().to_string();
            n1 = stack.pop().unwrap().to_string();
            let formatted_equation = &format!("({} {} {})", n1, token, n2);
            stack.push(formatted_equation.to_owned() );
        } else {
            stack.push(token.to_string());
        }
    }
    return stack.pop().unwrap();
}