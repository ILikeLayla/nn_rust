use crate::func::{Sigmoid, Softmax};

mod lin_alg;
mod func;
mod data_pro;

const LEARNING_RATE: f64 = 0.1;

fn main() {
    // let mut dataset = data_pro::DataSet::read_from("D:/code/nn_rust/data/mnist_test_short.csv").slice(0, 10).data();

    // dataset.restrict(-50.0, 50.0);
    // println!("{}", dataset);

    // let hidden_layer = lin_alg::Matrix::new((10, 784));

    // let out = dataset * hidden_layer;

    // let sig = out.sig_for();
    
    // let posibility = sig.soft_for();

    // println!("{}", posibility);

    // let number1: i32 = 80;
    // let number2: i64 = 80;
    // let sum = number1 + number2 as i32;
    // println!("{:?}", sum);
    let area = lin_alg::Determinant::from([
        [-17.142857, 9.964465, 1.0, 1.0, 1.0, 1.0],
        [-8.571428, 17.0 , 1.0, 1.0, 1.0, 1.0],
        [0.0, 14.0, 1.0, 1.0, 1.0, 1.0],
        [0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
        [-8.571428, -3.0, 1.0, 1.0, 1.0, 1.0],
        [-17.142857, 4.246938, 1.0, 1.0, 1.0, 1.0]
    ]);
    // -610.65
    println!("{}", area);
    println!("{}", area.cal())
}
