mod lin_alg;

fn main() {
    let a = lin_alg::Matrix::new((10,10));
    println!("{}", a);
}
