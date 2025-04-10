#![cfg(target_arch = "wasm32")]
#![no_main]

use std::cell::RefCell;

#[link(wasm_import_module = "RustData")]
unsafe extern "C" {
    safe fn rustdata_new(ptr: u32, len: u32) -> u32;
    safe fn rustdata_delete(id: u32) -> ();
    safe fn rustdata_lt(this: u32, other: u32) -> u32;
    safe fn rand(limit: u32) -> u32;
}

struct RustData(u32);

impl RustData {
    fn new(s: &str) -> Self {
        RustData(rustdata_new(s.as_ptr() as u32, s.len() as u32))
    }

    fn lt(&self, other: &Self) -> bool {
        rustdata_lt(self.0, other.0) != 0
    }
}

impl Drop for RustData {
    fn drop(&mut self) {
        rustdata_delete(self.0);
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
        quicksort(&mut arr[..p]);
        // Tail recursion
        arr = &mut arr[p + 1..];
    }
}

// Global storage to get access the sorted array
thread_local! {
    static SORTED_ARRAY: RefCell<Vec<RustData>> = RefCell::new(Vec::new());
}

#[no_mangle]
pub fn bench(store: u32) -> u32 {
    let length = 10_000;
    let mut array = (0..length)
        .map(|_| RustData::new(&generate_string(rand(16) as usize + 8)))
        .collect::<Vec<_>>();
    quicksort(&mut array);

    if store != 0 {
        SORTED_ARRAY.with(|arr| {
            *arr.borrow_mut() = array;
        });
    }
    length as u32
}

// Get the ID at specific index in the sorted array
// Used only at validation step
#[no_mangle]
pub fn get_id(index: u32) -> u32 {
    SORTED_ARRAY.with(|cell| cell.borrow()[index as usize].0)
}
