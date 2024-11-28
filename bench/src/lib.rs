wit_bindgen::generate!(in "../wit");

use component::bench::types::RustData;

static CHARSET: &[u8] = b"0123456789abcdef";

fn generate_string(len: usize) -> String {
    (0..len)
        .map(|_| CHARSET[rand(CHARSET.len() as u32) as usize] as char)
        .collect()
}

fn partition(arr: &mut [RustData]) -> usize {
    let (lo, hi) = (0, arr.len() - 1);
    let pivot_idx = (lo + hi) / 2;
    arr.swap(pivot_idx, hi);
    let Some((pivot, arr)) = arr.split_last_mut() else {
        return 0;
    };
    let mut j = lo;
    for i in lo..hi {
        if arr[i].lt(pivot) {
            arr.swap(i, j);
            j += 1;
        }
    }
    if let Some(arr_j) = arr.get_mut(j) {
        std::mem::swap(arr_j, pivot);
    }
    j
}

fn quicksort(mut arr: &mut [RustData]) {
    while !arr.is_empty() {
        let p = partition(arr);
        quicksort(&mut arr[..p.saturating_sub(1)]);
        // Tail recursion
        arr = &mut arr[p + 1..];
    }
}

struct Bench;

impl Guest for Bench {
    fn bench() {
        let mut array = (0..10_000)
            .map(|_| RustData::new(&generate_string(rand(16) as usize + 8)))
            .collect::<Vec<_>>();
        quicksort(&mut array);
    }
}

export!(Bench);
