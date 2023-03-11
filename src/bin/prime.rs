use procon_library_rs::prime::*;
use std::time::Instant;

fn main() {
    // 素因数分解の速度比較

    let n = 999978;
    let start = Instant::now();
    for _ in 0..1_000_000 {
        prime_factorize(n);
    }
    let end = start.elapsed();
    println!("time of prime_factorize: {} milli sec", end.as_millis());

    let start = Instant::now();
    let min_factor = pre_osa_k(1_000_000);
    let end = start.elapsed();
    println!("time of pre_osa_k: {} milli sec", end.as_millis());

    let start = Instant::now();
    for _ in 0..1_000_000 {
        osa_k(n, &min_factor);
    }
    let end = start.elapsed();
    println!("time of osa_k: {} milli sec", end.as_millis());
}
