# sorting algorithm benchmarks

## requirements
- `cargo`
Set up rust and cargo by running the installation script [here](https://www.rust-lang.org/tools/install).

## run
- run `cargo r <optional list length int value>`

## sorting algorithms
- Bubble Sort
- Heap Sort
- Insertion Sort
- Merge Sort
- Quick Sort
- Radix Sort
- Selection Sort
- Shell Sort

## sample output:

### command: `cargo run --release 100000`

```
Sorting Algorithm Benchmarks (list length: 100000)

Bubble Sort
        Average Case    12.487627137
        Best Case       0.000222747
        Worst Case      5.066368709

Heap Sort
        Average Case    0.018291214
        Best Case       0.000252006
        Worst Case      0.01063478

Insertion Sort
        Average Case    1.125501204
        Best Case       0.000234803
        Worst Case      2.197871822

Merge Sort
        Average Case    0.031143804
        Best Case       0.00024912
        Worst Case      0.02351766

Quick Sort
        Average Case    0.009799731
        Best Case       0.000216063
        Worst Case      0.002679227

Radix Sort
        Average Case    0.077469944
        Best Case       0.000220508
        Worst Case      0.012302892

Selection Sort
        Average Case    5.325710678
        Best Case       0.000707082
        Worst Case      4.73135191

Shell Sort
        Average Case    0.016538554
        Best Case       0.000295325
        Worst Case      0.013123315

Benchmarks ran asynchronously in 12.489135133 seconds!
```
