use rand::Rng;


fn main() {

    let mut arr = vec![];
    let mut i = 0;
    let mut rng = rand::thread_rng();
    while i < 100000000 {
        arr.push(rng.gen::<i32>());
        i += 1;
    };

    let len = arr.len();

    let start: u128 = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_millis();

    quicksort(&mut arr, 0, len-1);
    
    let end: u128 = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_millis();


    println!("Sorted: {}. \n milis: {}", verify(&arr), end - start);
}

fn partition(arr: &mut [i32], start: usize, end: usize) -> usize {
    let pivot = arr[end];
    let mut i = start;
    let mut k = start;
    let mut first_swap = true;
    while i < end {
        if arr[i] < pivot {
            // usize cannot be negative so we simulate our first swap
            if !first_swap {
                k += 1;
            } else {
                first_swap = false;
            }
            // Swap
            let temp = arr[k];
            arr[k] = arr[i];
            arr[i] = temp;
        }
        i += 1;
    }
    // Swap pivot into final place.
    if !first_swap {
        k += 1;
    }
    arr[end] = arr[k];
    arr[k] = pivot;

    return k;
}

fn quicksort(arr: &mut [i32], start: usize, end: usize) {
    if start < end {
        let sorted_index = partition(arr, start, end);
        if sorted_index != 0 {
            quicksort(arr, start, sorted_index - 1);
        }
        quicksort(arr, sorted_index + 1, end);
    }
}


fn verify(arr : &[i32]) -> bool {
    let len = arr.len();
    let mut i = 1;
    while i < len {
        if arr[i] < arr[i - 1] {
            return false;
        }
        i += 1;
    }
    true
}