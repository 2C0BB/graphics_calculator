use std::collections::hash_map::HashMap;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use regex::Regex;

mod utils;

mod calculus;
use calculus::*;

pub mod roots;
use roots::*;

#[wasm_bindgen]
pub fn setup() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
    X,
    Var(char),
    Num(f64),
    IndefiniteFunction(char)
}

fn value_operator(input: &LexerTokenType) -> u32 {
    match input {
        LexerTokenType::Func(..) => 0,
        LexerTokenType::Mul => 1,
        LexerTokenType::Div => 1,
        LexerTokenType::Add => 2,
        LexerTokenType::Sub => 2,

        LexerTokenType::Num(_) | LexerTokenType::Var(_) | LexerTokenType::X | LexerTokenType::IndefiniteFunction(_) => panic!()
    }
}


#[derive(Clone, Debug)]
pub struct LexerToken {
    token_type: LexerTokenType,
    bracket_depth: u32
}

const FUNCTIONS: [&str; 7] = ["ln", "log", "sin", "cos", "tan", "sqrt", "int"];

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

    let mut bracket_depth: u32 = 1;

    let mut args_sets: Vec<Vec<char>> = Vec::new();
    let mut current_set: Vec<char> = Vec::new();

    let mut closing_found: bool = false;

    for c in i {

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

    // this is a bad solution, but integration will replace the first function with the relevant tree

    // TODO: check argument amount in lexer instead of evaluator

    let int_function_name: Option<char>;
    if function_name == "int" {
        // regex for function of x

        if args_sets.len() != 3 {
            return Err(LexError);
        }

        let first_argument: String = args_sets[0]
            .iter()
            .collect();

        let re = Regex::new(r"^[a-zA-Z]\(x\)$").unwrap();
        if re.is_match(&first_argument) {

            let first_char = match first_argument.chars().next() {
                Some(c) => c,
                None => {
                    return Err(LexError);
                }
            };

            int_function_name = Some(first_char);
            args_sets.remove(0);
        } else {
            int_function_name = None;
        }
    } else {
        int_function_name = None;
    }

    println!("{:?}", int_function_name);

    let parsed_sets: Vec<Result<Vec<LexerToken>, LexError>> = args_sets.into_iter()
        .map(|set| set.iter().collect::<String>())
        .map(|set_string| {println!("SET: {}", set_string); lex(&set_string)})
        .collect();

    if parsed_sets.iter().any(|x| x.is_err()) {
        return Err(LexError);
    }

    let mut parsed_sets: Vec<Vec<LexerToken>> = parsed_sets.into_iter()
        .map(|x| x.unwrap())
        .collect();

    if let Some(fn_name) = int_function_name {
        parsed_sets.insert(0, vec![
            LexerToken {
                token_type: LexerTokenType::IndefiniteFunction(fn_name),
                bracket_depth
            }
        ]);
    }

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


        if character.is_numeric() || character == '.' {
            let mut num_buf: Vec<char> = vec![character];
            
            while let Some(c) = iter.peek() {
                if !(c.is_numeric() || *c == '.') {
                    break;
                }

                num_buf.push(*c);
                iter.next();
            }

            if num_buf[0] == '.' {
                num_buf.insert(0, '0');
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

    out = out.into_iter()
        .map(|mut t| {
            if let LexerTokenType::Var('x') = t.token_type {
                t.token_type = LexerTokenType::X;
            }

            t
        })
        .collect();

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
            LexerTokenType::Num(_) | LexerTokenType::Var(_) | LexerTokenType::X => {
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

#[derive(Debug, Clone)]
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
    fn new_from_tokens(items: &[LexerToken], graphs: &HashMap<char, ParseTree>) -> Result<TreeNode, ParseError>  {
        let next_op_pos = find_next_op(items);

        match next_op_pos {
            Some(pos) => {
                let mut token_type = items[pos].token_type.clone();
                if let LexerTokenType::Func(vars, _name) = &mut token_type {
                    assert_eq!(items.len(), 1);

                    let int_fn_node: Option<TreeNode>;

                    if vars[0].len() != 1 {
                        return Err(ParseError);
                    }

                    if let LexerTokenType::IndefiniteFunction(fn_int_name) = vars[0][0].token_type {
                        int_fn_node = match graphs.get(&fn_int_name) {
                            Some(int_fn) => Some(*int_fn.inner_tree.clone().unwrap()),
                            
                            None => {
                                return Err(ParseError);
                            }
                        };

                        vars.remove(0);
                    } else {
                        int_fn_node = None;
                    }

                    let function_args: Vec<Result<TreeNode, ParseError>> = vars.iter()
                        .map(|x| TreeNode::new_from_tokens(x, graphs))
                        .collect();

                    if function_args.iter().any(|x| x.is_err()) {
                        return Err(ParseError);
                    }

                    let mut function_args: Vec<TreeNode> = function_args.into_iter()
                        .map(|x| x.unwrap())
                        .collect();

                    if let Some(inner_int_node) = int_fn_node {
                        function_args.insert(0, inner_int_node);
                    }

                    return Ok(TreeNode {
                        token_type: token_type.clone(),
                        function_args,

                        left: None,
                        right: None,
                    });
                }
                
                let left_items = &items[0..pos];
                let left_node = TreeNode::new_from_tokens(left_items, graphs)?;

                let right_items = &items[pos+1..];
                let right_node = TreeNode::new_from_tokens(right_items, graphs)?;

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

    fn evaluate(&self, x: Option<f64>, vars: &HashMap<char, f64>) -> Result<f64, EvaluateError> {
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

                return Ok(self.function_args[0].evaluate(x, vars)?.ln());
            }

            if name == "log" {
                assert!(self.function_args.len() == 1 || self.function_args.len() == 2);
                
                let base = match self.function_args.len() {
                    1 => 10.0,
                    2 => self.function_args[1].evaluate(x, vars)?,

                    _ => unreachable!()
                };

                return Ok(self.function_args[0].evaluate(x, vars)?.log(base));
            }

            if name == "sin" {
                assert_eq!(self.function_args.len(), 1);

                return Ok(self.function_args[0].evaluate(x, vars)?.sin());
            }
            if name == "cos" {
                assert_eq!(self.function_args.len(), 1);

                return Ok(self.function_args[0].evaluate(x, vars)?.cos());
            }
            if name == "tan" {
                assert_eq!(self.function_args.len(), 1);

                return Ok(self.function_args[0].evaluate(x, vars)?.tan());
            }
            if name == "sqrt" {
                assert_eq!(self.function_args.len(), 1);

                return Ok(self.function_args[0].evaluate(x, vars)?.sqrt());
            }

            if name == "int" {
                assert_eq!(self.function_args.len(), 3);

                let f = |x: f64| self.function_args[0].evaluate(Some(x), vars).unwrap();

                return Ok(integrate(f, 
                        self.function_args[1].evaluate(x, vars)?, 
                        self.function_args[2].evaluate(x, vars)?, 
                        10000
                ));
            }

            unimplemented!()
        }

        if let LexerTokenType::X = self.token_type {
            return match x {
                Some(v) => Ok(v),
                None => Err(EvaluateError)
            };
        }

        // TODO: remove unwraps if necessary
        let left_val: f64 = self.left.as_ref().unwrap().evaluate(x, vars)?;
        let right_val: f64 = self.right.as_ref().unwrap().evaluate(x, vars)?;

        Ok(match &self.token_type {
            LexerTokenType::Add => left_val + right_val,
            LexerTokenType::Sub => left_val - right_val,
            LexerTokenType::Mul => left_val * right_val,
            LexerTokenType::Div => left_val / right_val,

            LexerTokenType::Num(_) | LexerTokenType::Var(_) | LexerTokenType::Func(..) | LexerTokenType::X | LexerTokenType::IndefiniteFunction(_) => unreachable!()
        })
    }

}

#[derive(Debug, Clone)]
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
    pub fn new(lexed: &[LexerToken], graphs: &HashMap<char, ParseTree>) -> Result<ParseTree, ParseError> {
        let inner_tree = Some(Box::new(
            TreeNode::new_from_tokens(lexed, graphs)?
        ));

        Ok(ParseTree { inner_tree })
    }

    pub fn evaluate(&self, x: Option<f64>, vars: &HashMap<char, f64>) -> Result<f64, EvaluateError> {
        if let Some(tree) = &self.inner_tree {
            tree.evaluate(x, vars)
        } else {
            panic!()
        }
    }
}

#[wasm_bindgen]
pub struct Evaluator {
    vars: HashMap<char, f64>,
    graphs: HashMap<char, ParseTree>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum EvaluatorResponse {
    Value {
        value: f64,
        var_name: Option<String>
    },

    Graph {
        points: Vec<[f64; 2]>,
        //graph_name don't know if this is necessary yet, may be used for 
    }
}

fn evaluate_value_if_valid(
    input: &str,
    vars: &HashMap<char, f64>,
    graphs: &HashMap<char, ParseTree>
) -> Option<f64> {

    let tokens = match lex(input) {
        Ok(v) => v,

        Err(_) => {
            return None;
        }
    };

    let tree = match ParseTree::new(&tokens, graphs) {
        Ok(v) => v,

        Err(_) => {
            return None;
        }
    };

    let value = match tree.evaluate(None, vars) {
        Ok(v) => v,

        Err(_) => {
            return None;
        }
    };

    Some(value)

}

/*
struct FunctionDef {
    name: char,
    differentiates: u32,
    inside: String,
}

impl FunctionDef {
    /*
    fn new(s: &'a str) -> Self {
        let name = s.chars().nth(0).unwrap();
        let differentiates = s.chars()
            .filter(|x| *x == '\'')
            .count() as u32;
        
        let inside_start = s.find('(').unwrap() + 1;
        let inside_end = s.find(')').unwrap();

        let inside = &s[inside_start..inside_end];

        FunctionDef { name, differentiates, inside }
    }
    */

    fn new(s: &str) {
        let mut characters= s.chars().peekable();

        let name: char = characters.next().unwrap();

        let mut differentiates: usize = 0;
        let mut inner: Vec<char> = Vec::new();

        while let Some('\'') = characters.peek() {
            differentiates += 1;
            characters.next();
        }

        characters.next();

        let bracket_depth: u32 = 1;

        //for c in 
    }
}
*/

#[wasm_bindgen]
impl Evaluator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Evaluator {
        Evaluator {
            vars: HashMap::new(),
            graphs: HashMap::new(),
        }
    }

    pub fn find_intercepts(&self, fn1_name: char, fn2_name: char) -> Option<Vec<f64>> {
        let fn1 = match self.graphs.get(&fn1_name) {
            Some(v) => v,
            None => {
                return None
            }
        };

        let fn2 = match self.graphs.get(&fn2_name) {
            Some(v) => v,
            None => {
                return None
            }
        };

        let f = |x: f64| 
            fn1.evaluate(Some(x), &self.vars).unwrap() - fn2.evaluate(Some(x), &self.vars).unwrap();

        Some(find_roots(f, 0.0, 20.0, 0.001, 0.001))
    }

    pub fn evaluate(&mut self, input: String) -> JsValue {
        let equals_count: usize = input.chars()
            .filter(|x| *x == '=')
            .count();

        match equals_count {
            0 => {

                let re = Regex::new(r"^[a-zA-Z][']+\(x\)$").unwrap();

                if re.is_match(&input) {

                    let fn_name = input.chars().nth(0).unwrap();
                    let differentiation_count = input.chars()
                        .filter(|c| *c == '\'')
                        .count();

                    let tree = match self.graphs.get(&fn_name) {
                        Some(v) => v,
                        None => {
                            return JsValue::NULL;
                        }
                    };

                    let mut points: Vec<[f64; 2]> = Vec::new();
                    let mut x: f64 = -10.0;


                    // TODO : TRY USING RC REFCELL TO AVOID CLONES
                    // this is done as differentiate requires static closure
                    let cloned_tree = tree.clone();
                    let cloned_vars = self.vars.clone();

                    // TODO : FIX THIS SO IT DOESN'T CRASH IF f(x) FAILS TO EVALUATE
                    let f = move |x: f64| cloned_tree.evaluate(Some(x), &cloned_vars).unwrap();
                    let f_prime = differentiate(f, differentiation_count);

                    while x <= 10.0 {
                        let y = f_prime(x);
                        points.push([x, y]);

                        x += 0.1;
                    }

                    return serde_wasm_bindgen::to_value(&
                        EvaluatorResponse::Graph { points }
                    ).unwrap();
                }

                let value = match evaluate_value_if_valid(&input, &self.vars, &self.graphs) {
                    Some(v) => v,
                    None => {
                        return JsValue::NULL;
                    }
                };

                serde_wasm_bindgen::to_value(
                    &EvaluatorResponse::Value {
                        value,
                        var_name: None
                    }
                )
                    .expect("failed to serialize")
                
            },

            1 => {
                // [ function def, function ]
                let parts: Vec<String> = input.split('=')
                    .map(|x| x.chars().filter(|y| *y != ' ').collect())
                    .collect();

                let fn_re = Regex::new(r"^[a-zA-Z]\(x\)$")
                    .expect("regex failed");

                if fn_re.is_match(&parts[0]) {
                    let fn_name = parts[0]
                        .chars()
                        .nth(0)
                        .unwrap();

                    let tokens = match lex(&parts[1]) {
                        Ok(v) => v,
                        Err(_) => {
                            return JsValue::NULL;
                        }
                    };

                    let tree = match ParseTree::new(&tokens, &self.graphs) {
                        Ok(v) => v,
                        Err(_) => {
                            return JsValue::NULL;
                        }
                    };

                    let mut points: Vec<[f64; 2]> = Vec::new();

                    let mut x: f64 = -10.0;

                    while x <= 10.0 {
                        let y = match tree.evaluate(Some(x), &self.vars) {
                            Ok(v) => v,
                            Err(_) => {
                                return JsValue::NULL;
                            }
                        };

                        points.push([x, y]);

                        x += 0.1;
                    }

                    self.graphs.insert(fn_name, tree);

                    serde_wasm_bindgen::to_value(
                        &EvaluatorResponse::Graph { points }
                    ).unwrap()

                } else {
                    let var_name = parts[0]
                        .chars()
                        .nth(0)
                        .unwrap();

                    let value = match evaluate_value_if_valid(&parts[1], &self.vars, &self.graphs) {
                        Some(v) => v,
                        None => {
                            return JsValue::NULL;
                        }
                    };

                    self.vars.insert(var_name, value);

                    serde_wasm_bindgen::to_value(
                        &EvaluatorResponse::Value {
                            value,
                            var_name: Some(var_name.to_string())
                        }
                    ).unwrap()
                }
            }

            _ => JsValue::NULL
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Evaluator::new()
    }
}
