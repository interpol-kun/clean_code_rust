Rust implementation of the Casey's Muratori example from the ["Clean" Code, Horrible Performance article][clean-code]

Although the first benchmark with VTBL is unstable (it varies from 2.8 ms to ~600 µs), the following picture looks like this:

GS (General Struct)

**Benchmark uses the array of 400 000 structs in a for-loop**

| Method                                         	| Time      	| Difference from baseline 	|
|------------------------------------------------	|-----------	|--------------------------	|
| Dynamic dispatch                               	| 632.34 µs 	| 100%                     	|
| GS with enum field + match                     	| 379.52 µs 	| 60.01%                   	|
| GS with enum field + match + lookup table     	| 340.19 µs 	| 53.79%                   	|
| Rust enums                                        | 366.25 µs 	| 57.91%                   	|

**It is worth noting that both trait methods and pure methods have the same performance whith the static dispatch, so I deleted one of the benchmarks**

[clean-code]: https://www.computerenhance.com/p/clean-code-horrible-performance