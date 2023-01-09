use procon_library_rs::prime::get_prime;

fn main() {
    let list = get_prime(100);
    println!("{:?}", list);
}
