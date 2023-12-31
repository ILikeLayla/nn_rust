use crate::func::Tanh;

use super::{Determinant, Vector, DeterNum, func};

#[derive(Clone)]
pub struct Matrix {
    vec: Vec<Vector>,
    shape: (usize, usize)
}

impl Matrix {
    pub fn new(shape: (usize, usize)) -> Self {
        let mut buf = Vec::new();
        for _ in 0..shape.1 {
            buf.push(Vector::new(shape.0))
        }
        Self {
            vec: buf,
            shape
        }
    }

    pub fn from<const X: usize, const Y: usize>(val: [[f64; X]; Y]) -> Self {
        let mut buf = Vec::new();
        for i in 0..Y {
            buf.push(Vector::from(val[i]))
        }
        Self { vec: buf, shape: (X, Y) }
    }

    pub fn from_i<const X: usize, const Y: usize>(val: [[isize; X]; Y]) -> Self {
        let mut buf = Vec::new();
        for i in 0..Y {
            buf.push(Vector::from_i(val[i]))
        }
        Self { vec: buf, shape: (X, Y) }
    }

    pub fn from_vec(val: Vec<Vec<f64>>) -> Self {
        let mut buf = Vec::new();
        let standard_length = val[0].len();
        for i in val.iter() {
            if i.len() == standard_length {
                buf.push(Vector::from_vec(i.to_vec()))
            } else {
                panic!("Mismatched shapes!")
            }
        };

        Self { vec: buf, shape: (standard_length, val.len()) }
    }

    pub fn from_ver_vec(val: Vec<Vec<f64>>) -> Self {
        Self::from_vec(val).t()
    }

    pub fn from_ver_vec_i(val: Vec<Vec<i64>>) -> Self {
        let mut buf = Vec::new();
        for i in val.iter() {
            let mut buf_1 = Vec::new();
            for j in i.iter() {
                buf_1.push( *j as f64 )
            };
            buf.push(buf_1)
        };
        Self::from_ver_vec(buf)
    }

    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn same_shape(&self, rhs: &Matrix) -> bool {
        self.shape == rhs.shape
    }

    pub fn oper(&self, op: &(dyn Fn(f64) -> f64)) -> Self {
        let mut out = self.clone();
        for i in out.vec.iter_mut() {
            *i = i.oper(op)
        }
        out
    }

    pub fn oper_assign(&mut self, op: &(dyn Fn(f64) -> f64)) {
        for i in self.vec.iter_mut() {
            i.oper_assign(op)
        }
    }

    pub fn oper_with(&self, rhs: Matrix, op: &(dyn Fn(f64, f64)-> f64)) -> Self {
        if ! self.same_shape(&rhs) { panic!("The shapes are nut matched!") }
        let mut out = self.clone();
        for i in 0..self.shape.1 {
            out.vec[i].oper_with_assign(rhs.vec[i].clone(), op);
        };
        out
    }

