mod lin_alg;

fn main() {
    let mut a = lin_alg::Determinant::from_i([
        [1,2,3],
        [4,5,6],
        [7,8,9]
    ]);
    a.change_place_vector(0, 0, lin_alg::Vector::from_i([1,2,3]));
    println!("{}", a)
}
