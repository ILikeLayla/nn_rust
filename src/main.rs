mod lin_alg;
mod func;
mod data_pro;

fn main() {
    println!("{:?}", data_pro::DataSet::read_from("D:/code/nn_rust/data/mnist_test.csv").slice(100).shape());
}
