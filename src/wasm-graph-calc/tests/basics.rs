use wasm_graph_calc::*;

#[test]
fn lexer() {
    let items = lex("3*ln(5)");

    for item in items {
        println!("{:?}", item);
    }

    println!("{:?}", evaluate_string("3*ln(5+2)".to_string()));
}
