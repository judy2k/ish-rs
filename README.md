# Ish

Sometimes you want to compare something to true or false,
but you're not so much interested if it's *true* or *false*,
so much as whether it's true-ish, or false-ish.

Anyway, thanks to Rust's amazing ability to override the [hyphen operator](https://doc.rust-lang.org/std/ops/trait.Sub.html),
here it is - the Ish library, ported to Rust.

First you need a true-ish or a false-ish value.
And you can get that really easily using the hyphen `-` operator.
Once you have one, you can compare it to booleans, strings, and integers:

```rust
use ish::ish;

// Any non-zero integer is considered to be true-ish, and not false-ish.
1 == true-ish           // true!
1 == false-ish          // false!
0 == true-ish           // false!
0 == false-ish          // true!

// The word "true" in various different forms is true-ish:
"true" == true-ish      // true!
"TRUE" == true-ish      // true!
"not true" == true-ish  // false!
"snooker" == true-ish   // false!


// And the word "false" in various different forms is false-ish:
"false" == false-ish     // true!
"FALSE" == false-ish     // true!
"ferrets" == false-ish   // false!
```


## History

A long time ago I wrote a Python library called [ish](https://github.com/judy2k/ish),
and gave a talk about it at EuroPython.
People liked it.
Well, actually they *hated* it,
but they thought it was funny.

Anyway,
I've been writing quite a bit of Rust recently,
and it just occurred to me that I could port [ish](https://github.com/judy2k/ish) to Rust