
use anal_eyes::anal_eyes;
use rand::{ Rng, rngs::ThreadRng };


struct Junk;
impl Junk {
    #[anal_eyes]
    fn see(&self, no_evil: &mut ThreadRng) -> bool {
        no_evil.gen_range(0..3) == 2
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
