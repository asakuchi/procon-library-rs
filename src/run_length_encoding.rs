///
/// ランレングス圧縮
///
pub fn run_length_encoding<T>(list: &Vec<T>) -> Vec<(T, usize)>
where
    T: PartialEq + Copy,
{
    let mut result = Vec::new();

    for &value in list.iter() {
        if let Some((next, size)) = result.pop() {
            if next == value {
                result.push((next, size + 1));
            } else {
                result.push((next, size));
                result.push((value, 1));
            }
        } else {
            result.push((value, 1));
        }
    }

    result
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_run_length_enc() {
        let list = vec![1, 2, 3, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4];

        assert_eq!(
            run_length_encoding(&list),
            vec![(1, 1), (2, 1), (3, 1), (1, 2), (2, 3), (3, 4), (4, 1)]
        );

        let list = vec![
            'a', 'b', 'b', 'b', 'a', 'c', 'c', 'b', 'a', 'a', 'a', 'b', 'd',
        ];

        assert_eq!(
            run_length_encoding(&list),
            vec![
                ('a', 1),
                ('b', 3),
                ('a', 1),
                ('c', 2),
                ('b', 1),
                ('a', 3),
                ('b', 1),
                ('d', 1)
            ]
        );
    }
}
