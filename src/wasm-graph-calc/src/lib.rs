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
    Var(char),
    Num(f32)
}

fn value_operator(input: &LexerTokenType) -> u32 {
    match input {
        LexerTokenType::Mul => 0,
        LexerTokenType::Div => 0,
        LexerTokenType::Add => 1,
        LexerTokenType::Sub => 1,

        LexerTokenType::Num(_) | LexerTokenType::Var(_) => panic!()
    }
}


#[derive(Debug)]
pub struct LexerToken {
    token_type: LexerTokenType,
    bracket_depth: u32
}

struct LexerBuffer {
	chars: Vec<char>,
}

impl LexerBuffer {
	fn new() -> Self {
		LexerBuffer { chars: Vec::new() }
	}

    fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

	fn convert_string(&mut self) -> Option<String> {

        if self.chars.is_empty() {
            return None
        }

		let out: String = self.chars
            .iter()
            .collect();

        self.chars = Vec::new();

        Some(out)
	}

    fn convert_num(&mut self) -> Option<f32> {

        match self.convert_string() {
            Some(s) => {
                Some(s.parse().expect("not a num"))
            },

            None => {
                None
            }
        }
    }
}

pub fn lex(input: &str) -> Vec<LexerToken> {
    let mut iter = input.chars().peekable();

    let mut out: Vec<LexerToken> = Vec::new();

    while let Some(c) = iter.next() {
        if c == ' ' {
            continue;
        }

        if c.is_numeric() {
            let mut num_buf: Vec<char> = vec![c];
            
            while let Some(x) = iter.peek() {
                if x.is_numeric() {
                    num_buf.push(*x);
                } else {
                    break;
                }
            }
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

/*impl std::cmp::PartialOrd for OperatorOrdering {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ordering1 = self.bracket_depth.cmp(&other.bracket_depth);

        match ordering1 {
            std::cmp::Ordering::Equal => {},
            _ => {
                return Some(ordering1);
            }
        }

        let ordering2 = self.operator_val.cmp(&other.operator_val);

        match ordering2 {
            std::cmp::Ordering::Equal => {},
            _ => {
                return Some(ordering2);
            }
        }

        Some(self.position.cmp(&other.position))
    } 
}*/

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

    left: TreeLink,
    right: TreeLink
}

impl TreeNode {
    fn new_from_tokens(items: &[LexerToken]) -> Result<TreeNode, ParseError>  {
        let next_op_pos = find_next_op(items);

        match next_op_pos {
            Some(pos) => {
                let token_type = items[pos].token_type.clone();
                
                let left_items = &items[0..pos];
                let left_node = TreeNode::new_from_tokens(left_items)?;

                let right_items = &items[pos+1..];
                let right_node = TreeNode::new_from_tokens(right_items)?;


                Ok(TreeNode { 
                    token_type,
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

        let left_val = self.left.as_ref().unwrap().evaluate(vars.clone());
        let right_val = self.right.as_ref().unwrap().evaluate(vars.clone());

        match self.token_type {
            LexerTokenType::Add => left_val + right_val,
            LexerTokenType::Sub => left_val - right_val,
            LexerTokenType::Mul => left_val * right_val,
            LexerTokenType::Div => left_val / right_val,

            LexerTokenType::Num(_) | LexerTokenType::Var(_) => unreachable!()
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
