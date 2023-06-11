use std::{
    clone::Clone,
    thread::{JoinHandle, Result},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct TestConfig {
    pub name: String,
    pub sort_fn: fn(&mut Vec<usize>) -> &Vec<usize>,
}

pub struct TestResult {
    pub name: String,
    pub average_case_duration: Result<Duration>,
    pub best_case_duration: Result<Duration>,
    pub worst_case_duration: Result<Duration>,
}

pub struct TestResultHandles {
    pub name: String,
    pub average_case_duration_handle: JoinHandle<Duration>,
    pub best_case_duration_handle: JoinHandle<Duration>,
    pub worst_case_duration_handle: JoinHandle<Duration>,
}

pub struct TestVecs {
    pub average_case_vec: Vec<usize>,
    pub best_case_vec: Vec<usize>,
    pub worst_case_vec: Vec<usize>,
}

pub struct Timer {
    now: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            now: Instant::now(),
        }
    }

    pub fn stop(&mut self) -> Duration {
        self.now.elapsed()
    }
}

pub fn create_test_vecs(vec_size: usize) -> TestVecs {
    TestVecs {
        average_case_vec: (0..vec_size).map(|_| rand::random()).collect(),
        best_case_vec: (0..vec_size).collect(),
        worst_case_vec: (0..vec_size).rev().collect(),
    }
}

pub fn get_array_size() -> usize {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        args[1].parse().expect("Expected input to be an integer.")
    } else {
        10_000
    }
}

pub fn print_test_result(
    TestResult {
        name,
        average_case_duration,
        best_case_duration,
        worst_case_duration,
    }: TestResult,
) {
    println!("{}", name);

    print!("\tAverage Case\t");
    match average_case_duration {
        Ok(duration) => println!("{}", duration.as_secs_f64()),
        Err(error) => eprintln!("{:?}", error),
    };

    print!("\tBest Case\t");
    match best_case_duration {
        Ok(duration) => println!("{}", duration.as_secs_f64()),
        Err(error) => eprintln!("{:?}", error),
    };

    print!("\tWorst Case\t");
    match worst_case_duration {
        Ok(duration) => println!("{}", duration.as_secs_f64()),
        Err(error) => eprintln!("{:?}", error),
    };

    println!();
}

pub fn run_test(config: TestConfig, vec: &mut Vec<usize>) -> Duration {
    let start = Instant::now();
    (config.sort_fn)(vec);
    let end = Instant::now() - start;

    end
}
