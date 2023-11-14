mod lin_alg;
mod func;

fn main() {
    let mut a = lin_alg::Matrix::from_i([
        [1,2,3],
        [4,5,6]
    ]);
    let b = lin_alg::Matrix::from_i([
        [1,3],
        [0,1],
        [2,2]
    ]);
    a *= b;
    println!("{}", a);
    println!("{}", a.softmax());
}
