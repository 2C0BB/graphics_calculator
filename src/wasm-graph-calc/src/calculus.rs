use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn differentiate<F>(f: F, amount: usize) -> Box<dyn Fn(f64) -> f64>
    where F: Fn(f64) -> f64 + 'static
{
    let mut out: Box<dyn Fn(f64) -> f64> = Box::new(f);

    let h = 0.0001;
    for _ in 0..amount {
        out = Box::new(move |x: f64| (out(x+h) - out(x)) / h);
    }
    
    out
}

pub fn integrate<F>(f: F, start: f64, end: f64, squares_amt: usize) -> f64
    where F: Fn(f64) -> f64
{

    let square_width: f64 = (end - start) / squares_amt as f64;

    let left_estimate: f64 = (0..=squares_amt-1)
        .map(|x| f(start + x as f64 * square_width))
        .sum::<f64>() * square_width;

    let right_estimate: f64 = (1..=squares_amt)
        .map(|x| f(start + x as f64 * square_width))
        .sum::<f64>() * square_width;

    (left_estimate + right_estimate) / 2.0
}
