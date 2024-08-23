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
    Num(f64)
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

fn generate_function<T>(i: &mut T, function_name: String) -> Result<LexerTokenType, LexError>
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

    let parsed_sets: Vec<Result<Vec<LexerToken>, LexError>> = args_sets.into_iter()
        .map(|set| set.iter().collect::<String>())
        .map(|set_string| {println!("SET: {}", set_string); lex(&set_string)})
        .collect();

    if parsed_sets.iter().any(|x| x.is_err()) {
        return Err(LexError);
    }

    let parsed_sets: Vec<Vec<LexerToken>> = parsed_sets.into_iter()
        .map(|x| x.unwrap())
        .collect();

    Ok(LexerTokenType::Func(parsed_sets, function_name))
}

fn is_valid_brackets(input: &str) -> bool {
    let mut bracket_depth: u32 = 0;

    for c in input.chars() {
        match c {
            '(' => {bracket_depth += 1},
            ')' => {
                if bracket_depth == 0 {
                    return false;
                }

                bracket_depth -= 1;
            },

            _ => {}
        }
    }

    bracket_depth == 0
}

#[derive(Debug)]
pub struct LexError;

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to lex expression")
    }
}

impl std::error::Error for LexError {}

pub fn lex(input: &str) -> Result<Vec<LexerToken>, LexError> {

    if !is_valid_brackets(input) {
        return Err(LexError);
    }

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

            out.push(LexerToken {token_type, bracket_depth});
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

            let number: f64 = num_buf
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


        let mut buffer: Vec<char> = vec![character];
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
                    )?;

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

            /*
            if iter.next().is_none() {
                break;
            };
            */
        }

        for new_var in buffer.iter() {
            out.push(LexerToken {
                token_type: LexerTokenType::Var(*new_var),
                bracket_depth
            });
        }
    }

    Ok(out)
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

#[derive(Debug)]
pub struct TreeNode {
    token_type: LexerTokenType,
    function_args: Vec<TreeNode>,

    left: TreeLink,
    right: TreeLink
}

#[derive(Debug)]
pub struct EvaluateError;

impl std::fmt::Display for EvaluateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to evaluate expression")
    }
}

impl std::error::Error for EvaluateError {}

