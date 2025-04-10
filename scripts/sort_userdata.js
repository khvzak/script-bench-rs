const charset = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
function generate_string(len) {
    let data = new Array(len);
    for (let i = 0; i < len; i++) {
        data.push(charset[rand(charset.length)]);
    }
    return data.join("");
}

function swap(arr, i, j) {
    const t = arr[i];
    arr[i] = arr[j];
    arr[j] = t;
}

function partition(arr, lo, hi) {
    let pivot_idx = Math.floor((lo + hi) / 2);
    let pivot = arr[pivot_idx];
    swap(arr, pivot_idx, hi);
    let j = lo;
    for (let i = lo; i < hi; i++) {
        if (arr[i].lt(pivot)) {
            swap(arr, i, j);
            j++;
        }
    }
    swap(arr, j, hi);
    return j;
}

function quicksort(arr, lo, hi) {
    while (lo < hi) {
        let p = partition(arr, lo, hi);
        quicksort(arr, lo, p - 1);
        // Tail recursion
        lo = p + 1;
    }
}

function bench() {
    let array = [];
    for (let i = 0; i < 10000; i++) {
        array.push(new RustData(generate_string(8 + rand(16))));
    }
    quicksort(array, 0, array.length - 1);
    return array;
}
