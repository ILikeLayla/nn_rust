pub fn relu(a: f64) -> f64 {
    return if a > 0.0 { a } else { 0.0 }
}

pub fn sigmoid(a: f64) -> f64 {
    return 1.0 / (1.0 + (-a).exp())
}

pub fn tanh(a: f64) -> f64 {
    let exp = a.exp();
    let exp_ne = (-a).exp();
    return (exp + exp_ne) / (exp - exp_ne)
}

pub fn relu_diff(a: f64) -> f64 {
    return if a > 0.0 { 1.0 } else { 0.0 }
}

pub fn sigmoid_diff(a: f64) -> f64 {
    let buf = sigmoid(a);
    return buf * (1.0 - buf)
}

pub fn tanh_diff(a: f64) -> f64 {
    let buf = tanh(a);
    return 1.0 - buf * buf
}