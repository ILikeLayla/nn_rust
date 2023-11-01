#[derive(Clone)]
pub struct Vector {
    val: Vec<f64>
}

impl Vector {
    pub fn new(len: usize) -> Self {
        let mut buf = Vec::new();
        for _ in 0..len {
            buf.push(0.0)
        };
        Self {
            val: buf
        }
    }

    pub fn shape(&self) -> usize {
        self.val.len()
    }

    pub fn eqal_shape(&self, other: &Vector) -> bool {
        self.shape() == other.shape()
    }

    pub fn oper_with(&self, rhs: Vector, op: &(dyn Fn(f64, f64) -> f64)) -> Vector {
        if ! self.eqal_shape(&rhs) {panic!("They don't have the same shape!")}
        let mut out = self.clone();
        for i in 0..out.shape() {
            out.val[i] = op(out.val[i], rhs.val[i])
        };
        out
    }

    pub fn oper(&self, op: &(dyn Fn(f64, f64) -> f64)) -> Vector {
        if ! self.eqal_shape(&rhs) {panic!("They don't have the same shape!")}
        let mut out = self.clone();
        for i in 0..out.shape() {
            out.val[i] = op(out.val[i], rhs.val[i])
        };
        out
    }
}

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.oper_with(rhs, &|i,j| { i + j })
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector<{}>", self.val.iter().map(|a| {format!("{}", a)}).collect::<Vec<_>>().join(", "))
    }
}

// impl std::ops::

pub struct Matrix {
    vec: Vec<Vector>
}

impl Matrix {
    pub fn new(shape:(usize, usize)) -> Self {
        let mut buf = Vec::new();
        for _ in 0..shape.0 {
            buf.push(Vector::new(shape.1))
        };
        Self {
            vec: buf
        }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix<\n    {}\n>", self.vec.iter().map(|a| {format!("{}", a)}).collect::<Vec<_>>().join("\n    "))
    }
}