    pub fn oper_with_assign(&mut self, rhs: Matrix, op: &(dyn Fn(f64, f64)-> f64)) {
        if ! self.same_shape(&rhs) { panic!("The shapes are nut matched!") }
        for i in 0..self.shape.1 {
            self.vec[i].oper_with_assign(rhs.vec[i].clone(), op);
        };
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Vector> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Vector> {
        self.vec.iter_mut()
    }

    pub fn change_place(&mut self, place: (usize, usize), val: f64) {
        self.vec[place.1].get_mut_val()[place.0] = val
    }

    pub fn change_row<const X: usize>(&mut self, y: usize, val: [f64; X]) {
        if ! X == self.shape.0 { panic!("The shapes are not matched!") }
        self.vec[y] = Vector::from_vec(val.to_vec());
    }

    pub fn change_col<const Y: usize>(&mut self, x: usize, val: [f64; Y]) {
        if ! Y == self.shape.1 { panic!("The shapes are not matched!") }
        for i in 0..self.vec.len() {
            self.vec[i].get_mut_val()[x] = val[i]
        }
    }

    pub fn slice(&self, from: usize, to:usize) -> Self {
        if from >= to { panic!("The range is unaccpetable!") }
        Self { vec: self.vec[from..to].to_vec(), shape: (self.shape.0, to - from) }
    }

    pub fn sum(&self) -> (Vec<f64>, Vec<f64>) {
        let mut row_sum = Vec::new();
        let mut col_sum = Vec::new();

        for i in 0..self.shape.1 {
            row_sum.push(self.vec[i].sum())
        };

        for i in 0..self.shape.0 {
            let mut buf = 0.0;
            for j in 0..self.shape.1 {
                buf += self.vec[j].get_val()[i]
            }
            col_sum.push(buf)
        }

        (row_sum, col_sum)

    }

    pub fn had_product(&self, rhs: Matrix) -> Self {
        self.oper_with(rhs, &|i, j| { i * j })
    }

    pub fn had_product_assign(&mut self, rhs: Matrix) {
        *self = self.had_product(rhs)
    }

    pub fn kron_product(&self, rhs: Matrix) -> Self {
        let mut out = Matrix::new((self.shape.0 * rhs.shape.0, self.shape.1 * self.shape.1));
        for i in 0..out.shape.0 {
            for j in 0..out.shape.1 {
                let a = (i / rhs.shape.0, j / rhs.shape.1);
                let b = (i % rhs.shape.0, j % rhs.shape.1);
                out.change_place((i, j), self.get(a.0, a.1) * rhs.get(b.0, b.1))
            };
        };
        out
    }

    pub fn kron_product_assign(&mut self, rhs: Matrix) {
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                let a = (i / rhs.shape.0, j / rhs.shape.1);
                let b = (i % rhs.shape.0, j % rhs.shape.1);
                self.change_place((i, j), self.get(a.0, a.1) * rhs.get(b.0, b.1))
            };
        };
    }

    pub fn strech(&self, times: f64) -> Self {
        self.oper(&|i| {i * times})
    }

    pub fn strech_assign(&mut self, times: f64) {
        self.oper_assign(&|i| {i * times})
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        self.vec[y].get_val()[x]
    }

    pub fn get_val(&self) -> Vec<Vec<f64>> {
        let mut out = Vec::new();
        for i in self.vec.iter() {
            out.push(i.get_val().clone())
        };
        out
    }

    pub fn min(&self) -> f64 {
        let mut min = f64::MAX;
        for i in self.vec.iter() {
            let min_buf = i.min();
            if min > min_buf {
                min = min_buf
            }
        };
        min
    }

    pub fn max(&self) -> f64 {
        let mut max = f64::MIN;
        for i in self.vec.iter() {
            let max_buf = i.max();
            if max < max_buf {
                max = max_buf
            }
        };
        max
    }

    pub fn get_determinant(&self) -> Determinant {
        Determinant::from_matrix(self)
    }

    pub fn det(&self) -> f64 {
        let deter = self.get_determinant();
        match deter.cal() {
            DeterNum::Float(num) => num,
            DeterNum::Vec(_) => panic!("Shouldn't be here.")
        }
    }

    pub fn t(&self) -> Self {
        let mut buf = Vec::new();
        for i in 0..self.shape.0 {
            let mut buf_1 = Vec::new();
            for j in 0..self.shape.1 {
                buf_1.push(self.get(i, j))
            }
            buf.push(buf_1)
        };
        Matrix::from_vec(buf)
    }

    pub fn t_assign(&mut self) {
        *self = self.t()
    }

    pub fn exp(&self) -> Self {
        self.oper(&|i| {i.exp()})
    }

    pub fn restrict(&mut self, down: f64, up: f64) {
        if down >= up { panic!("Unaccpetable range!") }
        let max = self.max();
        let min = self.min();
        let m = (up - down) / (max - min);
        let c = max * m - up;
        self.oper_assign(&|x| { m * x - c})
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix[\n    {}\n]", self.vec.iter().map(|a| {format!("{}", a)}).collect::<Vec<_>>().join("\n    "))
    }
}

impl std::ops::Add for Matrix {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i, j| { i + j })
    }
}

impl std::ops::Sub for Matrix {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i, j| { i - j })
    }
}

impl std::ops::AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        self.oper_with_assign(rhs, &|i, j| { i + j })
    }
}

impl std::ops::SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        self.oper_with_assign(rhs, &|i, j| { i - j })
    }
}

impl std::ops::Mul for Matrix{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.shape.0 != rhs.shape.1 { panic!("Shape Mismatch") }
        let mut out = Matrix::new((rhs.shape.0, self.shape.1));
        for i in 0..self.shape.1 {
            for j in 0..rhs.shape.0 {
                let mut buf = 0.0;
                for k in 0..self.shape.0 {
                    buf += self.get(k, i) * rhs.get(j, k)
                };
                out.change_place((j, i), buf)
            }
        }
        out
    }
}

impl std::ops::MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs
    }
}

impl std::ops::Div for Matrix {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i ,j| { i/j })
    }
}

impl std::ops::DivAssign for Matrix {
    fn div_assign(&mut self, rhs: Self) {
        self.oper_with_assign(rhs, &|i ,j| { i/j })
    }
}

impl func::Softmax for Matrix {
    fn soft_for(&self) -> Self {
        let mut out = Matrix::new(self.shape);
        let exp = self.exp();

        let mut exp_sum = Vec::new();
        for i in 0..self.vec[0].len() {
            let mut sum = 0.0;
            for j in 0..self.vec.len() {
                sum += exp.get(i, j)
            };
            exp_sum.push(sum)
        };

        for i in 0..self.vec.len() {
            for j in 0..self.vec[0].len() {
                let val = exp.get(j, i) / exp_sum[j];
                out.change_place((j, i), val)
            }
        };
        out
    }
}

impl func::Sigmoid for Matrix {
    fn sig_for(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.clone().sig_for()
        };
        out
    }

    fn sig_back(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.clone().sig_back()
        };
        out
    }
}

impl func::Relu for Matrix {
    fn relu_for(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.clone().relu_for()
        };
        out
    }

    fn relu_back(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.clone().relu_back()
        };
        out
    }
}

impl func::Tanh for Matrix {
    fn tanh_for(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.clone().tanh_for();
        };
        out
    }

    fn tanh_back(&self) -> Self {
        let mut out = self.clone();
        for i in out.iter_mut() {
            *i = i.clone().tanh_back()
        };
        out
    }
}