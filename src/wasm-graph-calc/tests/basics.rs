use wasm_graph_calc::*;

#[test]
fn lexer() {

    //let s = "(3 + 2) *ln( 5 + 2)";

    let mut evaluator: Evaluator = Evaluator::new();

    println!("{:?}", evaluator.evaluate("a=2*5".to_string()));
    println!("{:?}", evaluator.evaluate("a + 1".to_string()));
}
