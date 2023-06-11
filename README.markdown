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
- Selection Sort

## sample output:

```
Sorting Algorithm Benchmarks (list length: 66666)

Bubble Sort
        Average Case    6.728099895
        Best Case       0.000199725
        Worst Case      3.686846555

Heap Sort
        Average Case    0.011889215
        Best Case       0.000224784
        Worst Case      0.009526588

Insertion Sort
        Average Case    0.535928447
        Best Case       0.000188315
        Worst Case      1.114860071

Merge Sort
        Average Case    0.026658533
        Best Case       0.000139282
        Worst Case      0.021818274

Quick Sort
        Average Case    0.008514629
        Best Case       0.00012705
        Worst Case      0.002264807

Selection Sort
        Average Case    2.547715437
        Best Case       0.000166546
        Worst Case      2.253252412

Benchmarks ran asynchronously in 6.730001506 seconds!
```
