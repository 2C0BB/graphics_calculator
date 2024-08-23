use wasm_graph_calc::*;

#[test]
fn lexer() {

    let s = "(3 + 2) *ln( 5 + 2)";

    let items = lex(s);

    for item in items {
        println!("{:?}", item);
    }

    println!("{:?}", evaluate_string(s.to_string()));
}
