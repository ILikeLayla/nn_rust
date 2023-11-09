use std::iter::zip;

use super::{Matrix, Vector};

#[derive(Clone, Debug)]
pub enum DeterNum {
    Vec(Vector),
    Float(f64)
}

impl std::fmt::Display for DeterNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let words = match self {
            DeterNum::Vec(vec) => format!("{}", vec),
            DeterNum::Float(float) => format!("{}", float)
        };
        write!(f, "{}", words)
    }
}

impl std::ops::Add for DeterNum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DeterNum::Float(a), DeterNum::Float(b)) => DeterNum::Float(a+b),
            (DeterNum::Vec(a), DeterNum::Vec(b)) => DeterNum::Vec(a+b),
            _ => panic!("Vector can't add to float!")
        }
    }
}

impl std::ops::Sub for DeterNum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DeterNum::Float(a), DeterNum::Float(b)) => DeterNum::Float(a-b),
            (DeterNum::Vec(a), DeterNum::Vec(b)) => DeterNum::Vec(a-b),
            _ => panic!("Vector can't sub from or to float!")
        } 
    }
}

impl std::ops::Mul for DeterNum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DeterNum::Float(a), DeterNum::Float(b)) => DeterNum::Float(a*b),
            (DeterNum::Vec(a), DeterNum::Vec(b)) => DeterNum::Float(a*b),
            (DeterNum::Float(a), DeterNum::Vec(b)) => DeterNum::Vec(b.strech(a)),
            (DeterNum::Vec(a), DeterNum::Float(b)) => DeterNum::Vec(a.strech(b)),
        } 
    }
}

impl std::ops::Div for DeterNum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DeterNum::Float(a), DeterNum::Float(b)) => DeterNum::Float(a/b),
            _ => panic!("Vector can't divide or be divided!")
        } 
    }
}

impl std::ops::AddAssign for DeterNum {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

#[derive(Clone, Debug)]
pub struct Determinant {
    val: Vec<Vec<DeterNum>>
}

impl Determinant {
    pub fn from_matrix(matrix: &Matrix) -> Self {
        Self::from_vec(matrix.get_val())
    }

    pub fn from<const L: usize>(val: [[f64; L]; L]) -> Self {
        let mut buf = Vec::new();
        for i in val.iter() {
            let mut buf_1 = Vec::new();
            for j in i.iter() {
                buf_1.push(DeterNum::Float(*j))
            }
            buf.push(buf_1)
        };
        Self {
            val: buf
        }
    }

    pub fn from_i<const L: usize>(val: [[isize; L]; L]) -> Self {
        let buf = Matrix::from_i(val);
        Self::from_matrix(&buf)
    }

    pub fn from_vec(val: Vec<Vec<f64>>) -> Self {
        let mut buf = Vec::new();
        for i in val.iter() {
            let mut buf_1 = Vec::new();
            for j in i.iter() {
                buf_1.push(DeterNum::Float(*j))
            };
            buf.push(buf_1)
        };
        Self::from_vec_deter(buf)
    }

    pub fn from_vec_deter(val: Vec<Vec<DeterNum>>) -> Self {
        let out = Self { val };
        out.check_shape();
        out
    }

    fn check_shape(&self) {
        if self.val.len() != 0 && self.val.len() != self.val[0].len() {
            panic!("This shape is not available to determinant!")
        }
    }

    pub fn change_place(&mut self, x: usize, y: usize, val: DeterNum) {
        self.val[y][x] = val
    }

    pub fn change_place_float(&mut self, x: usize, y: usize, val: f64) {
        self.change_place(x, y, DeterNum::Float(val))
    }

    pub fn change_place_vector(&mut self, x: usize, y: usize, val: Vector) {
        self.change_place(x, y, DeterNum::Vec(val))
    }

    pub fn cal(&self) -> DeterNum {

        if self.val.len() == 0 {
            return DeterNum::Float(0.0)
        } else if self.val.len() == 1 {
            return self.val[0][0].clone()
        } else if self.val.len() == 2 {
            return self.val[0][0].clone() * self.val[1][1].clone() -
                   self.val[0][1].clone() * self.val[1][0].clone()
        }

        let mut add = Vec::new();
        let mut sub = Vec::new();

        for i in 0..self.val[0].len() {
            let mut buf = Vec::new();
            for row in 1..self.val.len() {
                let mut buf_1 = Vec::new();
                for col in 0..self.val[0].len() {
                    if col != i {
                        buf_1.push(self.val[row][col].clone());
                    }
                };
                buf.push(buf_1)
            };

            let cal = self.val[0][i].clone() * Self::from_vec_deter(buf).cal();
            if i % 2 == 0 { add.push(cal) } else { sub.push(cal) }
        }
        
        let mut buf_add = add[0].clone();
        for i in 1..add.len() {
            buf_add += add[i].clone()
        };

        let mut buf_sub = sub[0].clone();
        for i in 1..sub.len() {
            buf_sub += sub[i].clone()
        };

        return buf_add - buf_sub
    }
}

impl std::fmt::Display for Determinant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.val.len() == 0 {
            return write!(f, "| |")
        }

        let mut words_buf = Vec::new();
        let mut words = Vec::new();
        let mut length = Vec::new();

        // change the number into string, and collect their length.
        for i in self.val.iter() {
            let mut buf = Vec::new();
            let mut length_buf = Vec::new();
            for j in i.iter() {
                let word = format!("{}", j);
                length_buf.push(word.len());
                buf.push(word)
            };
            words_buf.push(buf);
            length.push(length_buf)
        };

        // get the max length for each column
        let max_length = {
            let mut max_length = Vec::new();
            for i in 0..length[0].len() {
                let mut max = 0;
                for j in 0..length.len() {
                    if length[j][i] > max {
                        max = length[j][i]
                    }
                }
                max_length.push(max)
            };
            max_length
        };

        // fill with the blank to make them have the same length
        for i in words_buf.iter_mut() {
            for (index, j) in zip(0..i.len(), i.iter_mut()) {
                let mut blank = String::new();
                for _ in 0..(max_length[index] - j.len()) {
                    blank.push(' ')
                };
                *j = format!("{}{}", blank, j)
            }
        }

        // connect them together
        for i in words_buf.iter_mut() {
            words.push(i.join(" "))
        }
        for i in words.iter_mut() {
            i.insert_str(0, "| ");
            i.insert_str(i.len(), " |")
        }

        write!(f, "{}", words.join("\n"))
    }
}