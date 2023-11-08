mod lin_alg;

fn main() {
    let a = lin_alg::Matrix::from_i([
        [1,2,3],
        [4,5,6]
    ]);
    println!("{}", a);
    println!("{}", a.softmax()) 
}
