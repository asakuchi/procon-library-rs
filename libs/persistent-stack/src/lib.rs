use std::rc::Rc;

///
/// 永続スタック
///
#[derive(Debug, Clone)]
pub enum PersistentStack {
    Cons(usize, Rc<PersistentStack>),
    Nil,
}

impl PersistentStack {
    pub fn new() -> PersistentStack {
        PersistentStack::Nil
    }

    pub fn top(&self) -> Option<usize> {
        match self {
            Self::Cons(value, _prev) => Some(*value),
            Self::Nil => None,
        }
    }

    pub fn push(self: &Rc<PersistentStack>, x: usize) -> Rc<PersistentStack> {
        Rc::new(PersistentStack::Cons(x, Rc::clone(self)))
    }

    pub fn pop(&self) -> Option<Rc<PersistentStack>> {
        match self {
            Self::Cons(_value, prev) => Some(Rc::clone(prev)),
            Self::Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::PersistentStack;

    #[test]
    fn test_sample() {
        let a_0 = Rc::new(PersistentStack::new());
        let a_1 = a_0.push(3);

        assert_eq!(Some(3), a_1.top());

        let a_2 = a_1.push(1);

        assert_eq!(Some(3), a_1.top());
        assert_eq!(Some(1), a_2.top());

        let a_3 = a_2.push(4);

        assert_eq!(Some(3), a_1.top());
        assert_eq!(Some(1), a_2.top());
        assert_eq!(Some(4), a_3.top());

        let b_2 = a_1.push(3);
        let b_3 = b_2.push(4);

        assert_eq!(Some(3), a_1.top());
        assert_eq!(Some(1), a_2.top());
        assert_eq!(Some(4), a_3.top());
        assert_eq!(Some(3), b_2.top());
        assert_eq!(Some(4), b_3.top());

        let c_2 = a_1.pop().unwrap();

        assert_eq!(Some(3), a_1.top());
        assert_eq!(Some(1), a_2.top());
        assert_eq!(Some(4), a_3.top());
        assert_eq!(Some(3), b_2.top());
        assert_eq!(Some(4), b_3.top());
        assert_eq!(None, c_2.top());
    }
}
