use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[derive(Clone, Debug)]
pub enum LexerTokenType {
    Add,
    Sub,
    Div,
    Mul,
    Func(Vec<Vec<LexerToken>>, String),
    Var(char),
    Num(f32)
}

fn value_operator(input: &LexerTokenType) -> u32 {
    match input {
        LexerTokenType::Func(..) => 0,
        LexerTokenType::Mul => 1,
        LexerTokenType::Div => 1,
        LexerTokenType::Add => 2,
        LexerTokenType::Sub => 2,

        LexerTokenType::Num(_) | LexerTokenType::Var(_) => panic!()
    }
}


#[derive(Clone, Debug)]
pub struct LexerToken {
    token_type: LexerTokenType,
    bracket_depth: u32
}

const FUNCTIONS: [&str; 1] = ["ln"];

fn string_to_token(s: &str) -> Option<LexerTokenType> {

    dbg!(s);

    Some(match s {
        "+" => LexerTokenType::Add,
        "-" => LexerTokenType::Sub,
        "*" => LexerTokenType::Mul,
        "/" => LexerTokenType::Div,

        _ => {
            return None;
        }
    })
}

pub fn lex(input: &str) -> Vec<LexerToken> {

    println!("input is {}", input);

    let mut iter = input.chars().peekable();

    let mut bracket_depth: u32 = 0;

    let mut out: Vec<LexerToken> = Vec::new();

    while let Some(character) = iter.next() {
        if character == ' ' {
            continue;
        }

        if character == '(' {
            bracket_depth += 1;
            continue;
        }

        if character == ')' {
            bracket_depth -= 1;
            continue;
        }


        if character.is_numeric() {
            let mut num_buf: Vec<char> = vec![character];
            
            while let Some(c) = iter.peek() {
                if !c.is_numeric() {
                    break;
                }

                num_buf.push(*c);
                iter.next();
            }

            let number: f32 = num_buf
                .iter()
                .collect::<String>()
                .parse()
                .unwrap();

            out.push(LexerToken {
                token_type: LexerTokenType::Num(number),
                bracket_depth
            });

            continue;
        }

        /*
        if !character.is_alphabetic() {
            dbg!(out);
            panic!("not alphabetic");
        }
        */

        let mut buf: Vec<char> = vec![character];

        // fix this later
        let buf_as_string: String = buf.iter().collect();
        if let Some(token_type) = string_to_token(&buf_as_string) {
            out.push(LexerToken { token_type, bracket_depth });
            continue;
        }

        // and rework this
        while let Some(next_c) = iter.peek() {
            if *next_c == ' ' {
                break;
            }

            buf.push(*next_c);
            iter.next();

            let buf_as_string: String = buf.iter().collect();
            if let Some(token_type) = string_to_token(&buf_as_string) {

                out.push(LexerToken { token_type, bracket_depth });

            } else if FUNCTIONS.contains(&buf_as_string.as_str()) {

                // skip all the whitespace before the opening bracket for the function
                while let Some(&' ') = iter.peek() {
                    iter.next();
                }

                if let Some(&'(') = iter.peek() {
                    iter.next();

                    // this is NOT for operation ordering, this is just to match the actual closing bracket rather than some inner ones
                    let mut inner_bracket_depth: u32 = 1;
                    // start at 1 because the opening brace has already been consumed

                    // these are characters inside function braces that will be lexed recursively
                    let mut func_chars_sets: Vec<Vec<char>> = vec![Vec::new()];
                    let mut func_chars_sets_ptr: usize = 0;

                    let mut closing_found: bool = false;

                    while let Some(func_next_char) = iter.peek() {
                        if *func_next_char == ')' {

                            inner_bracket_depth -= 1;

                            if inner_bracket_depth == 0 {
                                closing_found = true;

                                iter.next();
                                break;
                            }
                        }

                        if *func_next_char == '(' {
                            inner_bracket_depth += 1;
                            // dont consume because this is still to be added to the function inp
                        }

                        if *func_next_char == ',' {
                            func_chars_sets.push(Vec::new());
                            func_chars_sets_ptr += 1;
                            iter.next();
                            continue;
                        }

                        func_chars_sets[func_chars_sets_ptr].push(*func_next_char);
                        iter.next();
                    }

                    if !closing_found {
                        panic!("implement real error, but there is no closing brace for this function");
                    }

                    let lexed_func_args: Vec<Vec<LexerToken>> = func_chars_sets.into_iter()
                        .map(|set| set.iter().collect::<String>())
                        .map(|set_string| lex(&set_string))
                        .collect();

                    out.push(
                        LexerToken {
                            token_type: LexerTokenType::Func(lexed_func_args, buf_as_string),
                            bracket_depth
                        }
                    );

                    buf = Vec::new();

                    break;

                } else {
                    // its actually variables????8
                    break;
                    // this will be done by outside extend func as buf is not cleared
                }
            }

        }

        out.extend(buf.into_iter().map(|x| LexerToken {
            token_type: LexerTokenType::Var(x),
            bracket_depth
        }));

    }

    out
}

#[derive(Debug)]
struct OperatorOrdering {
    bracket_depth: u32,
    operator_val: u32,
    position: usize
}

