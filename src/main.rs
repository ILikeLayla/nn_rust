use crate::func::Softmax;

mod lin_alg;
mod func;
mod data_pro;

fn main() {
    let data = data_pro::DataSet::read_from("D:/code/nn_rust/data/mnist_test.csv").slice(10).data();
    let hidden_layer = lin_alg::Matrix::new((10, 784));
    let out = data * hidden_layer;
    println!("{}", out);
    println!("{}", out.soft_for());
    println!("{:?}", out.shape())
}
