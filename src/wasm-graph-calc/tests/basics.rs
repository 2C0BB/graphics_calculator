use wasm_graph_calc::*;

#[test]
fn lexer() {

    //let s = "(3 + 2) *ln( 5 + 2)";
    let s = "a";

    let items = lex(s);
    println!("{}", items.len());

    for item in items {
        println!("{:?}", item);
    }

    println!("{:?}", evaluate_string(s.to_string()));

}
