Rust implementation of the Casey's Muratori example from the ["Clean" Code, Horrible Performance article][clean-code]

Although the first benchmark with VTBL is unstable (somehow I managed to get to ~600µs in a few runs), the following picture looks like this:

GS (General Struct)

| Method                                         	| Time      	| Difference from baseline 	|
|------------------------------------------------	|-----------	|--------------------------	|
| Dynamic dispatch                               	| 2.8753 ms 	| 100%                     	|
| GS with enum field + match                     	| 379.52 µs 	| 13.18%                   	|
| GS with enum field + match + coefficient array 	| 340.19 µs 	| 11.82%                   	|
| Rust enums                                     	| 480.07 µs 	| 16.69%                   	|

[clean-code]: https://www.computerenhance.com/p/clean-code-horrible-performance