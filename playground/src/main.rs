use rand::Rng;
//mod timeit;

const AAA: [[i32; 5]; 2] = [[5, 9, 3, 8, 7], [4, 1, 6, 2, 0]];

fn main() {
    let mut rng = rand::thread_rng();

    // an unbiased integer over the entire range:
    let _i: i32 = rng.gen();
    // a uniformly distributed value between 0 and 1:
    let _x: f64 = rng.gen();
    // simulate rolling a die:
    let _roll = rng.gen_range(1..7);

    // copy
    let mut arr = AAA[0];
    arr.reverse();
    dbg!(arr);
    dbg!(AAA[0]);

    // timeit!(10, {
    //     let mut arr = [0; 10000];
    //     rand::thread_rng().fill(&mut arr[..]);
    // });
}