impl OperatorOrdering {
    fn is_lower_precedence_than(&self, other: &Self) -> bool {
        if self.bracket_depth != other.bracket_depth {
            return self.bracket_depth < other.bracket_depth;
        }

        if self.operator_val != other.operator_val {
            return self.operator_val > other.operator_val;
        }

        assert_ne!(self.position, other.position);

        self.position > other.position
    }
}

fn find_next_op(items: &[LexerToken]) -> Option<usize> {

    let mut next_op_pos: Option<usize> = None;
    let mut lowest_precedence: Option<OperatorOrdering> = None;

    for (pos, item) in items.iter().enumerate() {
        match item.token_type {
            LexerTokenType::Num(_) | LexerTokenType::Var(_) => {
                continue;
            }

            _ => {}
        }

        let op_val: u32 = value_operator(&item.token_type);

        let precedence: OperatorOrdering = OperatorOrdering {
            bracket_depth: item.bracket_depth,
            operator_val: op_val,
            position: pos
        };
        
        let should_swap: bool = match &lowest_precedence {
            Some(last_val) => {

                precedence.is_lower_precedence_than(last_val)
            },

            None => true
        };

        if should_swap {
            next_op_pos = Some(pos);
            lowest_precedence = Some(precedence);
        }
    }    

    next_op_pos
}

type TreeLink = Option<Box<TreeNode>>;
pub type SharedVars = Rc<RefCell<HashMap<char, f32>>>;

#[derive(Debug)]
pub struct TreeNode {
    token_type: LexerTokenType,
    function_args: Vec<TreeNode>,

    left: TreeLink,
    right: TreeLink
}

impl TreeNode {
    fn new_from_tokens(items: &[LexerToken]) -> Result<TreeNode, ParseError>  {
        let next_op_pos = find_next_op(items);

        match next_op_pos {
            Some(pos) => {
                let token_type = items[pos].token_type.clone();

                if let LexerTokenType::Func(vars, _name) = &token_type {
                    assert_eq!(items.len(), 1);

                    return Ok(TreeNode {
                        token_type: token_type.clone(),
                        function_args: vars.iter().map(|x| TreeNode::new_from_tokens(x).unwrap()).collect(),

                        left: None,
                        right: None,
                    });
                }
                
                let left_items = &items[0..pos];
                let left_node = TreeNode::new_from_tokens(left_items)?;

                let right_items = &items[pos+1..];
                let right_node = TreeNode::new_from_tokens(right_items)?;


                Ok(TreeNode { 
                    token_type,
                    function_args: Vec::new(),
                    left: Some(Box::new(left_node)), 
                    right: Some(Box::new(right_node)), 
                })
            },

            None => {

                if items.len() != 1 {
                    return Err(ParseError);
                }

                Ok(TreeNode {
                    token_type: items[0].token_type.clone(),
                    function_args: Vec::new(),
                    left: None,
                    right: None,
                })
            }
        }
    }

    fn evaluate(&self, vars: SharedVars) -> f32 {
        if let LexerTokenType::Num(num) = self.token_type {

            // num shouldn't have left and right args
            assert!(self.left.is_none());
            assert!(self.right.is_none());

            return num;

        } else if let LexerTokenType::Var(var) = self.token_type {
            // var shouldn't have left and right args
            assert!(self.left.is_none());
            assert!(self.right.is_none());

            let var_value = *vars.borrow().get(&var)
                .unwrap();

            return var_value;
        }

        if let LexerTokenType::Func(_, name) = &self.token_type {
            if name == "ln" {
                assert_eq!(self.function_args.len(), 1);

                return self.function_args[0].evaluate(vars.clone()).ln();
            }

            unimplemented!()
        }

        dbg!(self);

        let left_val = self.left.as_ref().unwrap().evaluate(vars.clone());
        let right_val = self.right.as_ref().unwrap().evaluate(vars.clone());

        match &self.token_type {
            LexerTokenType::Add => left_val + right_val,
            LexerTokenType::Sub => left_val - right_val,
            LexerTokenType::Mul => left_val * right_val,
            LexerTokenType::Div => left_val / right_val,

            LexerTokenType::Num(_) | LexerTokenType::Var(_) | LexerTokenType::Func(..) => unreachable!()
        }
    }

}

#[derive(Debug)]
pub struct ParseTree {
    inner_tree: TreeLink
}

#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to evaluate equation")
    }
}

impl std::error::Error for ParseError {}

impl ParseTree {
    pub fn new(lexed: &[LexerToken]) -> Result<ParseTree, ParseError> {
        let inner_tree = Some(Box::new(
            TreeNode::new_from_tokens(lexed)?
        ));

        Ok(ParseTree { inner_tree })
    }

    pub fn evaluate(&self, vars: SharedVars) -> f32 {
        if let Some(tree) = &self.inner_tree {
            tree.evaluate(vars.clone())
        } else {
            panic!()
        }
    }
}

#[wasm_bindgen]
pub fn evaluate_string(input: String) -> Option<f32> {

    let vars: SharedVars = Rc::new(RefCell::new(HashMap::new()));

    let lexed = lex(&input);
    match ParseTree::new(&lexed) {
        Ok(tree) => {
            Some(tree.evaluate(vars))
        }

        Err(_) => None
    }
}
