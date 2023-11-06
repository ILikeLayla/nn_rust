mod lin_alg;

fn main() {
    let a = lin_alg::Vector::from_i([1,2,3]);
    let b = lin_alg::Vector::from_i([4,5]);
    println!("{}", a.out_product(b))
}
