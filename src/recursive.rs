#![allow(dead_code)]
use crate::timeit;
use crate::utils::Stack;

// closure: no generic, no recursive
// function: function pointer

// TODO: generator version may be faster, since rust have no tail recursion
// or I should write loop version by hand?

fn fact_iter(n: u64) -> u64 {
    fn rec(res: u64, counter: u64) -> u64 {
        if counter < 2 {
            res
        } else {
            rec(res * counter, counter - 1)
        }
    }
    rec(1, n)
}

fn fib_raw(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib_raw(n - 1) + fib_raw(n - 2)
    }
}

fn fib_iter(n: u64) -> u64 {
    fn rec(res: u64, res_1: u64, counter: u64) -> u64 {
        if counter == 0 {
            res
        } else {
            rec(res + res_1, res, counter - 1)
        }
    }
    rec(0, 1, n)
}

const MAX_SIZE: usize = 100;

pub struct Fib {
    // loop version, for reuse
    cache: [u64; MAX_SIZE], // actually MAX_SIZE = 100 overflow already
    cursor: usize,
}

impl Fib {
    fn new() -> Self {
        let mut memo = Fib {
            cache: [0; MAX_SIZE],
            cursor: 2,
        };
        memo.cache[1] = 1;
        memo
    }

    fn eval(&mut self, n: usize) -> u64 {
        if n < self.cursor {
            self.cache[n]
        } else {
            for i in self.cursor..=n {
                self.cache[i] = self.cache[i - 1] + self.cache[i - 2];
            }
            self.cursor = n;
            self.cache[n]
        }
    }
}

//const COINS: [u8; 5] = [50, 25, 10, 5, 1];
const COINS: [i32; 5] = [1, 5, 10, 25, 50];

fn count_change(total: i32) -> i32 {
    fn rec(i: usize, mut left: i32) -> i32 {
        if i == 0 {
            (left % COINS[0] == 0) as i32
        } else if left == 0 {
            1
        } else {
            let mut res = 0;
            while left >= 0 {
                res += rec(i - 1, left);
                left -= COINS[i];
            }
            res
        }
    }
    rec(COINS.len() - 1, total)
}

fn count_change_stack(total: i32) -> i32 {
    // NOTE: this version is slower, the overhead of vec?
    // this version will be much faster in release mode
    // use memo to speed up?
    let mut stack = Stack::new();
    stack.push((COINS.len() - 1, total));
    let mut res = 0;
    while let Some((i, mut left)) = stack.pop() {
        if i == 0 {
            res += (left % COINS[0] == 0) as i32;
        } else if left == 0 {
            res += 1;
        } else {
            while left >= 0 {
                stack.push((i - 1, left));
                left -= COINS[i];
            }
        }
    }
    res
}

fn expt(b: i64, n: u32) -> i64 {
    if n == 0 {
        1
    } else {
        let sqrt = expt(b, n >> 1);
        sqrt * sqrt * if n & 1 != 0 { b } else { 1 }
    }
}

fn fib_logn(n: u64) -> u64 {
    fn rec(a: u64, b: u64, p: u64, q: u64, count: u64) -> u64 {
        if count == 0 {
            b
        } else if (count & 1) != 0 {
            rec((p + q) * a + q * b, q * a + p * b, p, q, count - 1)
        } else {
            rec(a, b, p * p + q * q, 2 * p * q + q * q, count >> 1)
        }
    }
    rec(1, 0, 0, 1, n)
}

// other examples: gcd, prime

fn product(n: u16, k: u8) -> Vec<u16> {
    // n < 11
    let mut ret = vec![];
    if k == 0 {
        ret.push(0);
    } else {
        for comb in product(n, k - 1) {
            let comb10 = comb * 10;
            for v in 0..n {
                ret.push(comb10 + v);
            }
        }
    }
    ret
}

#[test]
fn test() {
    assert_eq!(120, fact_iter(5));

    timeit!(100, fib_iter(80));
    timeit!(100, Fib::new().eval(80));
    timeit!(100, fib_logn(80));

    assert_eq!(fib_iter(80), Fib::new().eval(80));
    assert_eq!(fib_iter(80), fib_logn(80));

    timeit!(count_change(1000));
    timeit!({
	count_change(1000);
	count_change_stack(1000)
    });
    timeit!(count_change_stack(1000));

    assert_eq!(count_change(1000), count_change_stack(1000));

    let b = -5i64;
    let n = 7u32;
    assert_eq!(b.pow(n), expt(b, n));

    //println!("{:#?}", product(3, 3));
}
