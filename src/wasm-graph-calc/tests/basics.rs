use wasm_graph_calc::*;
use wasm_graph_calc::roots::find_roots;
use std::collections::HashMap;

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
    let tokens = lex("sin(2+1)").unwrap();

    let a: HashMap<char, ParseTree> = HashMap::new();

    let tree: ParseTree = ParseTree::new(&tokens, &a).unwrap();

    println!("{:?}", tokens);
    println!("{:?}", tree);
}

/*
#[test]
fn intercepts() {
    let graphs: HashMap<char, ParseTree> = HashMap::new();
    let vars: HashMap<char, f64> = HashMap::new();

    let tokens1 = lex("2*x + 1").unwrap();
    let tree1  = ParseTree::new(&tokens1, &graphs).unwrap();


    let tokens2 = lex("3*x").unwrap();
    let tree2  = ParseTree::new(&tokens2, &graphs).unwrap();

    let f = |x: f64| 
        tree1.evaluate(Some(x), &vars).unwrap() - tree2.evaluate(Some(x), &vars).unwrap();

    println!("{:?}", find_roots(f, 0.0, 20.0, 0.01, 0.00001));
}
*/
