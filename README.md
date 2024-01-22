# range-overlap

A set of methods to determine if and how two ranges of values overlap.

## Versioning

The version will be kept < 0.1.0 until we have enough real-world use to
be confident edge cases are handled correctly. 

## Examples

The simplest use is when you only want to know whether or not two ranges
that have both ends defined overlap:

```rust
use range_overlap::*;

// Does the range 0 to 10 overlap with the range 5 to 15?
assert_eq!(has_incl_overlap(0, 10, 5, 15), true);
// What above 11 to 20?
assert_eq!(has_incl_overlap(0, 10, 11, 20), false);

// Depending on the case, you can include or exclude the ends of the ranges:
assert_eq!(has_incl_overlap(0, 10, 10, 20), true);
assert_eq!(has_excl_overlap(0, 10, 10, 20), false);
```

If it matters *how* the two ranges overlap, then other methods can determine that:

```rust
// Check if the second range (5 to 10) is entirely inside the first:
assert_eq!(excl_classify(0, 20, 5, 10), RangeOverlap::AContainsB);
assert_neq!(excl_classify(0, 20, 5, 21), RangeOverlap::AContainsB);

// Check if the one range starts or ends inside the other:
assert_eq!(excl_classify(10, 20, 5, 15), RangeOverlap::AStartsInB);
assert_eq!(excl_classify(0, 10, 5, 15), RangeOverlap::AEndsInB);

// Check if the two ranges are the same:
assert_eq!(excl_classify(0, 10, 0, 10), RanegOverlap::AEqualsB);
```

Finally, if you have ranges which are open on one or both sides, 
those can be handled too:

```rust
// A is fully open - no start or end
assert_eq!(classify_any(None, None, Some(0), Some(10), false), RangeOverlap::AContainsB);

// A has no start, B has no end:
assert_eq!(classify_any(None, Some(10), Some(5), None, false), RangeOverlap::AEndsInB);
```