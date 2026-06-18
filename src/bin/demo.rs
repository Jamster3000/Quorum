use fhe::context::Context;
use fhe::bfv::{BfvPublicKey, BfvSecretKey};
use std::io;
use std::time::{Instant, Duration};

const BENCHMARK_ITERATIONS: u32 = 5000;

fn main() {
    //FHE (Fully Homomorphic Encryption)
    let ctx = Context::new(); //Initiate the FHE context one time

    println!("------ RUNNING STANDARD DEMOS ------");
    run_integer_demo(&ctx);

    let (bfv_sk, bfv_pk, patterns) = run_bfv_moderation_demo(&ctx);

    run_noise_growth_test(&ctx);

    println!("\n\n\n\n------ RUNNING PERFORMANCE BENCHMARKS ------");
    println!("Running {BENCHMARK_ITERATIONS} iterations per test for stable averages...\n");

    benchmark_core_ops(&ctx);
    benchmark_string_moderation(&ctx, &bfv_pk, &bfv_sk, &patterns);

    run_interactive_loop(ctx, bfv_pk, bfv_sk, patterns);
}

fn run_integer_demo(ctx: &Context) {
    // ------Integer scheme------
    // This is a more basic example of FHE operating on single integers.
    // Encrypt a number, decrypt a number, add two encrypted numbers together.

    let t = Instant::now();
    let (sk, pk) = ctx.generate_keys();

    println!("------INTEGER SCHEME------");
    println!("Key generation:     {:>8.3?}", t.elapsed());


    // Basic a+b operation on encryption.
    let (a, b) = (5u64, 7u64); // u64 integer bytes for 5 and 7
    let t = Instant::now();

    let a_encrypted = ctx.encrypt(a, &pk);
    let b_encrypted = ctx.encrypt(b, &pk);
    let a_plus_b_encrypted = ctx.add(&a_encrypted, &b_encrypted);
    let result = ctx.decrypt(&a_plus_b_encrypted, &sk);

    println!("encrypt({a}) + encrypt({b}) -> {result}  (expected {})  {}  {:>8.3?}", a + b, check(result == a + b), t.elapsed());


    // Sum a list of numbers
    let vals: Vec<u64> = vec![3, 7, 2, 8, 5];
    let t = Instant::now();

    let cts: Vec<_> = vals
        .iter()
        .map(|&v| ctx.encrypt(v, &pk))
        .collect();

    let total = cts[1..]
        .iter()
        .fold(ctx.encrypt(vals[0], &pk), |acc, ct| ctx.add(&acc, ct));

    let result = ctx.decrypt(&total, &sk);
    let expected = vals.iter().sum::<u64>();

    println!("sum({vals:?}) -> {result}  (expected {expected})  {}  {:>8.3?}", check(result == expected), t.elapsed());


    // Raw bytes
    let raw: Vec<u8> = vec![0x2f, 0x1a, 0x3d, 0x00, 0x01, 0x00, 0x71, 0xf6, 0x02, 0x02];
    let t = Instant::now();

    let encrypted = ctx.encrypt_bytes(&raw, &pk);
    let decrypted = ctx.decrypt_bytes(&encrypted, &sk);

    println!("encrypt_bytes({raw:02x?})  {}  {:>8.3?}", check(raw == decrypted), t.elapsed());


    //Encrypting string
    let msg = "Hey, are you free tonight?";
    let t = Instant::now();

    let encrypt_msg = ctx.encrypt_str(msg, &pk);
    let decrypted = ctx.decrypt_str(&encrypt_msg, &sk).unwrap();

    println!("encrypt_str(\"{msg}\")  {}  {:>8.3?}", check(msg == decrypted), t.elapsed());
}

fn run_bfv_moderation_demo(ctx: &Context) -> (BfvSecretKey, BfvPublicKey, Vec<Vec<u8>>) {
    // ------ BFV (Brakerski/Fan-Vercauteren) scheme + moderation using banned words.
    println!("\n------BFV SCHEME + MODERATION------");

    let t = Instant::now();
    let (bfv_sk, bfv_pk) = ctx.generate_bfv_keys();
    println!("bfv keygen:      {:>8.3?}", t.elapsed());

    //Example list of banned words. Each string has to be converted
    //to bytes before encryption.
    let banned: Vec<&[u8]> = vec![b"spam", b"banned", b"http://evil.com"];

    let t = Instant::now();

    let patterns: Vec<Vec<u8>> = banned
        .iter()
        .map(|p| ctx.encrypt_pattern(p, &bfv_pk))
        .collect();

    let total_pattern_bytes: usize = banned
        .iter()
        .map(|p| p.len())
        .sum();

    println!("prepare patterns ({total_pattern_bytes} bytes total):  {:>8.3?}", t.elapsed());

    let messages: Vec<(&str, bool)> = vec![
        ("Hey, are you free tonight?",        false),
        ("Click here: http://evil.com/thing", true),
        ("This is spam and you know it",      true),
        ("I'll be online in 10 minutes",      false),
        ("You are banned from this server",   true),
    ];

    println!();
    for (msg, expect_flagged) in &messages {
        let t = Instant::now();
        let encrypted_msg = ctx.encrypt_message(msg, &bfv_pk);
        let encrypt_time = t.elapsed();

        let t = Instant::now();
        let flagged = ctx.scan(&encrypted_msg, &patterns, &bfv_sk, msg.len());
        let scan_time = t.elapsed();

        let status = if flagged { "FLAGGED" } else { "clean  " };
        println!("  [{status}]  {}  encrypt {:>8.3?}  scan {:>8.3?}", check(flagged == *expect_flagged), encrypt_time, scan_time);
        println!("           \"{}\"", msg);
    }

    (bfv_sk, bfv_pk, patterns)
}

