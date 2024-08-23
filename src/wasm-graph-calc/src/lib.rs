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

// return the variables, and the function name
// this will be called when a openin brace is next, but not in buf
fn find_function(input: &str) -> Option<(Vec<char>, &str)> {
    if input.is_empty() {
        return None;
    }

    for fun in FUNCTIONS {
        if let Some(pos) = input.find(fun) {
            let leftover_chars: Vec<char> = input[..pos]
                .chars()
                .collect();

            let function_name: &str = &input[pos..];

            return Some((leftover_chars, function_name));
        }
    }

    None
}

fn generate_function<T>(i: &mut T, function_name: String) -> LexerTokenType
    where T: Iterator<Item = char>
{

    println!("STARTED");

    let mut bracket_depth: u32 = 1;

    let mut args_sets: Vec<Vec<char>> = Vec::new();
    let mut current_set: Vec<char> = Vec::new();

    let mut closing_found: bool = false;

    while let Some(c) = i.next() {

        println!("C: {}", c);

        if c == '(' {
            bracket_depth += 1;
        }

        if c == ')' {
            bracket_depth -= 1;

            if bracket_depth == 0 {
                closing_found = true;
                break;
            }
        }

        if c == ',' {
            args_sets.push(current_set);
            current_set = Vec::new();

            continue;
        }

        current_set.push(c);
    }

    args_sets.push(current_set);

    if !closing_found {
        panic!();
    }

    let parsed_sets: Vec<Vec<LexerToken>> = args_sets.into_iter()
        .map(|set| set.iter().collect::<String>())
        .map(|set_string| {println!("SET: {}", set_string); lex(&set_string)})
        .collect();

    LexerTokenType::Func(parsed_sets, function_name)
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

        if ['+', '-', '*', '/'].contains(&character) {
            let token_type: LexerTokenType = match character {
                '+' => LexerTokenType::Add,
                '-' => LexerTokenType::Sub,
                '*' => LexerTokenType::Mul,
                '/' => LexerTokenType::Div,

                _ => unreachable!()
            };

            out.push(LexerToken {token_type, bracket_depth})
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


        let mut buffer: Vec<char> = Vec::new();
        while let Some(buf_char_next) = iter.peek() {

            if *buf_char_next == ' ' {
                break;
            }

            if *buf_char_next == '(' {

                let buffer_string: String = buffer.iter().collect();

                if let Some((vars, function_name)) = find_function(&buffer_string) {
                    iter.next();

                    for v in vars {
                        out.push(LexerToken {
                            token_type: LexerTokenType::Var(v),
                            bracket_depth
                        });
                    }

                    let function_type = generate_function(
                        &mut iter,
                        function_name.to_string()
                    );

                    out.push(LexerToken {
                        token_type: function_type,
                        bracket_depth
                    });

                    buffer = Vec::new();
                    break;
                }
            }

            if !buf_char_next.is_alphabetic() {
                break;
            }

            buffer.push(*buf_char_next);
            iter.next();
        }

        for new_var in buffer.iter() {
            out.push(LexerToken {
                token_type: LexerTokenType::Var(*new_var),
                bracket_depth
            });
        }
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

                    dbg!(items);

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
