pub trait Sigmoid {
    fn sig_for(&self) -> Self;
    fn sig_back(&self) -> Self;
}

pub trait Tanh {
    fn tanh_for(&self) -> Self;
    fn tanh_back(&self) -> Self;
}

pub trait Relu {
    fn relu_for(&self) -> Self;
    fn relu_back(&self) -> Self;
}

pub trait Softmax {
    fn soft_for(&self) -> Self;
}

impl Sigmoid for f64 {
    fn sig_back(&self) -> Self {
        let buf = self.sig_for();
        return buf * (1.0 - buf)
    }

    fn sig_for(&self) -> Self {
        return 1.0 / (1.0 + (-self).exp())
    }
}

impl Relu for f64 {
    fn relu_for(&self) -> Self {
        return if self > &0.0 { *self } else { 0.0 }
    }

    fn relu_back(&self) -> Self {
        return if self > &0.0 { 1.0 } else { 0.0 }
    }
}

impl Tanh for f64 {
    fn tanh_for(&self) -> Self {
        let exp = self.exp();
        let exp_ne = (-self).exp();
        return (exp + exp_ne) / (exp - exp_ne)
    }

    fn tanh_back(&self) -> Self {
        let buf = self.tanh_for();
        return 1.0 - buf * buf
    }
}