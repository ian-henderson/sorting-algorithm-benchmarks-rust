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

## sample output:

```
Sorting Algorithm Benchmarks (list length: 66666)

Bubble Sort
        Average Case    5.649028088
        Best Case       0.000348441
        Worst Case      2.3756466019999998

Heap Sort
        Average Case    0.010744392
        Best Case       0.000479743
        Worst Case      0.008833129

Insertion Sort
        Average Case    0.475855201
        Best Case       0.000186461
        Worst Case      1.010570874

Merge Sort
        Average Case    0.022103039
        Best Case       0.000167711
        Worst Case      0.017434174

Quick Sort
        Average Case    0.006535983
        Best Case       0.000129839
        Worst Case      0.001882602

Radix Sort
        Average Case    0.049363798
        Best Case       0.00934979
        Worst Case      0.006938483

Selection Sort
        Average Case    2.469629011
        Best Case       0.000294187
        Worst Case      2.157265563

Benchmarks ran asynchronously in 5.650625458 seconds!
```
