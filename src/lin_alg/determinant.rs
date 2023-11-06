use super::{Matrix, Vector};

#[derive(Clone)]
enum Num {
    Vec(Vector),
    Float(f64)
}

#[derive(Clone)]
pub struct Determinant {
    val: Vec<Vec<Num>>
}

impl Determinant {
    // pub fn from_matrix(matrix: Matrix) -> Self {
    //     Self {
    //         matrix
    //     }
    // }

    pub fn from<const X: usize, const Y: usize>(val: [[f64; X]; Y]) -> Self {
        let mut buf = Vec::new();
        for i in val.iter() {
            let mut buf_1 = Vec::new();
            for j in i.iter() {
                buf_1.push(Num::Float(*j))
            }
            buf.push(buf_1)
        };
        Self {
            val: buf
        }
    }

    // pub fn from_vec(val: Vec<Vec<>>)

    fn change_place(&mut self, x: usize, y: usize, val: Num) {
        self.val[y][x] = val
    }

    pub fn change_place_float(&mut self, x: usize, y: usize, val: f64) {
        self.change_place(x, y, Num::Float(val))
    }

    pub fn change_place_vector(&mut self, x: usize, y: usize, val: Vector) {
        self.change_place(x, y, Num::Vec(val))
    }
}