let charset = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
let generate_string = |len| {
    let data = [];
    for i in 0..len {
        data.push(charset[rand(charset.len)]);
    }
    return concat(data);
};

fn swap(i, j) {
    let t = this[i];
    this[i] = this[j];
    this[j] = t;
}

fn partition(lo, hi) {
    let pivot_idx = (lo + hi) / 2;
    let pivot = this[pivot_idx];
    this.swap(pivot_idx, hi);
    let j = lo;
    for i in lo..hi {
        if this[i] < pivot {
            this.swap(i, j);
            j = j + 1;
        }
    }
    this.swap(j, hi);
    return j;
}

fn quicksort(lo, hi) {
    while lo < hi {
        let p = this.partition(lo, hi);
        this.quicksort(lo, p - 1);
        // Tail recursion
        lo = p + 1;
    }
}

let array = [];
for i in 0..10000 {
    array.push(RustData_new(generate_string.call(8 + rand(16))));
}
array.quicksort(0, array.len - 1);
return array;
