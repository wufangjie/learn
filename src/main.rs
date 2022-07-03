#[allow(unused_imports)]
use utils::dbgt;
//use utils::dbgt::type_of;

mod ch04_reference;
mod ch06_enum;

mod ch08_1_vector;
mod ch08_2_string;
mod ch08_3_hashmap;
mod ch08_4_btree;

mod format;
mod quicksort;

mod ch10_2_trait;
mod ch10_3_lifetime;

mod ch13_1_closure;
mod ch13_2_iterator;

mod ch15_2_deref;
mod ch15_3_drop;
mod ch15_4_rc;
mod ch15_5_refcell;
mod ch15_6_ref_cycle;
mod ch15_7_cell;

mod ch16_concurrency;

mod ch17_2_trait_object;
mod ch17_3_1_state_pattern;
mod ch17_3_2_type_driven;
mod ch18_match;
mod ch19_1_unsafe;
mod ch19_2_adv_trait;
mod ch19_4_adv_fn;

mod recursive;

mod calculator;

mod conversion;

fn main() {
    // let mut p1 = Human {
    //     name: "wfj".to_string(),
    //     age: 32,
    //     current_thought: "rust is awesome".to_string(),
    // };
    // p1.with_thought_1("from thought 1");
    // dbg!(&p1);
    // let p2 = p1.with_thought_2("from thought 2");
    // dbg!(&p2);

    // test_1("wfj".to_string());
    // test_2("xl".to_string());

    //dbgt!(&String::from(s).as_str()); // &str
    // let k = 21;
    // let mut a1: Option<i32> = Some(k);
    // let mut a2: Option<i32> = None;
    // assert_eq!(a1.take(), Some(21));
    // assert_eq!(a2.take(), None);
    // assert_eq!(a1, None);
    // assert_eq!(a2, None);

    // let line = "1\n2\n3\n4\na\n";

    // for num in line.lines() {
    //     match num.parse::<i32>().map(|i| i * 2) {
    //         Ok(n) => println!("{n}"),
    //         Err(..) => println!("empty"),
    //     }
    // }
    //dbg!(prod_triple((2, 3, 4)));

    // let value = "hello world".to_string();
    // match value {
    //     //x if value.len() == 11 => println!("Found: {}", x),
    //     _ => println!("Not Found!"),
    // }
    // dbg!(&value); // we can not use value here, value was moved

    // use std::mem::{size_of, size_of_val};

    // let pointer_size = size_of::<&u8>();
    // assert_eq!(2 * pointer_size, size_of::<&str>());
    // assert_eq!(1 * pointer_size, size_of::<&String>());
    // assert_eq!(3 * pointer_size, size_of::<Vec<u8>>());
    // // start position, length, capacity
    // assert_eq!(3 * pointer_size, size_of::<String>());
    // // special vector

    // assert_eq!(1 * pointer_size, size_of::<Box<u8>>());
    // assert_eq!(1 * pointer_size, size_of::<std::rc::Rc<u8>>());
    // dbg!(size_of::<std::cell::Ref<u8>>()); // 16
    // dbg!(size_of::<std::cell::Cell<u8>>()); // 1
    // assert_eq!(2 * pointer_size, size_of::<std::cell::RefCell<u8>>());

    // let s = "hello world";
    // assert_eq!(s.len(), size_of_val(s));

    //dbg!("{}", 5.copy());

    // double::<{ 4 + 5 }>();

    // use std::collections::HashMap;
    // let mut map: HashMap<i64, i64> = HashMap::new();

    // map.insert(1, 2);
    // map.insert(3, 4);
    // map.insert(5, 6);
    // dbgt!(&map);
    // dbgt!(&map.iter());

    // for (&k, &v) in map.iter() {
    //     dbgt!(&k);
    //     dbgt!(&v);
    // }

    // for (k, v) in map.iter() {
    //     dbgt!(&k);
    //     dbgt!(&v);
    // }

    // let arr = [1, 2, 3, 4, 5, 6];
    // dbg!(&arr.into_iter().collect::<Vec<i32>>());
    // dbg!(&arr);

    // let a: Option<i32> = Some(5);
    // let b: Option<i32> = None;

    // assert_eq!(5, a.unwrap_or(42));
    // assert_eq!(42, b.unwrap_or(42));

    // // dbg!(a.or(Some(42)));
    // // dbg!(b.or(Some(42)));

    // let g: Vec<Option<i32>> = vec![Some(1), None, Some(42), None, None, Some(5), None];
    // dbg!(g.into_iter().filter_map(|x| x).collect::<Vec<i32>>());
    // // for v in iter {
    // //     dbg!(&v);
    // // }

    // dbg!(std::mem::size_of::<&dyn std::string::ToString>());
    // dbg!(std::mem::size_of_val(&"sdfsba".to_string()));
    // dbg!(std::mem::size_of::<Box<dyn std::string::ToString + Send>>());
    // dbg!(std::mem::size_of::<Box<u8>>());
    // dbg!(std::mem::size_of::<Box<String>>());

    // let a = "hello world".to_string();
    // let b = &a;
    // dbgt!(&*b);

    // dbg!(std::mem::size_of::<Box<dyn ToString + B>>());
    // let trait_objects: Vec<Box<dyn A + B>> = vec![Box::new(5i8), Box::new(42u8)];
}


