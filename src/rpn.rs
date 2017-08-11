
pub fn rpn_to_infix(expr: &str) {
    println!("My expression: {}", expr);
    let tokens_collection = expr.split_whitespace();
    let tokens: Vec<&str> = tokens_collection.collect();
    for token in tokens {
        println!("{}", token);
    }
}