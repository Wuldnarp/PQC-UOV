use uov::keys::keygen;
use uov::signature::{sign, verify};

#[cfg(target_arch = "x86_64")]
fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

fn measure(runs: usize, mut f: impl FnMut()) -> (f64, u64, u64, u64) {
    // Warmup
    f();

    let mut samples: Vec<u64> = (0..runs)
        .map(|_| {
            let start = rdtsc();
            f();
            rdtsc() - start
        })
        .collect();

    samples.sort_unstable();

    let avg = samples.iter().sum::<u64>() as f64 / runs as f64;
    let q1  = samples[runs / 4];
    let med = samples[runs / 2];
    let q3  = samples[3 * runs / 4];

    (avg, q1, med, q3)
}

fn print_result(label: &str, runs: usize, avg: f64, q1: u64, med: u64, q3: u64) {
    println!("  {label}");
    println!("    avg:    {:>12.0} cycles  (over {runs} runs)", avg);
    println!("    median: {:>12} cycles", med);
    println!("    Q1/Q3:  {:>12} / {} cycles", q1, q3);
}

#[test]
fn bench_cycles() {
    println!("\n=== OV(16,160,64) Rust Implementation ===\n");

    let (avg, q1, med, q3) = measure(100, || { keygen(); });
    print_result("Keygen", 100, avg, q1, med, q3);

    println!();

    let (_pk, sk) = keygen();
    let message = b"post-quantum salatmaxxing";
    let (avg, q1, med, q3) = measure(1000, || { sign(&sk, message).unwrap(); });
    print_result("Sign", 1000, avg, q1, med, q3);

    println!();

    let (pk, sk) = keygen();
    let sig = sign(&sk, message).unwrap();
    let (avg, q1, med, q3) = measure(1000, || { verify(&pk, message, &sig); });
    print_result("Verify", 1000, avg, q1, med, q3);
}