extern crate rand;

use rand::{Rng, thread_rng};
use std::usize;

pub struct Iterate<A> {
    func: A
}

impl<B, A> Iterator for Iterate<A> where A: FnMut() -> B {
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> { Some((self.func)()) }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { (usize::MAX, None) }
}

fn iterate<A, B>(f: A) -> Iterate<A> where A: FnMut() -> B {
    Iterate{func: f}
}

fn main() {
    let mut rng = thread_rng();
    let a = std::iter::repeat(1).into_iter().map(|_| rng.gen::< u32 >() % 100u32).take_while(|x| *x != 0u32).collect::<Vec<u32>>();

    let i = iterate(|| rng.gen::< u32 >() % 100u32).take_while(|x| x != &0u32).collect::<Vec<u32>>();
    println!("{:?}", a);
    println!("{:?}", i);
}
