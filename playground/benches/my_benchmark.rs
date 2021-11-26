use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

const N: usize = 10000;
static mut ARR: [i32; N] = [0; N];

// https://doc.rust-lang.org/cargo/reference/build-scripts.html

#[link(name = "badarray", kind = "static")]
extern "C" {
    fn reverse_array(arr: *mut i32, n: usize);
}

fn gen_arr() -> [i32; N] {
    //let mut arr = [0; N];
    //rand::thread_rng().fill(&mut arr[..]);
    //arr
    unsafe { ARR }
}

fn reverse_by_c() {
    let mut arr = gen_arr();
    unsafe {
        reverse_array(arr.as_mut_ptr(), arr.len());
    }
}

fn reverse_by_rust() {
    let mut arr = gen_arr();
    arr.reverse();
}

fn reverse_unsafe_1() {
    let mut arr = gen_arr();
    let n = arr.len();
    unsafe {
        for i in 0..(n >> 1) {
            std::ptr::swap(&mut arr[i], &mut arr[n - 1 - i]);
        }
    }
}

fn reverse_unsafe_2() {
    let mut arr = gen_arr();
    let n = arr.len() as isize;
    let ptr = arr.as_mut_ptr();
    for i in 0..(n >> 1) as isize {
        unsafe {
            std::ptr::swap(ptr.offset(i), ptr.offset(n - 1 - i));
        }
    }
}

pub fn criterion_benchmark_change_arr(c: &mut Criterion) {
    c.bench_function("c", |b| {
        b.iter(|| unsafe {
            rand::thread_rng().fill(&mut ARR[..]);
        })
    });
}

pub fn criterion_benchmark_c(c: &mut Criterion) {
    c.bench_function("c", |b| b.iter(|| reverse_by_c()));
}

pub fn criterion_benchmark_rust(c: &mut Criterion) {
    c.bench_function("rust", |b| b.iter(|| reverse_by_rust()));
}

pub fn criterion_benchmark_unsafe_1(c: &mut Criterion) {
    c.bench_function("unsafe_1", |b| b.iter(|| reverse_unsafe_1()));
}

pub fn criterion_benchmark_unsafe_2(c: &mut Criterion) {
    c.bench_function("unsafe_2", |b| b.iter(|| reverse_unsafe_2()));
}

criterion_group!(
    benches,
    //criterion_benchmark_change_arr,
    criterion_benchmark_c,
    criterion_benchmark_rust,
    criterion_benchmark_unsafe_1,
    criterion_benchmark_unsafe_2,
);
criterion_main!(benches);
