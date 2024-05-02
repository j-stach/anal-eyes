
# anal-eyes  \* ͜\*
Attribute macro to quickly add crude debugging markers to functions. <br>

Inserts a `println!` statement following every variable declaration
and semicolon-terminated expression within the function, to assist in debugging
when a backtrace is not available or when behavior needs to be assessed at runtime.

## Use
1. Add `anal-eyes` to your project
```
$ cargo add anal_eyes
```
2. Place the attribute `#[anal_eyes]` above your troublesome function(s)
```
use anal_eyes::anal_eyes;
use rand::{ Rng, rngs::ThreadRng };

struct Junk;
impl Junk {
    #[anal_eyes]
    fn see(&self, no_evil: &mut ThreadRng) -> bool {
        no_evil.gen_range(0..3) == 3
    }
    #[anal_eyes]
    fn r#do(&self) -> Self { Junk }

    #[anal_eyes]
    fn funky(monkey: Self) -> Self {
        let mut no_evil = rand::thread_rng();
        if monkey.see(&mut no_evil) {
            monkey.r#do()
        } else {
            Self::funky(monkey)
        }
    }

}

#[anal_eyes]
fn main() {
    Junk::funky(Junk);
}

```
3. ???
```
$ cargo run
```
4. Profit
```
Executing 'main'
Executing 'funky'
 funky, declaration 1
Executing 'see'
Executing 'funky'
 funky, declaration 1
Executing 'see'
Executing 'funky'
 funky, declaration 1
Executing 'see'

{ ... }

Executing 'funky'
 funky, declaration 1
Executing 'see'

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
```

## Future directions
I'm developing this crate primarily for personal use so I'll only be expanding it
as the need arises. Feel free to fork!
- [ ] More flexibility with template formatting
- [ ] Expression values extracted & reflected in println
- [ ] Code line and column
- [ ] Tool for auto-cleanup of attributes

## Disclaimer
The name is an immature play on the word "analyze", referencing the trait of 
"anal-retentiveness" in the context of code scrutiny. <br> Don't ban me!
