#![cfg(target_arch = "wasm32")]
#![no_main]

#[link(wasm_import_module = "RustData")]
unsafe extern "C" {
    safe fn rustdata_new(ptr: u32, len: u32) -> u32;
    safe fn rustdata_lt(this: u32, other: u32) -> u32;
    safe fn rand(limit: u32) -> u32;
}

#[derive(Copy, Clone)]
struct RustData(u32);

impl RustData {
    fn new(s: &str) -> Self {
        RustData(rustdata_new(s.as_ptr() as u32, s.len() as u32))
    }

    fn lt(self, other: Self) -> bool {
        rustdata_lt(self.0, other.0) != 0
    }
}

static CHARSET: &[u8] = b"0123456789abcdef";

fn generate_string(len: usize) -> String {
    (0..len)
        .map(|_| CHARSET[rand(CHARSET.len() as u32) as usize] as char)
        .collect()
}

fn partition(arr: &mut [RustData]) -> usize {
    let (lo, hi) = (0, arr.len() - 1);
    let pivot_idx = (lo + hi) / 2;
    let pivot = arr[pivot_idx];
    arr.swap(pivot_idx, hi);
    let mut j = lo;
    for i in lo..hi {
        if arr[i].lt(pivot) {
            arr.swap(i, j);
            j += 1;
        }
    }
    arr.swap(j, hi);
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

#[no_mangle]
pub fn bench() {
    let mut array = (0..10_000)
        .map(|_| RustData::new(&generate_string(rand(16) as usize + 8)))
        .collect::<Vec<_>>();
    quicksort(&mut array);
}
