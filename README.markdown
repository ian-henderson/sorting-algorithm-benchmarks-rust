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
Sorting Algorithm Benchmarks (list length: 100000)

Bubble Sort
        Average Case    12.150150156
        Best Case       0.00012432
        Worst Case      4.867759848

Heap Sort
        Average Case    0.016824272
        Best Case       0.000252409
        Worst Case      0.01163086

Insertion Sort
        Average Case    1.038506017
        Best Case       0.000173572
        Worst Case      2.150144329

Merge Sort
        Average Case    0.028286003
        Best Case       0.000177912
        Worst Case      0.021096375

Quick Sort
        Average Case    0.010165404
        Best Case       0.000198268
        Worst Case      0.002483986

Radix Sort
        Average Case    0.073244186
        Best Case       0.000180406
        Worst Case      0.009725553

Selection Sort
        Average Case    5.170163335
        Best Case       0.000301345
        Worst Case      4.53579333

Benchmarks ran asynchronously in 12.15186369 seconds!
```