// trait A: std::fmt::Display {
//     fn print_a(&self) {
//         println!("print_a: {}", self);
//     }
// }

// trait B: std::fmt::Display {
//     fn print_b(&self) {
//         println!("print_b: {}", self);
//     }
// }

// impl A for u8 {}
// impl B for u8 {}

// impl A for i8 {}
// impl B for i8 {}

// fn double<const N: i32>() {
//     println!("double: {}", N * 2);
// }

// fn prod_triple((x, y, z): (i32, i32, i32)) -> i32 {
//     return x * y * z;
// }

// #[derive(Debug)]
// struct Human {
//     name: String,
//     age: i8,
//     current_thought: String,
// }

// impl Human {
//     fn with_thought_1(&mut self, thought: &str) -> () {
//         self.current_thought = thought.to_string();
//     }

//     fn with_thought_2(mut self, thought: &str) -> Human {
//         // let mut self = Human {
//         //     name: "xl".to_string(),
//         //     age: 30,
//         //     current_thought: thought.to_string()
//         // };
//         self.current_thought = thought.to_string();
//         return self;
//     }
// }

// fn test_1(mut name: String) {
//     name.push_str(", hello");
//     dbg!(name);
// }

// fn test_2(name: String) {
//     let mut name = name;
//     name.push_str(", hello");
//     dbg!(name);
// }

// trait Animal {
//     fn get_name(&self) -> String;
// }

// struct Panda;
// struct Monkey;

// /// 以下实现对应vtable1
// impl Animal for Panda {
//     fn get_name(&self) -> String {
//         "panda".to_owned()
//     }
// }

// /// 以下实现对应vtable2
// impl Animal for Monkey {
//     fn get_name(&self) -> String {
//         "monkey".to_owned()
//     }
// }

// static panda: Panda = Panda {};
// static monkey: Monkey = Monkey {};

// // fn get_animal(animal_type: i32) -> Box<&'static dyn Animal> {
// //     if animal_type == 0 {
// // 	Box::new(&panda as &dyn Animal) // 将panda的数据的地址及vtable1组成fat pointer
// //     } else {
// // 	Box::new(&monkey as &dyn Animal) // 将monkey的数据的地址及vtable2组成fat pointer
// //     }
// // }

// fn get_animal(animal_type: i32) -> &'static dyn Animal {
//     //Box<&'static dyn Animal> {
//     if animal_type == 0 {
//         &panda
//         //Box::new(&panda)
//     } else {
//         &monkey
//         //Box::new(&monkey)
//     }
// }

// fn main() {
//     println!("{}", get_animal(0).get_name());
//     println!("{}", get_animal(1).get_name());
// }
