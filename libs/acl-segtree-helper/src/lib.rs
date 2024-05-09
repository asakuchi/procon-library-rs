use ac_library::{LazySegtree, MapMonoid, Min, Monoid};

pub fn sum_add_tree(n: usize) -> LazySegtree<SumAdd> {
    LazySegtree::<SumAdd>::from(vec![Data::one_cell(); n])
}

pub fn sum_update_tree(n: usize) -> LazySegtree<SumUpdate> {
    LazySegtree::<SumUpdate>::from(vec![Data::one_cell(); n])
}

pub fn min_add_tree(n: usize) -> LazySegtree<MinAdd> {
    LazySegtree::<MinAdd>::from(vec![0; n])
}

pub fn min_update_tree(n: usize) -> LazySegtree<MinUpdate> {
    LazySegtree::<MinUpdate>::from(vec![0; n])
}

#[derive(Clone)]
pub struct Data {
    pub value: isize,
    pub size: usize,
}

impl Data {
    pub fn one_cell() -> Data {
        Data { value: 0, size: 1 }
    }
}

#[derive(Clone)]
pub enum RangeUpdateMappingType {
    Value(isize),
    ID,
}

pub struct RangeSumMonoid;

impl Monoid for RangeSumMonoid {
    ///
    /// モノイドの型
    ///
    type S = Data;

    ///
    /// 単位元
    ///
    fn identity() -> Self::S {
        Data { value: 0, size: 0 }
    }

    ///
    /// 二項演算
    ///
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        Data {
            value: a.value + b.value,
            size: a.size + b.size,
        }
    }
}

pub struct SumAdd;

impl MapMonoid for SumAdd {
    type M = RangeSumMonoid;
    /// 写像の型
    type F = isize;

    ///
    /// 恒等写像
    /// 全ての`a`に対して`mapping(id, a) = a`となるもの
    ///
    fn identity_map() -> Self::F {
        0
    }

    ///
    /// f(x) を返す関数
    ///
    /// dataの値`x`に対して作用させる関数
    ///
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        Data {
            value: x.value + x.size as isize * f,
            size: x.size,
        }
    }

    ///
    /// f∘g を返す関数
    ///
    /// `g` がこれまでの操作、`f` が後に追加する操作で、
    ///「その2つの操作を順に行うようなひとまとめの操作（合成写像）」を返す
    ///
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

pub struct SumUpdate;

impl MapMonoid for SumUpdate {
    type M = RangeSumMonoid;
    /// 写像の型
    type F = RangeUpdateMappingType;

    ///
    /// 恒等写像
    /// 全ての`a`に対して`mapping(id, a) = a`となるもの
    ///
    fn identity_map() -> Self::F {
        RangeUpdateMappingType::ID
    }

    ///
    /// f(x) を返す関数
    ///
    /// dataの値`x`に対して作用させる関数
    ///
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if let RangeUpdateMappingType::Value(value) = f {
            Data {
                value: x.size as isize * value,
                size: x.size,
            }
        } else {
            // f が ID ならそのまま x を返す
            x.clone()
        }
    }

    ///
    /// f∘g を返す関数
    ///
    /// `g` がこれまでの操作、`f` が後に追加する操作で、
    ///「その2つの操作を順に行うようなひとまとめの操作（合成写像）」を返す
    ///
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if let RangeUpdateMappingType::Value(_) = f {
            // 後からの操作で上書き
            f.clone()
        } else {
            // f が ID ならそのまま g を返す
            g.clone()
        }
    }
}

pub struct MinAdd;

impl MapMonoid for MinAdd {
    type M = Min<isize>;
    /// 写像の型
    type F = isize;

    ///
    /// 恒等写像
    /// 全ての`a`に対して`mapping(id, a) = a`となるもの
    ///
    fn identity_map() -> Self::F {
        0
    }

    ///
    /// f(x) を返す関数
    ///
    /// dataの値`x`に対して作用させる関数
    ///
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }

    ///
    /// f∘g を返す関数
    ///
    /// `g` がこれまでの操作、`f` が後に追加する操作で、
    ///「その2つの操作を順に行うようなひとまとめの操作（合成写像）」を返す
    ///
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

pub struct MinUpdate;

impl MinUpdate {
    const ID: isize = isize::MAX;
}

impl MapMonoid for MinUpdate {
    type M = Min<isize>;
    /// 写像の型
    type F = isize;

    ///
    /// 恒等写像
    /// 全ての`a`に対して`mapping(id, a) = a`となるもの
    ///
    fn identity_map() -> Self::F {
        MinUpdate::ID
    }

    ///
    /// f(x) を返す関数
    ///
    /// dataの値`x`に対して作用させる関数
    ///
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if f != MinUpdate::ID {
            f
        } else {
            x
        }
    }

    ///
    /// f∘g を返す関数
    ///
    /// `g` がこれまでの操作、`f` が後に追加する操作で、
    ///「その2つの操作を順に行うようなひとまとめの操作（合成写像）」を返す
    ///
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        if f == MinUpdate::ID {
            g
        } else {
            f
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::UnionFind;

    #[test]
    fn test_disjoint_set() {
        // let mut set = UnionFind::new(3);

        // assert_eq!(set.equiv(0, 1), false);
        // assert_ne!(set.find(0), set.find(1));

        // set.union(0, 1);

        // assert_eq!(set.equiv(0, 1), true);
        // assert_eq!(set.find(0), set.find(1));

        // set.union(0, 2);

        // assert_eq!(set.equiv(0, 2), true);
        // assert_eq!(set.find(0), set.find(1));
        // assert_eq!(set.find(1), set.find(2));
    }
}
