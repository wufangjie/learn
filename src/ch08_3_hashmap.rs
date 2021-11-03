use crate::dbgt;
use std::collections::HashMap;

#[test]
pub fn test() {
    let mut map_1: HashMap<_, _> = HashMap::new();
    map_1.insert(1, 2); // insert will transfer ownership
                        // map_1.get(&1) // return Some(&V)
    assert!(map_1.contains_key(&1));
    assert!(!map_1.contains_key(&2));

    let keys = vec![1, 2, 3, 4, 5];
    let vals = vec![6, 7, 8, 9, 0];
    let mut map_from_pairs: HashMap<_, _> = keys.into_iter().zip(vals.into_iter()).collect();
    assert_eq!(5, map_from_pairs.len());
    assert_eq!(Some(9), map_from_pairs.remove(&4));

    // deref coercion
    let key = String::from("hello");
    let val = String::from("world");
    let mut map_3: HashMap<&str, &str> = HashMap::new();
    map_3.insert(&key, &val);
    map_3.insert("world", "hello");
    assert_eq!(Some(&"hello"), map_3.get("world")); // why this works?
    assert_eq!(Some(&"world"), map_3.get(&"hello"));

    // NOTE: update, insert, or_insert
    map_3.insert("hello", "rust"); // insert will update
    assert_eq!(Some(&"rust"), map_3.get(&"hello"));

    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
        let counter = letters.entry(ch).or_insert(0); // will return &mut
        *counter += 1;
    }
    assert_eq!(Some(&3), letters.get(&'t'));
    // NOTE: both entry() and or_insert() receive T, not &T

    // iterator
    // &map, map.iter()
    // &mut map, map.iter_mut()
    // map.into_iter()
    // keys()
    // values() values_mut()
}
