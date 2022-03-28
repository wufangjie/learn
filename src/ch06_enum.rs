#![allow(dead_code)]
use std::fmt;

// enum with implicit discriminator (starts at 0)
#[derive(Debug)]
enum Number {
    Zero,
    Two,
    One,
}

// enum with explicit discriminator
#[derive(Debug)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
pub enum List<T>
where
    T: fmt::Display + fmt::Debug,
{
    Cons(T, Box<List<T>>), // NOTE: Box
    Nil,
}

impl<T> List<T>
where
    T: fmt::Display + fmt::Debug,
{
    pub fn new() -> List<T> {
        Self::Nil
    }

    pub fn prepend(self, item: T) -> List<T> {
        Self::Cons(item, Box::new(self))
    }

    pub fn stringify(&self) -> String {
        match self {
            Self::Cons(head, tail) => format!("({}, {})", head, tail.stringify()),
            Self::Nil => format!("()"),
        }
    }
}

#[test]
fn test() {
    // `enums` can be cast as integers, but do not rely on this
    assert_eq!(0, Number::Zero as i32);
    assert_eq!(2, Number::One as i32);
    // Message::Quit as i32; // Error: can not convert

    assert_eq!("Red", format!("{:#?}", Color::Red));
    assert_eq!("#0000ff", format!("#{:06x}", Color::Blue as i32));

    let list = List::new().prepend(1).prepend(2).prepend(3);
    assert_eq!("(3, (2, (1, ())))", list.stringify());
}

#[derive(Debug, PartialEq)]
enum Attribute {
    Strength,
    Agility,
    Intellect,
}

#[derive(Debug, PartialEq)]
enum Parameter {
    Health,
    Mana,
}

#[derive(Debug, PartialEq)]
struct MultiBuf {
    attr_lst: Vec<Attribute>,
    param_lst: Vec<Parameter>,
}

#[derive(Debug, PartialEq)]
enum BuffTarget {
    Attribute(Attribute),
    Parameter(Parameter),
    Multi(MultiBuf),
}

#[test]
fn test_nested() {
    let a = BuffTarget::Attribute(Attribute::Strength);
    let b = BuffTarget::Parameter(Parameter::Mana);
    let c = BuffTarget::Multi(MultiBuf {
        attr_lst: vec![Attribute::Strength],
        param_lst: vec![Parameter::Mana],
    });
    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
    assert_ne!(a, b);
    assert_ne!(a, c);
}
