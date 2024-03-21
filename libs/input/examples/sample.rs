use asakuchi_input::Input;

fn main() {
    let mut input = Input::stdio();

    let n = 3;
    let list: Vec<usize> = input.vec(n);

    println!("{:?}", list);

    let list_2: Vec<(usize, isize)> = input.vec(n);

    println!("{:?}", list_2);
}
