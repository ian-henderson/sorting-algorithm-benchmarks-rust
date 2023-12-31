use crate::sorting::{
    bubble_sort, gnome_sort, heap_sort, insertion_sort, merge_sort, quick_sort, radix_sort,
    selection_sort, shell_sort,
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
            name: "Gnome Sort".to_string(),
            sort_fn: gnome_sort,
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
            name: "Radix Sort".to_string(),
            sort_fn: radix_sort,
        },
        TestConfig {
            name: "Selection Sort".to_string(),
            sort_fn: selection_sort,
        },
        TestConfig {
            name: "Shell Sort".to_string(),
            sort_fn: shell_sort,
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
        .map(|test_result_handles| TestResult {
            name: test_result_handles.name,
            average_case_duration: test_result_handles.average_case_duration_handle.join(),
            best_case_duration: test_result_handles.best_case_duration_handle.join(),
            worst_case_duration: test_result_handles.worst_case_duration_handle.join(),
        });

    let benchmark_duration = timer.stop();

    test_results.into_iter().for_each(print_test_result);

    println!(
        "Benchmarks ran parallelly in {} seconds!",
        benchmark_duration.as_secs_f64()
    );
}
