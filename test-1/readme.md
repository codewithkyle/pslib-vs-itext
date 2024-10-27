>[!WARNING]
>Take the timing results with a 50lb bag of industrial salt. Rust should (almost) always be faster than C#.

## Round 1
| | Time | Output Size |
| - | - | - |
| pslib | 1.5s | 91mb |
| itext | 2.95s | 10mb |

## Round 2
| | Time | Output Size | gzip | brotli |
| - | - | - | - | - |
| pslib | 1.3s | 54mb | 12mb | 9.5mb |
| itext | 2.9s | 10mb | 10mb | 10mb |

#### Improvements
- Optimized pslib fill/stroke (coverted to procedures).
- Reduced whitespace.

#### Notes
I think the overall size of PostScript files will alway be larger due to the compression offered in PDF (expected?). When we gzip both files PDF does not change while the PostScript files are only slightly larger.

It's not until we compress the PostScript file with Brotli that the PostScript file becomes smaller than the PDF version.

I also think it's interesting that PDFs cannot be compressed by both gzip and brotli.

> **Note:** files compressed with `-q 11` (highest compression)
