use super::{Determinant, Matrix, DeterNum, func};

#[derive(Clone, Debug)]
pub struct Vector {
    val: Vec<f64>
}

impl Vector {
    pub fn new(length: usize) -> Self {
        let mut buf = Vec::new();
        for _ in 0..length {
            buf.push(0.0)
        };
        Self {
            val: buf
        }
    }

    pub fn from<const L: usize>(val: [f64; L]) -> Self {
        Self { val: val.to_vec() }
    }

    pub fn from_i<const L: usize>(val: [isize; L]) -> Self {
        let mut buf = [0.0; L];
        for i in 0..L {
            buf[i] = val[i] as f64
        };
        Self::from(buf)
    }

    pub fn from_vec(val: Vec<f64>) -> Self {
        Self { val }
    }

    pub fn shape(&self) -> usize {
        self.val.len()
    }

    pub fn same_shape(&self, rhs: &Vector) -> bool {
        self.shape() == rhs.shape()
    }

    pub fn oper_with(&self, rhs: Vector, op: &(dyn Fn(f64, f64) -> f64)) -> Vector {
        if ! self.same_shape(&rhs) { panic!("The shapes are not matched!") }
        let mut out = self.clone();
        for i in 0..out.shape() {
            out.val[i] = op(out.val[i], rhs.val[i])
        };
        out
    }

    pub fn oper(&self, op: &(dyn Fn(f64) -> f64)) -> Vector {
        let mut out = self.clone();
        for i in 0..out.shape() {
            out.val[i] = op(out.val[i])
        };
        out
    }

    pub fn oper_assign(&mut self, op: &(dyn Fn(f64) -> f64)) {
        for i in self.val.iter_mut() {
            *i = op(*i)
        }
    }

    pub fn oper_with_assign(&mut self, rhs: Vector, op: &(dyn Fn(f64, f64) -> f64)) {
        if ! self.same_shape(&rhs) { panic!("The shapes are not matched!") }
        for i in 0..self.shape() {
            self.val[i] = op(self.val[i], rhs.val[i])
        };
    }

    pub fn strech(&self, times: f64) -> Self {
        self.oper(&|i| { i * times })
    }

    pub fn strech_assign(&mut self, times: f64) {
        self.oper_assign(&|i| { i * times })
    }

    pub fn slice(&self, place: usize) -> Self {
        Self {
            val: self.val[0..place].to_vec()
        }
    }

    pub fn sum(&self) -> f64 {
        let mut out = 0.0;
        for i in self.val.iter() {
            out += i
        };
        out
    }

    pub fn out_product(&self, rhs: Vector) -> Matrix {
        let mut out = Matrix::new((rhs.shape(), self.shape()));
        for i in 0..self.shape() {
            for j in 0..rhs.shape() {
                out.change_place((j, i), self.val[i] * rhs.val[j])
            }
        }
        out
    }

    pub fn get_mut_val(&mut self) -> &mut Vec<f64> {
        &mut self.val
    }

    pub fn get_val(&self) -> &Vec<f64> {
        &self.val
    }

    pub fn cross_product(&self, rhs: Vector) -> Vector {
        if ! self.same_shape(&rhs) { panic!("Mismatched shapes!") }

        let length = self.val.len();
        let mut buf = Vec::new();
        for _ in 0..length {
            let mut buf_1 = Vec::new();
            for _ in 0..length {
                buf_1.push(1.0)
            };
            buf.push(buf_1)
        };
        let mut deter = Determinant::from_vec(buf);

        for i in 0..length {
            let mut buf = Vec::new();
            for j in 0..length {
                if j == i {
                    buf.push(1.0)
                } else {
                    buf.push(0.0)
                }
            }
            deter.change_place_vector(i, 0, Self::from_vec(buf))
        };

        for i in 0..length {
            deter.change_place_float(i, 1, self.val[i]);
            deter.change_place_float(i, 2, rhs.val[i])
        };
    
        match deter.cal() {
            DeterNum::Float(_) => panic!("Shouldn't be here!"),
            DeterNum::Vec(vec) => vec
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.val.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64> {
        self.val.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.val.len()
    }

    pub fn max(&self) -> f64 {
        let mut max = f64::MIN;
        for i in self.val.iter() {
            if i > &max {
                max = *i
            }
        };
        max
    }

    pub fn min(&self) -> f64 {
        let mut min = f64::MAX;
        for i in self.val.iter() {
            if i < &min {
                min = *i
            }
        };
        min
    }
}

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i,j| { i + j })
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i,j| { i - j })
    }
}

impl std::ops::Mul for Vector {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i,j| { i * j }).sum()
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.oper_with_assign(rhs, &|i ,j| { i + j })
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.oper_with_assign(rhs, &|i ,j| { i - j })
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.oper(&|i| { -i })
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.val.iter().map(|a| {format!("{}", a)}).collect::<Vec<_>>().join(", "))
    }
}

impl func::Relu for Vector {
    fn relu_for(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.relu_back()
        };
        out
    }

    fn relu_back(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.relu_back()
        };
        out
    }
}

impl func::Sigmoid for Vector {
    fn sig_for(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.sig_for()
        };
        out
    }
    
    fn sig_back(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.sig_back()
        };
        out
    }
}

impl func::Tanh for Vector {
    fn tanh_for(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.tanh_for()
        };
        out
    }

    fn tanh_back(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.tanh_back()
        };
        out
    }
}

impl func::Softmax for Vector {
    fn soft_for(&self) -> Self {
        let buf = self.oper(&| i | { i.exp() });
        let sum = buf.sum();
        buf.oper(&| i | { i/sum })
    }
}