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
        Average Case    12.498927602
        Best Case       0.00020131
        Worst Case      5.031648812

Heap Sort
        Average Case    0.019747255
        Best Case       0.000298749
        Worst Case      0.011353959

Insertion Sort
        Average Case    1.040045028
        Best Case       0.000202356
        Worst Case      2.188278767

Merge Sort
        Average Case    0.03847803
        Best Case       0.000186211
        Worst Case      0.025431437

Quick Sort
        Average Case    0.012571574
        Best Case       0.00046866
        Worst Case      0.00296292

Radix Sort
        Average Case    0.063946446
        Best Case       0.000187339
        Worst Case      0.016368267

Selection Sort
        Average Case    5.253504747
        Best Case       0.000342557
        Worst Case      4.6347967180000005

Shell Sort
        Average Case    0.024600298
        Best Case       0.008383005
        Worst Case      0.005389586

Benchmarks ran asynchronously in 12.501030289 seconds!
```
