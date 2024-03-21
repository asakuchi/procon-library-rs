use asakuchi_input::Input;

fn main() {
    let stdin = std::io::stdin();
    let mut input = Input::new(stdin.lock());

    let n = 3;
    let list: Vec<usize> = input.vec(n);

    println!("{:?}", list);

    let list_2: Vec<(usize, isize)> = input.vec(n);

    println!("{:?}", list_2);
}
