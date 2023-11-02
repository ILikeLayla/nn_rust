#[derive(Clone, Copy)]
pub struct Vector<const L: usize> {
    val: [f64; L]
}

impl<const L: usize> Vector<L> {
    pub fn new() -> Self {
        Self {
            val: [0.0; L]
        }
    }

    pub fn from(val: [f64; L]) -> Self {
        Self { val }
    }

    pub fn shape(&self) -> usize {
        self.val.len()
    }

    pub fn oper_with(&self, rhs: Vector<L>, op: &(dyn Fn(f64, f64) -> f64)) -> Vector<L> {
        let mut out = self.clone();
        for i in 0..out.shape() {
            out.val[i] = op(out.val[i], rhs.val[i])
        };
        out
    }

    pub fn oper(&self, op: &(dyn Fn(f64) -> f64)) -> Vector<L> {
        let mut out = self.clone();
        for i in 0..out.shape() {
            out.val[i] = op(out.val[i])
        };
        out
    }
}

impl<const L: usize> std::ops::Add for Vector<L> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i,j| { i + j })
    }
}

impl<const L: usize> std::ops::Sub for Vector<L> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i,j| { i - j })
    }
}

impl<const L: usize> std::ops::Mul for Vector<L> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i,j| { i * j })
    }
}

impl<const L: usize> std::ops::Div for Vector<L> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i,j| { i / j })
    }
}

impl<const L: usize> std::ops::AddAssign for Vector<L> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.oper_with(rhs, &|i ,j| { i + j })
    }
}

impl<const L: usize> std::ops::SubAssign for Vector<L> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.oper_with(rhs, &|i ,j| { i - j })
    }
}

impl<const L: usize> std::ops::MulAssign for Vector<L> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.oper_with(rhs, &|i ,j| { i * j })
    }
}

impl<const L: usize> std::ops::DivAssign for Vector<L> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.oper_with(rhs, &|i ,j| { i / j })
    }
}

impl<const L: usize> std::fmt::Display for Vector<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector<{}>", self.val.iter().map(|a| {format!("{}", a)}).collect::<Vec<_>>().join(", "))
    }
}

#[derive(Clone, Copy)]
pub struct Matrix<const X: usize, const Y: usize> {
    vec: [Vector<X>; Y],
}

impl<const X: usize, const Y: usize> Matrix<X, Y> {
    pub fn new() -> Self {
        Self {
            vec: [Vector::new(); Y]
        }
    }

    pub fn from(val: [[f64; X]; Y]) -> Self {
        let mut buf = [Vector::<X>::new(); Y];
        for i in 0..Y {
            buf[i] = Vector::from(val[i])
        }
        Self { vec: buf }
    }

    pub fn shape(&self) -> (usize, usize) {
        (X, Y)
    }

    pub fn oper(&self, op: &(dyn Fn(usize) -> usize)) -> Self {
        let mut out = self.clone();
        for i in out.vec.iter_mut() {
            
        }
        out
    }

    pub fn change_place(&mut self, x: usize, y: usize, val: f64) {
        self.vec[y].val[x] = val
    }

    pub fn change_row(&mut self, y: usize, val: [f64; X]) {
        self.vec[y] = Vector { val: val };
    }

    pub fn change_col(&mut self, x: usize, val: [f64; Y]) {
        for i in 0..self.vec.len() {
            self.vec[i].val[x] = val[i]
        }
    }
}

impl<const X: usize, const Y: usize> std::fmt::Display for Matrix<X,Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix<\n    {}\n>", self.vec.iter().map(|a| {format!("{}", a)}).collect::<Vec<_>>().join("\n    "))
    }
}