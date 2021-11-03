use crate::dbgt;

pub fn quick_sort<T>(arr: &mut [T], lo: usize, hi: usize)
where
    T: Copy + PartialOrd,
{
    let i = _quick_sort(arr, lo, hi);
    if lo < i - 1 {
        quick_sort(arr, lo, i - 1);
    }
    if i + 1 < hi {
        quick_sort(arr, i + 1, hi);
    }
}

fn _quick_sort<T>(arr: &mut [T], mut lo: usize, mut hi: usize) -> usize
where
    T: Copy + PartialOrd,
{
    let to_arrange: T = arr[lo];
    while lo < hi {
        while lo < hi && arr[hi] >= to_arrange {
            hi -= 1;
        }
        if lo == hi {
            break;
        }
        arr[lo] = arr[hi];
        lo += 1;
        while lo < hi && arr[lo] <= to_arrange {
            lo += 1;
        }
        arr[hi] = arr[lo];
        hi -= 1;
    }
    arr[lo] = to_arrange;
    lo
}

#[test]
fn test() {
    let mut a: [i32; 20] = [
        23, 33, 6, 84, 70, 29, 57, 43, 47, 18, 63, 62, 24, 74, 23, 87, 57, 58, 50, 92,
    ];
    let mut b = Vec::from(a.clone());
    let n_1 = a.len() - 1;
    quick_sort(&mut a, 0, n_1);
    quick_sort(&mut b, 0, n_1);

    dbgt!(&a);
    dbgt!(&b);
}
