use crate::dbgt;
// Iterator trait

// methods:
// map filter zip enumerate rev
// skip skip_while take take_while
// chain step_by
// reduce fold
// sum product any all count
// chunks(n) <= n
// flat_map

// T: Iterator<Item = i32>

// impl Iterator for T {
//     type Item = i32;

//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

#[test]
fn test_iterator() {
    let a = vec![1, 2, 3, 4];

    assert_eq!(24, a.iter().product());
    assert_eq!(
        vec![&3, &4],
        a.iter().skip_while(|&&x| x < 3).collect::<Vec<&i32>>()
    );
    // NOTE: &&3, skip_while take a reference of the iterator
    assert_eq!(
        Some(5),
        vec![2, 3].into_iter().reduce(|x, y| {
            let ret = x + y;
            dbgt!(&x);
            dbgt!(&ret);
            ret
        })
    );
    // NOTE: the type of &1 + &3 is i32, so reduce is hard to write on reference

    assert_eq!(
        vec![1, 2, 3, 4, 3, 4].iter().collect::<Vec<&i32>>(),
        a.iter().chain(a.iter().skip(2)).collect::<Vec<&i32>>()
    );

    //dbg!(a.iter().repeat(2).collect::<Vec<&i32>>())
    assert_eq!(vec![&1, &4], a.iter().step_by(3).collect::<Vec<&i32>>());
    assert_eq!(11, a.clone().into_iter().fold(1, |x, y| x + y)); // accumulate

    // NOTE: by_ref for not consuming iter
    let mut words = vec!["hello", "world", "of", "Rust"].into_iter();
    let hello_world: Vec<_> = words.by_ref().take(2).collect();
    assert_eq!(hello_world, vec!["hello", "world"]);
    assert_eq!(words.collect::<Vec<_>>(), vec!["of", "Rust"]);
}

// Iterators are one of Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead.
