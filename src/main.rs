use crate::func::{Sigmoid, Softmax};

mod lin_alg;
mod func;
mod data_pro;

const LEARNING_RATE: f64 = 0.1;

fn main() {
    let mut dataset = data_pro::DataSet::read_from("D:/code/nn_rust/data/mnist_test_short.csv").slice(0, 10).data();

    dataset.restrict(-50.0, 50.0);
    println!("{}", dataset);

    let hidden_layer = lin_alg::Matrix::new((10, 784));

    let out = dataset * hidden_layer;

    let sig = out.sig_for();
    
    let posibility = sig.soft_for();

    println!("{}", posibility);


}
