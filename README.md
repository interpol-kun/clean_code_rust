Rust implementation of the Casey's Muratori example from the ["Clean" Code, Horrible Performance article][clean-code]

The following picture looks like this:

GS (General Struct)

**Benchmark uses the array of 400 000 structs in a for-loop**

**Method**|**Time**|**Difference from baseline**
:-----:|:-----:|:-----:
Dynamic dispatch + shuffled|3972 ± 38 µs|1.000x
Dynamic dispatch|2730 ± 30 µs|1.455x
Dynamic dispatch + strided|1870 ± 12 µs|2.124x
GS with enum field + match|2704 ± 30 µs|1.469x
GS with enum field + match + strided|526 ± 3 µs|7.551x
GS with enum field + lookup table|289 ± 1 µs|13.744x
GS with enum field + lookup table + strided|291 ± 1 µs|13.649x
Rust enums|2565 ± 20 µs|1.549x
Rust enums + strided|437 ± 5 µs|9.089x
Static promotion|564 ± 1 µs|7.042x
Separated data|286 ± 1 µs|13.888x

**It is worth noting that both trait methods and pure methods have the same performance whith the static dispatch, so I deleted one of the benchmarks**

[clean-code]: https://www.computerenhance.com/p/clean-code-horrible-performance
