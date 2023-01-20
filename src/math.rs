///
/// 10進数をn進数に変換
///
/// 負も受け付ける
///
/// https://atcoder.jp/contests/abc105/tasks/abc105_c
///
pub fn to_n_adic(x: isize, radix: isize) -> Vec<usize> {
    if x == 0 {
        return vec![0];
    }

    let mut x = x;

    let mut list = Vec::new();

    while x != 0 {
        let mut p = x / radix;
        let mut q = x % radix;

        if q < 0 {
            p += 1;
            q += -radix;
        }

        list.push(q as usize);

        x = p;
    }

    list.reverse();

    list
}

#[test]
fn test_to_n_adic() {
    fn join(list: &Vec<usize>) -> String {
        list.iter().map(|x| x.to_string()).collect()
    }

    assert_eq!(join(&to_n_adic(12, 2)), format!("{:b}", 12));
    assert_eq!(join(&to_n_adic(35, 8)), format!("{:o}", 35));

    assert_eq!(join(&to_n_adic(-9, -2)), "1011");
    assert_eq!(
        join(&to_n_adic(123456789, -2)),
        "11000101011001101110100010101"
    );
    // 0 はどうすべき？
    assert_eq!(join(&to_n_adic(0, -2)), "0");

    println!("{:?}", &to_n_adic(7, 8));
    println!("{:?}", &to_n_adic(-9, -2));
    println!("{:0b}", 0);
    println!("{:0b}", 1);
    println!("{:0b}", 2);
}
