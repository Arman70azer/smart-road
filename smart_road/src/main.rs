use smart_road::matrice::matrice;
use smart_road::interface::interface;

const LENGTH: u32 = 1000;
const WIDTH: u32 = 1000;

fn main() {
    let result = matrice(LENGTH/100, WIDTH/100);
    println!("{:?}", result);
    interface(LENGTH, WIDTH);
}
