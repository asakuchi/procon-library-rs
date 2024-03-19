use std::rc::Rc;

///
/// 永続配列
///
#[derive(Debug, Clone)]
pub enum PersistentArray {
    Cons(usize, Vec<Rc<PersistentArray>>),
    Nil,
}

impl PersistentArray {
    pub fn new() -> Rc<PersistentArray> {
        Rc::new(PersistentArray::Nil)
    }

    pub fn set(&self, index: usize, value: usize) -> Rc<PersistentArray> {
        let (mut new_value, mut new_children) = match self {
            Self::Cons(prev_value, prev_children) => (*prev_value, prev_children.clone()),
            Self::Nil => (0, vec![Rc::new(PersistentArray::Nil); 20]),
        };

        if index == 0 {
            new_value = value;
        } else {
            new_children[index % 20] = new_children[index % 20].set(index / 20, value);
        }

        Rc::new(PersistentArray::Cons(new_value, new_children))
    }

    pub fn get(&self, index: usize) -> usize {
        match self {
            Self::Cons(value, children) => {
                if index == 0 {
                    *value
                } else {
                    children[index % 20].get(index / 20)
                }
            }
            Self::Nil => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::PersistentArray;

    #[test]
    fn test_sample() {
        let a_0 = Rc::new(PersistentArray::Nil);

        let a_1 = a_0.set(2, 10);

        assert_eq!(10, a_1.get(2));

        let a_2 = a_1.set(30, 20);

        assert_eq!(10, a_1.get(2));
        assert_eq!(10, a_2.get(2));
        assert_eq!(0, a_1.get(30));
        assert_eq!(20, a_2.get(30));

        let a_3 = a_2.set(4_000_000, 100);
        let a_4 = a_3.set(5_000_000, 200);

        let b_3 = a_2.set(5_000_000, 300);

        assert_eq!(10, a_1.get(2));
        assert_eq!(10, a_2.get(2));
        assert_eq!(0, a_1.get(30));
        assert_eq!(20, a_2.get(30));
        assert_eq!(100, a_3.get(4_000_000));
        assert_eq!(200, a_4.get(5_000_000));
        assert_eq!(0, b_3.get(4_000_000));
        assert_eq!(300, b_3.get(5_000_000));
    }
}
