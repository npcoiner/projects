use rmathlibrary::mat;
use rmathlibrary::rand;
use rmathlibrary::vec;

fn main() {
    let a = [1.0, 2.0];
    let b = [3.0, 4.0];
    println!("Dot product: {}", vec::dot_prod(a, b));

    let m1 = [[1.0, 2.0], [3.0, 4.0]];
    let m2 = [[5.0, 6.0], [7.0, 8.0]];
    let product = mat::multiply_2x2(m1, m2);
    println!("Matrix product: {:?}", product);

    let r = rand::random_f32_range(0.0, 1.0);
    println!("Random number: {}", r);
}
