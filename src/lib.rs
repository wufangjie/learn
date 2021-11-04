pub mod utils;

// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
// NOTE: must be used for a debug purpose only:
pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

// `#[macro_export]` will be exported at the root of the crate
// NOTE: dbg!(var1, var2) is ok, but dbgt! cannot
#[macro_export]
macro_rules! dbgt {
    ($val:expr) => {
        match $val {
            tmp => {
                eprintln!(
                    "[{}:{}] ({}: {}) = {:#?}",
                    file!(),
                    line!(),
                    stringify!($val),
                    $crate::type_of(tmp), // not $val, &tmp
                    &tmp
                );
                tmp
            }
        }
    };
}

// seems no need to support statement
// put block before expr, block belongs to expr?
#[macro_export]
macro_rules! timeit {
    ($loops:expr, $code:block) => {
        let _n = $loops;
        let _start = std::time::Instant::now();
        for _ in 0.._n {
            $code
        }
        let _cost = _start.elapsed();
        println!(
            "[{}:{}] ({} loops, {:?} per loop) {{ ... }}",
            file!(),
            line!(),
            _n,
            _cost / _n
        );
    };
    ($loops:expr, $code:expr) => {
        let _n = $loops;
        let _start = std::time::Instant::now();
        for _ in 0.._n {
            $code;
        }
        let _cost = _start.elapsed();
        println!(
            "[{}:{}] ({} loops, {:?} per loop) {{ {} }}",
            file!(),
            line!(),
            _n,
            _cost / _n,
            stringify!($code)
        );
    };
}
