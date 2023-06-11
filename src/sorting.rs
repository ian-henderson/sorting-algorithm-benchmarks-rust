// https://en.wikipedia.org/wiki/Bubble_sort
pub fn bubble_sort(vec: &mut Vec<usize>) -> &Vec<usize> {
    let mut n = vec.len();

    while n > 1 {
        let mut new_n = 0;

        for i in 1..n {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
                new_n = i;
            }
        }

        n = new_n;
    }

    vec
}

// https://en.wikipedia.org/wiki/Heapsort
pub fn heap_sort(vec: &mut Vec<usize>) -> &Vec<usize> {
    if is_best_case_vec(vec) {
        return vec;
    }

    // build heap (rearrange array)
    // root needs to be a signed integer to work with this while loop
    let mut root: isize = (vec.len() as isize) / 2 - 1;
    while root >= 0 {
        heapify(vec, vec.len(), root as usize);
        root -= 1;
    }

    // one by one, extract the element from the heap
    let mut i = vec.len() - 1;
    while i > 0 {
        // move current root to end
        vec.swap(0, i);

        // call max heapify on the reduced heap
        heapify(vec, i, 0);

        i -= 1;
    }

    vec
}

fn heapify(vec: &mut Vec<usize>, heap_size: usize, root: usize) {
    let left = 2 * root + 1;
    let right = 2 * root + 2;
    let mut largest = root;

    // if left child is larger than root
    if left < heap_size && vec[left] > vec[largest] {
        largest = left;
    }

    // if right child is larger than largest so far
    if right < heap_size && vec[right] > vec[largest] {
        largest = right;
    }

    // if largest is not root
    if largest != root {
        vec.swap(root, largest);

        // recursively heapify the affected sub-tree
        heapify(vec, heap_size, largest);
    }
}

// https://en.wikipedia.org/wiki/Insertion_sort
pub fn insertion_sort(vec: &mut Vec<usize>) -> &Vec<usize> {
    for i in 1..vec.len() {
        let key = vec[i];
        let mut j = i;

        while j > 0 && vec[j - 1] > key {
            vec[j] = vec[j - 1];
            j -= 1;
        }

        vec[j] = key;
    }

    vec
}

fn is_best_case_vec(vec: &Vec<usize>) -> bool {
    let mut best_case = true;

    for i in 0..(vec.len() - 1) {
        if vec[i] > vec[i + 1] {
            best_case = false;
            break;
        }
    }

    best_case
}

// https://en.wikipedia.org/wiki/Merge_sort
pub fn merge_sort(vec: &mut Vec<usize>) -> &Vec<usize> {
    if is_best_case_vec(vec) {
        return vec;
    }

    merge_sort_r(vec, 0, vec.len() - 1);

    vec
}

fn merge_sort_r(vec: &mut Vec<usize>, begin: usize, end: usize) {
    if begin >= end {
        return;
    }

    let mid = begin + (end - begin) / 2;
    merge_sort_r(vec, begin, mid);
    merge_sort_r(vec, mid + 1, end);
    merge(vec, begin, mid, end);
}

fn merge(vec: &mut Vec<usize>, left: usize, mid: usize, right: usize) {
    let sub_array_one = mid - left + 1;
    let sub_array_two = right - mid;

    let mut left_vec: Vec<usize> = vec![0; sub_array_one];
    let mut right_vec: Vec<usize> = vec![0; sub_array_two];

    // copy data into vecs
    for i in 0..sub_array_one {
        left_vec[i] = vec[left + i];
    }
    for i in 0..sub_array_two {
        right_vec[i] = vec[mid + i + 1];
    }

    let mut index_of_sub_array_one = 0;
    let mut index_of_sub_array_two = 0;
    let mut index_of_merged_array = left;

    // merge the temp arrays back into array[left..right]
    while index_of_sub_array_one < sub_array_one && index_of_sub_array_two < sub_array_two {
        if left_vec[index_of_sub_array_one] <= right_vec[index_of_sub_array_two] {
            vec[index_of_merged_array] = left_vec[index_of_sub_array_one];
            index_of_sub_array_one += 1;
        } else {
            vec[index_of_merged_array] = right_vec[index_of_sub_array_two];
            index_of_sub_array_two += 1;
        }
        index_of_merged_array += 1;
    }

    // copy the remaining elements of left_vec if there are any
    while index_of_sub_array_one < sub_array_one {
        vec[index_of_merged_array] = left_vec[index_of_sub_array_one];
        index_of_merged_array += 1;
        index_of_sub_array_one += 1;
    }

    // copy the remaining elements of right_vec if there are any
    while index_of_sub_array_two < sub_array_two {
        vec[index_of_merged_array] = right_vec[index_of_sub_array_two];
        index_of_merged_array += 1;
        index_of_sub_array_two += 1;
    }
}

// https://en.wikipedia.org/wiki/Quicksort
pub fn quick_sort(vec: &mut Vec<usize>) -> &Vec<usize> {
    if is_best_case_vec(vec) {
        return vec;
    }

    quick_sort_r(vec, 0, vec.len() - 1);

    vec
}

fn quick_sort_r(vec: &mut Vec<usize>, low: usize, high: usize) {
    if low < high {
        let partition_index = partition(vec, low, high);
        quick_sort_r(vec, low, partition_index);
        quick_sort_r(vec, partition_index + 1, high);
    }
}

// Divides an array into two partitions
fn partition(vec: &mut Vec<usize>, low: usize, high: usize) -> usize {
    // the value in the middle of the array
    let pivot = vec[(high - low) / 2 + low];

    let mut i = low;
    let mut j = high;

    loop {
        // move the left index to the right at least once and while the
        // element at the left index is less than the pivot
        while vec[i] < pivot {
            i += 1;
        }

        // move the right index to the left at least once and while the
        // element at the right index is greater than the pivot
        while vec[j] > pivot {
            j -= 1;
        }

        // if the indices cross, return
        if i >= j {
            return j;
        }

        // swap the elements at the left and right indices
        vec.swap(i, j);
    }
}

// https://en.wikipedia.org/wiki/Selection_sort
pub fn selection_sort(vec: &mut Vec<usize>) -> &Vec<usize> {
    if is_best_case_vec(vec) {
        return vec;
    }

    for i in 0..(vec.len() - 1) {
        let mut smallest_index = i;

        for j in (i + 1)..vec.len() {
            if vec[j] < vec[smallest_index] {
                smallest_index = j;
            }
        }

        if i != smallest_index {
            vec.swap(i, smallest_index);
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        test_sorting_fn(bubble_sort);
    }

    #[test]
    fn test_heap_sort() {
        test_sorting_fn(heap_sort);
    }

    #[test]
    fn test_insertion_sort() {
        test_sorting_fn(insertion_sort);
    }

    #[test]
    fn test_merge_sort() {
        test_sorting_fn(merge_sort);
    }

    #[test]
    fn test_quick_sort() {
        test_sorting_fn(quick_sort);
    }

    #[test]
    fn test_selection_sort() {
        test_sorting_fn(selection_sort);
    }

    fn test_sorting_fn(sort_fn: fn(&mut Vec<usize>) -> &Vec<usize>) {
        let mut vec: Vec<usize> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        sort_fn(&mut vec);

        for i in 0..(vec.len() - 1) {
            assert!(vec[i] < vec[i + 1]);
        }
    }
}
