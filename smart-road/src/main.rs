use smart_road::matrice::matrice;
use smart_road::interface::interface;

const LENGTH: u8 = 30;
const WIDTH: u8 = 20;

fn main() {
    let result = matrice(LENGTH, WIDTH);
    println!("{:?}", result);
    interface(LENGTH, WIDTH);
}