impl TreeNode {
    fn new_from_tokens(items: &[LexerToken]) -> Result<TreeNode, ParseError>  {
        let next_op_pos = find_next_op(items);

        match next_op_pos {
            Some(pos) => {
                let token_type = items[pos].token_type.clone();

                if let LexerTokenType::Func(vars, _name) = &token_type {

                    assert_eq!(items.len(), 1);

                    let function_args: Vec<Result<TreeNode, ParseError>> = vars.iter()
                        .map(|x| TreeNode::new_from_tokens(x))
                        .collect();

                    if function_args.iter().any(|x| x.is_err()) {
                        return Err(ParseError);
                    }

                    let function_args: Vec<TreeNode> = function_args.into_iter()
                        .map(|x| x.unwrap())
                        .collect();

                    return Ok(TreeNode {
                        token_type: token_type.clone(),
                        function_args,

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

    fn evaluate(&self, vars: &HashMap<char, f64>) -> Result<f64, EvaluateError> {
        if let LexerTokenType::Num(num) = self.token_type {

            // num shouldn't have left and right args
            assert!(self.left.is_none());
            assert!(self.right.is_none());
            assert!(self.function_args.is_empty());

            return Ok(num);

        } else if let LexerTokenType::Var(var) = self.token_type {
            // var shouldn't have left and right args
            assert!(self.left.is_none());
            assert!(self.right.is_none());
            assert!(self.function_args.is_empty());

            let var_value: f64 = *vars.get(&var).ok_or(EvaluateError)?;

            return Ok(var_value);
        }

        if let LexerTokenType::Func(_, name) = &self.token_type {

            assert!(self.left.is_none());
            assert!(self.right.is_none());

            if name == "ln" {
                assert_eq!(self.function_args.len(), 1);

                return Ok(self.function_args[0].evaluate(vars)?.ln());
            }

            unimplemented!()
        }

        let left_val: f64 = self.left.as_ref().unwrap().evaluate(vars)?;
        let right_val: f64 = self.right.as_ref().unwrap().evaluate(vars)?;

        Ok(match &self.token_type {
            LexerTokenType::Add => left_val + right_val,
            LexerTokenType::Sub => left_val - right_val,
            LexerTokenType::Mul => left_val * right_val,
            LexerTokenType::Div => left_val / right_val,

            LexerTokenType::Num(_) | LexerTokenType::Var(_) | LexerTokenType::Func(..) => unreachable!()
        })
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

    pub fn evaluate(&self, vars: &HashMap<char, f64>) -> Result<f64, EvaluateError> {
        if let Some(tree) = &self.inner_tree {
            tree.evaluate(&vars)
        } else {
            panic!()
        }
    }
}

/*
#[wasm_bindgen]
pub struct EvaluateResponse {
    value: f64,
    var: Option<char>
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct EvaluateVarInput {
    var: char,
    value: f64
}

#[wasm_bindgen]
pub fn evaluate_vars(input: String, vars: Vec<EvaluateVarInput>) -> Option<EvaluateResponse> {

    let shared_vars: SharedVars = Rc::new(RefCell::new(
        HashMap::new()
    ));

    for v in vars {
        shared_vars.borrow_mut().insert(v.var, v.value);
    }

    let equals_amt = input
        .chars()
        .filter(|x| *x == '=')
        .count();

    if equals_amt == 0 {
        return Some(EvaluateResponse {
            value: evaluate_string(input, shared_vars.clone())?,
            var: None
        });
    }

    if equals_amt != 1 {
        return None;
    }

    let mut parts: Vec<String> = input
        .split("=")
        .map(|x| x.to_string())
        .collect();

    parts[0] = parts[0].trim().to_string();

    if parts[0].len() != 1 {
        return None;
    }

    Some(EvaluateResponse {
        value: evaluate_string(input, shared_vars.clone())?,
        var: Some(parts[0].chars().nth(0).unwrap())
    })

}

//#[wasm_bindgen]
pub fn evaluate_string(input: String, vars: SharedVars) -> Option<f64> {
    let lexed = match lex(&input) {
        Ok(o) => o,

        Err(_) => {
            return None
        }
    };

    match ParseTree::new(&lexed) {
        Ok(tree) => {
            match tree.evaluate(vars) {
                Ok(o) => Some(o),
                Err(_) => None
            }
        }

        Err(_) => None
    }
}
*/

#[wasm_bindgen]
pub struct Evaluator {
    vars: HashMap<char, f64>
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct EvaluatorResponse {
    value: f64,
    variable: Option<char>
}

fn evaluate_string_if_valid(inp: &str, vars: &HashMap<char, f64>) -> Option<f64> {
    let tokens = match lex(inp) {
        Ok(o) => o,

        Err(_) => {
            return None;
        }
    };

    let tree = match ParseTree::new(&tokens) {
        Ok(o) => o,

        Err(_) => {
            return None;
        }
    };

    // fix shared type
    match tree.evaluate(vars) {
        Ok(o) => Some(o),

        Err(_) => None
    }
}

#[wasm_bindgen]
impl Evaluator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Evaluator {
        Evaluator {
            vars: HashMap::new()
        }
    }

    pub fn evaluate(&mut self, input: String) -> Option<EvaluatorResponse> {
        let equals_count: usize = input.chars()
            .filter(|x| *x == '=')
            .count();

        println!("eq c: {}", equals_count);

        match equals_count {
            0 => {
                evaluate_string_if_valid(&input, &self.vars).map(|v| EvaluatorResponse {
                            value: v,
                            variable: None
                        })
            },

            1 => {
                let parts: Vec<&str> = input.split('=').collect();

                if parts[0].trim().len() != 1 {
                    return None;
                }

                let var: char = parts[0].trim().chars().next().unwrap();

                match evaluate_string_if_valid(parts[1], &self.vars) {
                    Some(v) => {

                        self.vars.insert(var, v);

                        Some(EvaluatorResponse {
                            value: v,
                            variable: Some(var)
                        })
                    },

                    None => None
                }
            }

            _ => None
        }
    }
}
