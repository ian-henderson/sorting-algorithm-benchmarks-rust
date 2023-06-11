use crate::sorting::{
    bubble_sort, heap_sort, insertion_sort, merge_sort, quick_sort, selection_sort,
};
use crate::utils::{
    create_test_vecs, get_array_size, print_test_result, run_test, TestConfig, TestResult,
    TestResultHandles, TestVecs, Timer,
};
use std::thread;
pub mod sorting;
pub mod utils;

fn main() {
    let vec_size = get_array_size();

    println!("Sorting Algorithm Benchmarks (list length: {vec_size})\n");

    let TestVecs {
        average_case_vec,
        best_case_vec,
        worst_case_vec,
    } = create_test_vecs(vec_size);

    let test_configs = [
        TestConfig {
            name: "Bubble Sort".to_string(),
            sort_fn: bubble_sort,
        },
        TestConfig {
            name: "Heap Sort".to_string(),
            sort_fn: heap_sort,
        },
        TestConfig {
            name: "Insertion Sort".to_string(),
            sort_fn: insertion_sort,
        },
        TestConfig {
            name: "Merge Sort".to_string(),
            sort_fn: merge_sort,
        },
        TestConfig {
            name: "Quick Sort".to_string(),
            sort_fn: quick_sort,
        },
        TestConfig {
            name: "Selection Sort".to_string(),
            sort_fn: selection_sort,
        },
    ];

    let mut timer = Timer::new();

    let test_results = test_configs
        .map(|test_config| {
            let mut average_case_vec = average_case_vec.clone();
            let mut best_case_vec = best_case_vec.clone();
            let mut worst_case_vec = worst_case_vec.clone();
            let test_config_clone_0 = test_config.clone();
            let test_config_clone_1 = test_config.clone();
            let test_config_clone_2 = test_config.clone();

            TestResultHandles {
                name: test_config.name,
                average_case_duration_handle: thread::spawn(move || {
                    run_test(test_config_clone_0, &mut average_case_vec)
                }),
                best_case_duration_handle: thread::spawn(move || {
                    run_test(test_config_clone_1, &mut best_case_vec)
                }),
                worst_case_duration_handle: thread::spawn(move || {
                    run_test(test_config_clone_2, &mut worst_case_vec)
                }),
            }
        })
        .map(
            |TestResultHandles {
                 name,
                 average_case_duration_handle,
                 best_case_duration_handle,
                 worst_case_duration_handle,
             }| TestResult {
                name,
                average_case_duration: average_case_duration_handle.join(),
                best_case_duration: best_case_duration_handle.join(),
                worst_case_duration: worst_case_duration_handle.join(),
            },
        );

    let benchmark_duration = timer.stop();

    test_results.into_iter().for_each(print_test_result);

    println!(
        "Benchmarks ran asynchronously in {} seconds!",
        benchmark_duration.as_secs_f64()
    );
}
