>[!WARNING]
>Take the timing results with a 50lb bag of industrial salt. Rust should (almost) always be faster than C#.

## Round 1
| | Time | Output Size |
| - | - | - |
| pslib | 1.5s | 91mb |
| itext | 2.95s | 10mb |

## Round 2
| | Time | Output Size |
| - | - | - |
| pslib | 1.3s | 54mb |
| itext | 2.9s | 10mb |

**Improvements**
- Optimized pslib fill/stroke (coverted to procedures).
- Reduced whitespace.
