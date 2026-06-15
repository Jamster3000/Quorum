//! This includes tests to verify the compression and decompression works both ways as expected
//!
//! Run the tests using `cargo test -- --nocapture`
//!
//! If the output doesn't return `All OK: true` then something has failed

use denspack::{
    compress, decompress, probe_constants, train_dictionary, DecoderDictionary, EncoderDictionary,
};
use zstd::bulk::Compressor;

fn test_messages() -> Vec<String> {
    vec![
        "Hey, are you free tonight?".to_string(),
        "Did you see what happened in the last session?".to_string(),
        "I'll be online in like 10 minutes".to_string(),
        "the dragon rolled a nat 20 lmao we're all dead".to_string(),
        concat!(
            "Hey, are you free tonight? Did you see what happend in the last session? ",
            "I'll be online in like 10 minutes the dragon rolled a nat 20 lmao we're all dead"
        )
        .to_string(),
        "This is a long string that should compress quite well. ".repeat(5),
    ]
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn full_roundtrip_with_table() {
    let messages = test_messages();

    // Build training samples: each message repeated 20×
    let samples: Vec<String> = messages
        .iter()
        .cycle()
        .take(messages.len() * 20)
        .cloned()
        .collect();

    let dict_bytes = train_dictionary(&samples, 4096).expect("dictionary training failed");

    // Extract shared constants via a probe compression (level 19, same as Python)
    let constants = probe_constants(&dict_bytes, &messages[0]).expect("probe failed");

    println!("MAGIC:   {}", hex(&constants.magic));
    println!("DICT_ID: {}", hex(&constants.dict_id));

    let enc_dict = EncoderDictionary::copy(&dict_bytes, 1);
    let dec_dict = DecoderDictionary::copy(&dict_bytes);

    println!(
        "\n{:<52} {:>6} {:>8} {:>7} {:>7}",
        "Message", "Orig", "Before", "After", "Saved"
    );
    println!("{}", "-".repeat(86));

    let mut results: Vec<(String, Vec<u8>)> = Vec::new();

    for msg in &messages {
        let mut cmp = Compressor::with_prepared_dictionary(&enc_dict)
            .expect("compressor init failed");
        let before = cmp.compress(msg.as_bytes()).expect("before-compress failed");

        let after = compress(msg, &enc_dict).expect("compress failed");

        let orig = msg.len();
        let saved = before.len() as isize - after.len() as isize;

        let preview = if msg.len() > 50 {
            format!("{}..", &msg[..50])
        } else {
            msg.clone()
        };

        println!(
            "{:<52} {:>6} {:>8} {:>7} {:>7}",
            preview,
            orig,
            before.len(),
            after.len(),
            saved
        );

        //uncommen the line below if you want to see the specific bytes for each compressed string
        //println!("  bytes: {:02x?}", after);

        results.push((msg.clone(), after));
    }

    println!("\n--- Round trip verification ---");
    let mut all_ok = true;

    for (original, stored) in &results {
        let recovered = decompress(stored, &constants, &dec_dict).expect("decompress failed");
        let ok = recovered == *original;
        if !ok {
            all_ok = false;
        }
        let status = if ok { "OK" } else { "FAIL" };
        let preview = if original.len() > 60 {
            &original[..60]
        } else {
            original.as_str()
        };
        println!("{status}  |  {preview}");
    }

    println!("\nAll OK: {all_ok}");
    assert!(all_ok, "One or more messages failed round-trip verification");
}