//use crate::dbgt;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn test_ch08_4() {
    let mut solar_distance = BTreeMap::from([("Mercury", 0.4), ("Earth", 1.0), ("Mars", 1.5)]);
    solar_distance.insert("Venus", 0.7);
    for (key, val) in &solar_distance {
        println!("{}: {}", key, val); // alphabet order
    }

    let mut planets = BTreeSet::from(["Mercury", "Earth", "Mars"]);
    planets.insert("Venus");
    for p in &planets {
        println!("{}", p);
    }
}
