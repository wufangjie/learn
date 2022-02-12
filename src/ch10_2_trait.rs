use crate::dbgt;

// as parameters and return
// impl trait_name
// &impl trait_name

// <T: trait_name> then (T or &T)

pub fn largest<T>(lst: &[T]) -> &T
where
    T: PartialOrd + std::fmt::Debug,
{
    let mut ret: &T = &lst[0];
    for v in lst {
        if v > ret {
            dbgt!(&&v);
            ret = v; //ret = &v; // is ok, deref?
            dbgt!(&ret);
        }
    }
    ret
}

#[test]
fn test_largest() {
    let a: Vec<String> = vec![23, 33, 6, 84, 70, 29, 57, 43, 47, 18, 63]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    assert_eq!("84", largest(&a));
}
