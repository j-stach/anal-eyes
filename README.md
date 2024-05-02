
# anal-eyes  \*_\*
Attribute macro to quickly add crude debugging markers to functions. <br>

Inserts a `println!` statement following every variable declaration
and semicolon-terminated expression within the function, to assist in debugging
when a backtrace is not available or when behavior needs to be assessed at runtime.

## Use
Add anal-eyes to the project:
```
cargo add anal_eyes
```
Insert the attribute `#[anal_eyes]` above your troublesome function(s):
```
#[anal_eyes]
fn funky(monkey: Self) -> Self {
    if monkey.see() {
        monkey.r#do()
    } else {
        funky(monkey)
    }
}
```
Profit. See `examples/funky_junk.rs` for a more complete example.


## Future directions
I'm developing this crate primarily for personal use so I'll only be expanding it
as the need arises. If you have any ideas in the meantime, feel free to fork!
- [ ] Convention for nested function declarations
- [ ] More flexibility with template formatting
- [ ] Expression values reflected in println invoction

## Disclaimer
The name has a dual-meaning; it is a childish play on the word "analyze", referencing
the trait of "anal-retentiveness" in the context of code scrutiny. Don't ban me!
