mod lin_alg;

fn main() {
    let a = lin_alg::Matrix::from([
        [1.0, 1.0], 
        [1.0, 1.0]
    ]);
    println!("{}", a);
}
