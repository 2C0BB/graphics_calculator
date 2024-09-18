pub fn find_roots<F>(f: F, start: f64, stop: f64, step: f64, epsilon: f64) -> Vec<f64> 
where
    F: Fn(f64) -> f64
{
    let mut ret = vec![];
    let mut current = start;
    while current < stop {
        if f(current).abs() < epsilon {
            ret.push(current);
        }
        current += step;
    }
    ret
}
