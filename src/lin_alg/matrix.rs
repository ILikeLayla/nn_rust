use super::{Determinant, Vector};

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

    pub fn oper_with(&self, rhs: Matrix, op: &(dyn Fn(f64, f64)-> f64)) -> Self {
        if ! self.same_shape(&rhs) { panic!("The shapes are nut matched!") }
        let mut out = self.clone();
        for i in 0..self.shape.1 {
            out.vec[i] = out.vec[i].oper_with(rhs.vec[i].clone(), op);
        };
        out
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

    pub fn slice<const X: usize, const Y: usize>(&self) -> [[f64; X]; Y] {
        let mut buf = [[0.0; X]; Y];
        for i in 0..self.shape.1 {
            buf[i] = self.vec[i].slice()
        }
        buf
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
        *self = self.kron_product(rhs)
    }

    pub fn strech(&self, times: f64) -> Self {
        self.oper(&|i| {i * times})
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        self.vec[y].get_val()[x]
    }

    pub fn get_val(&self) -> Vec<Vec<f64>> {
        let mut out = Vec::new();
        for i in self.vec.iter() {
            out.push(i.get_val())
        };
        out
    }

    pub fn get_determinant(&self) -> Determinant {
        let mut out = Determinant::
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
        *self = self.oper_with(rhs, &|i, j| { i + j })
    }
}

impl std::ops::SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.oper_with(rhs, &|i, j| { i - j })
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
                out.change_place((i, j), buf)
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