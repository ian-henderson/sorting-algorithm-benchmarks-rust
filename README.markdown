# sorting algorithm benchmarks

## requirements
- `cargo`
Set up rust and cargo by running the installation script [here](https://www.rust-lang.org/tools/install).

## run
- run `cargo r <optional list length int value>`

## sorting algorithms
- Bubble Sort
- Gnome Sort
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

Bubble Sort     Average Case:   12.322887609 seconds
                Best Case:      0.000118932 seconds
                Worst Case:     6.494159823 seconds

Gnome Sort      Average Case:   7.042260819 seconds
                Best Case:      0.000407938 seconds
                Worst Case:     12.716790014 seconds

Heap Sort       Average Case:   0.019971534 seconds
                Best Case:      0.000650108 seconds
                Worst Case:     0.012878383 seconds

Insertion Sort  Average Case:   1.350668601 seconds
                Best Case:      0.0002065 seconds
                Worst Case:     3.377137406 seconds

Merge Sort      Average Case:   0.04051903 seconds
                Best Case:      0.000334179 seconds
                Worst Case:     0.033061395 seconds

Quick Sort      Average Case:   0.017470633 seconds
                Best Case:      0.000414442 seconds
                Worst Case:     0.002368494 seconds

Radix Sort      Average Case:   0.071867771 seconds
                Best Case:      0.000738941 seconds
                Worst Case:     0.010871847 seconds

Selection Sort  Average Case:   6.009173841 seconds
                Best Case:      0.000180396 seconds
                Worst Case:     4.557693024 seconds

Shell Sort      Average Case:   0.020189003 seconds
                Best Case:      0.00017407 seconds
                Worst Case:     0.005070876 seconds

Benchmarks ran parallelly in 12.721030739 seconds!
```
