#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link(wasm_import_module = "RustData")]
extern "C" {
    fn new(ptr: i32, len: i32) -> i32;
    fn lt(ptr: i32, len: i32) -> i32;
    fn clear() -> ();
    fn rand(limit: i32) -> i32;
}

#[derive(Copy, Clone)]
struct RustData(i32);

impl RustData {
    fn new(s: &str) -> Self {
        RustData(unsafe { new(s.as_ptr() as i32, s.len() as i32) })
    }
    fn lt(self, other: Self) -> bool {
        unsafe { lt(self.0, other.0) != 0 }
    }
    fn clear() {
        unsafe { clear() }
    }
    fn rand(n: i32) -> i32 {
        unsafe { rand(n) }
    }
}

fn generate_string(buf: &mut [u8; 32]) -> &str {
    let charset = b"0123456789abcdef";
    let len = (RustData::rand(16) + 8) as usize & 31;
    for b in &mut buf[..len] {
        *b = charset[RustData::rand(16) as usize & 15]
    }
    // SAFETY: charset only contains single-byte codepoints
    unsafe { core::str::from_utf8_unchecked(&buf[..len]) }
}

fn partition(arr: &mut [RustData]) -> usize {
    let (lo, hi) = (0, arr.len() - 1);
    let pivot_idx = (lo + hi) >> 1;
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
    RustData::clear();
    let mut array = [(); 10_000].map(|_| {
        let mut buf = [0; 32];
        RustData::new(generate_string(&mut buf))
    });
    quicksort(&mut array[..]);
}
