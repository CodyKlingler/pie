fn main() {
    println!("{}", approx_pi());
}

// outputs 3.141592295640519
fn approx_pi() -> f64{
    let mut ac: u64 = 0;
    for x in 0..=u16::MAX as u64{
        ac += find_edge(x);
    }
    (ac*4) as f64 / u32::MAX as f64
}

fn find_edge(x: u64)-> u64 {
    let mut y = 0;
    // this is a binary search, but done bitwise and iteratively to avoid division
    for bit_n in (0..u16::BITS).rev() {
        y |= 1<<bit_n; 
        let dist = ((y*y + x*x) as f64).sqrt() as u64;
        if dist > u16::MAX as u64 {
            y ^= 1<<bit_n;          
        }
    }
    y
}