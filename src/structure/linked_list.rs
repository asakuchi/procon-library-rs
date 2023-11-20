use super::linked_list::LinkedList::*;

///
/// 連結リスト
///
#[derive(Debug, Clone)]
pub enum LinkedList {
    Cons(usize, Box<LinkedList>),
    Nil,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        Nil
    }

    pub fn prepend(self, element: usize) -> LinkedList {
        Cons(element, Box::new(self))
    }

    pub fn len(&self) -> usize {
        match &self {
            Cons(_, ref tail) => 1 + tail.len(),
            Self::Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
        match &self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Self::Nil => format!("Nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::linked_list::LinkedList;

    #[test]
    fn test_sample() {
        let mut list = LinkedList::new();

        list = list.prepend(10);
        list = list.prepend(20);
        list = list.prepend(30);

        assert_eq!(3, list.len());
        assert_eq!("30, 20, 10, Nil", list.stringify());
    }
}
