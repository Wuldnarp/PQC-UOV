use uov::keys::keygen;
use uov::signature::{sign, verify};
use std::time::Instant;

const RUNS: u32 = 10;

#[test]
fn bench_keygen() {
    let t = Instant::now();
    for _ in 0..RUNS {
        keygen();
    }
    println!("Keygen avg over {RUNS} runs: {:?}", t.elapsed() / RUNS);
}

#[test]
fn bench_sign() {
    let (_pk, sk) = keygen();
    let message = b"post-quantum salatmaxxing";

    let t = Instant::now();
    for _ in 0..RUNS {
        sign(&sk, message).expect("signing failed");
    }
    println!("Sign avg over {RUNS} runs: {:?}", t.elapsed() / RUNS);
}

#[test]
fn bench_verify() {
    let (pk, sk) = keygen();
    let message = b"post-quantum salatmaxxing";
    let sig = sign(&sk, message).expect("signing failed");

    let t = Instant::now();
    for _ in 0..RUNS {
        verify(&pk, message, &sig);
    }
    println!("Verify avg over {RUNS} runs: {:?}", t.elapsed() / RUNS);
}