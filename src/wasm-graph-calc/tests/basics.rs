use wasm_graph_calc::*;

/*
#[test]
fn lexer() {

    //let s = "(3 + 2) *ln( 5 + 2)";

    let mut evaluator: Evaluator = Evaluator::new();

    println!("{:?}", evaluator.evaluate("a=2*5".to_string()));
    println!("{:?}", evaluator.evaluate("a + 1".to_string()));
}
*/

/*
#[test]
fn numba2() {
    let tokens1 = lex("2").unwrap();
    let tokens2 = lex("(x-3)*(x-3)").unwrap();

    println!("intercept: {:?}", estimate_intercepts(&tokens1, &tokens2));
}
*/

#[test]
fn integration() {
    let tokens = lex("int(f(x), 0, 2, 100)").unwrap();

    println!("{:?}", tokens);
}
