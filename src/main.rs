trait Reduce {
    type Iterator;
    fn reduce(&self) -> Self::Iterator;
}

impl Reduce for u32 {
    type Iterator = U32ReduceIterator;
    
    fn reduce(&self) -> Self::Iterator {
        U32ReduceIterator::new(*self)
    }
}

struct U32ReduceIterator {
    current: u32,
    complete: bool
}

impl U32ReduceIterator {
    fn new(n: u32) -> U32ReduceIterator {
        U32ReduceIterator {
            current: n,
            complete: false
        }
    }
}

impl Iterator for U32ReduceIterator {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        if self.complete { return None; }
        
        let ret = self.current;
        match self.current {
            1 => self.complete = true,
            n => if n & 1 == 1 { self.current += 1; } else { self.current /= 2; }
        }
        Some(ret)
    }
}

pub fn main() {
    match std::env::args().nth(1).and_then(|n| n.parse::<u32>().ok()) {
        None => println!("invalid input"),
        Some(n) => println!("{}: {}", n, n.reduce().count())
    }
}

#[cfg(test)]
mod tests {
    use super::Reduce;

    #[test]
    fn reduce_five_in_six_steps() {
        assert!(&[5, 6, 3, 4, 2, 1] == &5u32.reduce().collect::<Vec<_>>()[..])
    }
}