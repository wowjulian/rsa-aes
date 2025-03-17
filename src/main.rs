mod aes_cbc;
mod bbs;
mod crt;

use aes::{
    cipher::{Block, KeyInit},
    Aes256,
};
use aes_cbc::{dec_cbc, enc_cbc, get_aes_256_key, get_iv};
use bbs::{blum_blum_shub, blum_blum_shub_with_constants, frequency_test, runs_test};
use clap::{command, Parser};
use crt::dec_rsa_crt;
use num::{BigUint, Integer, One};
use std::{fs::File, io::Read};
use std::{io::Write, time::Instant};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Task number, choose from 1 to 5.
    #[arg(short, long)]
    task: u8,
}

fn task_1_slow() {
    let bit_count = 2048;
    let (_q, _p, key) = blum_blum_shub(bit_count);
    println!("result: {:02048b}", key);

    // Test 1
    frequency_test(&key, 2048);

    // Test 2
    runs_test(&key, 2048);
}

fn task_1() {
    let bit_count = 2048;
    let (_q, _p, key) = blum_blum_shub_with_constants(bit_count);
    println!("result: {:02048b}", key);

    // Test 1
    frequency_test(&key, 2048);

    // Test 2
    runs_test(&key, 2048);
}

fn task_2() {
    // Get IV
    let iv = get_iv();
    // Get key
    let key = get_aes_256_key();
    let cipher: Aes256 = Aes256::new(&key);

    let plaintext = b"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";

    // Encrypt
    let encrypted_blocks: Vec<Block<Aes256>> = enc_cbc(&cipher, iv, plaintext);
    println!("Encrypted Blocks:");
    for (i, block) in encrypted_blocks.iter().enumerate() {
        println!("Block {}: {:?}", i + 1, block);
    }

    // Decrypt
    let decrypted = dec_cbc(&cipher, iv, encrypted_blocks);
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
}

fn task_3() {
    // Generate a AES symm key
    let (_, _, aes_key) = blum_blum_shub_with_constants(256);

    // Generate q and p
    let (q, p, _) = blum_blum_shub_with_constants(2048);

    // Calculate n and totient n
    let n = q.clone() * p.clone();
    let totient_n = (q - BigUint::one()) * (p - BigUint::one());

    // Cacluate e
    let mut e = BigUint::one() + BigUint::one();
    loop {
        if e >= totient_n {
            panic!("could not calculate e");
        }
        if e.gcd(&totient_n) == BigUint::one() {
            break;
        }
        e = e + BigUint::one();
    }
    // Calculate d
    let d = e.modinv(&totient_n).unwrap();
    // Encrpyt
    let encrypted_aes_key = aes_key.modpow(&e, &n);

    // Decrypt
    let decrypted_aes_key = encrypted_aes_key.modpow(&d, &n);

    println!("aes_key: {}", aes_key);
    println!("encrypted_aes_key: {}", encrypted_aes_key);
    println!("decrypted_aes_key: {}", decrypted_aes_key);
}

fn task_4() {
    // Get IV
    let iv = get_iv();
    // Get key
    let key = get_aes_256_key();
    let cipher: Aes256 = Aes256::new(&key);

    // Read image
    let mut img: File = File::open("./images/dune.webp").expect("file failed to open");
    let mut bytes = Vec::new();
    img.read_to_end(&mut bytes).expect("Failed to read file");

    // Encrypt
    let encrypted_blocks: Vec<Block<Aes256>> = enc_cbc(&cipher, iv, &bytes);
    // Decrypt
    let decrypted: Vec<u8> = dec_cbc(&cipher, iv, encrypted_blocks);

    // Write new image
    let mut out_file = File::create("./images/output.webp").unwrap();
    out_file.write_all(&decrypted).unwrap();
    println!("File written at ./images/output.webp");
}

fn task_5() {
    // Generate a AES symm key
    let (_, _, aes_key) = blum_blum_shub_with_constants(256);
    println!("key: {}", aes_key);

    // Generate q and p
    let (q, p, _) = blum_blum_shub_with_constants(2048);

    // Calculate n and totient n
    let n = q.clone() * p.clone();
    let totient_n = (&q - BigUint::one()) * (&p - BigUint::one());

    // Cacluate e
    let mut e = BigUint::one() + BigUint::one();
    loop {
        if e >= totient_n {
            panic!("could not calculate e");
        }
        if e.gcd(&totient_n) == BigUint::one() {
            break;
        }
        e = e + BigUint::one();
    }
    // Calculate d
    let d = e.modinv(&totient_n).unwrap();
    // Encrpyt
    let encrypted_aes_key = aes_key.modpow(&e, &n);

    // Decrypt using modpow
    let start = Instant::now();
    let decrypted_aes_key = encrypted_aes_key.modpow(&d, &n);
    let duration = start.elapsed();
    println!("REG dec [{:?}]: {}", duration, decrypted_aes_key);
    // Decrypt using CRT
    let start = Instant::now();
    let decrypted_with_crt = dec_rsa_crt(&d, &p, &q, &encrypted_aes_key);
    let duration = start.elapsed();
    println!("CRT dec [{:?}]: {}", duration, decrypted_with_crt);
}

fn main() {
    let args = Args::parse();
    let task = args.task;
    match task {
        1 => {
            task_1();
        }
        2 => {
            task_2();
        }
        3 => {
            task_3();
        }
        4 => {
            task_4();
        }
        5 => {
            task_5();
        }
        11 => {
            task_1_slow();
        }
        _ => {
            panic!("[INVALID TASK] Choose between 1 to 5 for task. Use Task 11 for task 1 but slow")
        }
    }
}
