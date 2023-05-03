fn main() {
    let pi = std::f64::consts::PI;

    for i in 8..20 {
        let almost_pi = approx_pi_n(i);

        let accuracy = ((pi-almost_pi)/pi).log2();
        println!("r={i},\tacc= {}",accuracy);
    }
}

// outputs 3.141592295640519
fn approx_pi_n(n: u16) -> f64{

    let radius:u64 = (1<<n)-1;

    let mut ac: u64 = 0;
    for x in 0..=radius{
        ac += binary_search_circle_n(x, n);
    }

    let radius_squared = (1u64<<(n*2)) - (1u64<<(n+1)) - 1;

    (ac*4) as f64 / radius_squared as f64
}

fn binary_search_circle_n(x: u64, n: u16)-> u64 {
    let radius:u64 = (1<<n)-1;
    let mut y = 0;
    for bit_n in (0..n).rev() {
        y |= 1<<bit_n; 
        let dist = ((y*y + x*x) as f64).sqrt() as u64;
        if dist >= radius{
            y ^= 1<<bit_n;          
        }
    }
    y
}