fn run_noise_growth_test(ctx: &Context) {
    let (sk, pk) = ctx.generate_keys();

    // Noise growth
    // This showed how many chain add's can be used before it breaks.
    // Most realistic use cases would never be doing 4096 adds in a row,
    // but this is a good stress test to see how many adds can be done
    // before the noise grows too large and the decryption fails.
    println!("\nNoise growth (integer scheme):");
    let base = ctx.encrypt(1, &pk);
    let mut cur = ctx.encrypt(1, &pk);
    let t = Instant::now();
    for i in 2..=4096u64 {
        cur = ctx.add(&cur, &base);
        let result = ctx.decrypt(&cur, &sk);
        let expected = i % 256;
        if result != expected {
            println!("  Broke at step {i}: got {result}, expected {expected}");
            return;
        }
    }

    println!("  Noise growth test completed in {:>8.3?}", t.elapsed());
    println!("  Held for all 4096 steps.");
}

// ==========================================
//          BENCHMARK IMPLEMENTATIONS
// ==========================================

fn benchmark_core_ops(ctx: &Context) {
    let (sk, pk) = ctx.generate_keys();

    let mut total_add_time = Duration::ZERO;
    for _ in 0..BENCHMARK_ITERATIONS {
        let a_enc = ctx.encrypt(5u64, &pk);
        let b_enc = ctx.encrypt(7u64, &pk);

        let start = Instant::now();
        let _res_enc = ctx.add(&a_enc, &b_enc);
        total_add_time += start.elapsed();
    }
    println!("Average encrypted add (a + b):        {:?}", total_add_time / BENCHMARK_ITERATIONS);

    // 8 Bytes Benchmark
    let mut total_enc_bytes_time = Duration::ZERO;
    let mut total_dec_bytes_time = Duration::ZERO;
    let benchmark_bytes = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];

    for _ in 0..BENCHMARK_ITERATIONS {
        let start_enc = Instant::now();
        let enc = ctx.encrypt_bytes(&benchmark_bytes, &pk);
        total_enc_bytes_time += start_enc.elapsed();

        let start_dec = Instant::now();
        let _dec = ctx.decrypt_bytes(&enc, &sk);
        total_dec_bytes_time += start_dec.elapsed();
    }
    println!("Average encrypt_bytes (8 bytes):      {:?}", total_enc_bytes_time / BENCHMARK_ITERATIONS);
    println!("Average decrypt_bytes (8 bytes):      {:?}", total_dec_bytes_time / BENCHMARK_ITERATIONS);

    // 256 Bytes Benchmark
    let mut total_enc_bytes_256_time = Duration::ZERO;
    let mut total_dec_bytes_256_time = Duration::ZERO;
    let benchmark_bytes_256 = vec![0xAA; 256];

    for _ in 0..BENCHMARK_ITERATIONS {
        let start_enc = Instant::now();
        let enc = ctx.encrypt_bytes(&benchmark_bytes_256, &pk);
        total_enc_bytes_256_time += start_enc.elapsed();

        let start_dec = Instant::now();
        let _dec = ctx.decrypt_bytes(&enc, &sk);
        total_dec_bytes_256_time += start_dec.elapsed();
    }
    println!("Average encrypt_bytes (256 bytes):    {:?}", total_enc_bytes_256_time / BENCHMARK_ITERATIONS);
    println!("Average decrypt_bytes (256 bytes):    {:?}", total_dec_bytes_256_time / BENCHMARK_ITERATIONS);
}

fn benchmark_string_moderation(ctx: &Context, bfv_pk: &BfvPublicKey, bfv_sk: &BfvSecretKey, patterns: &[Vec<u8>]) {
    let benchmark_str = "Warning: Click here http://evil.com/now";

    let mut total_msg_encrypt = Duration::ZERO;
    let mut total_scan_time = Duration::ZERO;

    for _ in 0..BENCHMARK_ITERATIONS {
        let start_enc = Instant::now();
        let encrypted_msg = ctx.encrypt_message(benchmark_str, bfv_pk);
        total_msg_encrypt += start_enc.elapsed();

        let start_scan = Instant::now();
        let _flagged = ctx.scan(&encrypted_msg, patterns, bfv_sk, benchmark_str.len());
        total_scan_time += start_scan.elapsed();
    }

    println!("Average encrypt_message (String):     {:?}", total_msg_encrypt / BENCHMARK_ITERATIONS);
    println!("Average homomorphic scan (String):    {:?}", total_scan_time / BENCHMARK_ITERATIONS);
}

fn run_interactive_loop(ctx: Context, bfv_pk: BfvPublicKey, bfv_sk: BfvSecretKey, patterns: Vec<Vec<u8>>) {
    // Interactive moderation test
    println!("\n------ INTERACTIVE MODERATION ------");
    println!("Enter a message to test the moderation");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();

        if input.len() > ctx.bfv.n {
            println!("Input too long (max {} characters).", ctx.bfv.n);
            continue;
        }

        let t = Instant::now();

        let encrypted_msg = ctx.encrypt_message(input, &bfv_pk);
        let encrypted_time = t.elapsed();

        let t = Instant::now();
        let flagged = ctx.scan(&encrypted_msg, &patterns, &bfv_sk, input.len());
        let scan_time = t.elapsed();

        let status = if flagged { "FLAGGED" } else { "clean  " };
        println!("  [{status}]  \"{input}\"");
        println!("  encrypt {:>8.3?}  scan {:>8.3?}  total {:>8.3?}", encrypted_time, scan_time, encrypted_time + scan_time);
    }
}

fn check(ok: bool) -> &'static str {
    if ok { "CORRECT" } else { "INCORRECT" }
}