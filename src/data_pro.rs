use polars::prelude::{self, SerReader, DataFrame};
use std::fs::File;
use super::lin_alg::{Vector, Matrix};

pub struct Data {
    val: Vector,
    label: u8
}

impl Data {
    
}

pub struct DataSet {
    val: Matrix,
    label: Vec<i64>
}

impl DataSet {
    pub fn read_from(path: &str) -> Self{
        let mut label: Vec<i64> = Vec::new();
        let mut buf: Vec<Vec<f64>> = Vec::new();
        let file = File::open(path.to_string()).expect("Cannot find the file");
        let csv = prelude::CsvReader::new(file)
            .infer_schema(None)
            .has_header(true)
            .finish().expect("Cannot not analyse the file");
        for i in csv.iter() {
            if label.is_empty() {
                label = i.i64().unwrap().into_no_null_iter().collect();
            } else {
                buf.push(i.f64().unwrap().into_no_null_iter().collect())
            }
        }
        Self {
            val: Matrix::from_ver_vec(buf),
            label
        }
    }
}

impl std::fmt::Display for DataSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "label: {:?}\n {}", self.label, self.val)
    }
}