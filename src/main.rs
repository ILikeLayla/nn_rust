use crate::func::{Sigmoid, Softmax};

mod lin_alg;
mod func;
mod data_pro;

fn main() {
    let data = data_pro::DataSet::read_from("D:/code/nn_rust/data/mnist_test.csv").slice(10).data();
    let hidden_layer = lin_alg::Matrix::new((10, 784));
    let out = data * hidden_layer;
    let after_sig = out.sig_for();
    println!("{}", out);
    println!("{}", after_sig);
    println!("{}", after_sig.soft_for())
}
