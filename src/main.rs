mod aes_cbc;
mod bbs;
mod crt;

use aes::{
    cipher::{generic_array::GenericArray, Block, Key, KeyInit},
    Aes256,
};
use aes_cbc::{dec_cbc, enc_cbc, get_iv};
use bbs::blum_blum_shub;
use clap::{command, Parser};
use crt::dec_rsa_crt;
use libm::erfc;
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

fn task_1() -> BigUint {
    // Task 1
    let (_q, _p, key) = blum_blum_shub(2048);
    println!("result: {:02048b}", key);
    println!("bits: {}", key.bits());

    // Test 1
    let zeros = key.bits() - key.count_ones();
    let ones = key.count_ones();
    let s_obs: f64 = (ones - zeros) as f64 / (key.bits() as f64).sqrt();
    let p_value = erfc(s_obs.abs() / (2 as f64).sqrt());

    println!("zeroes: {}", zeros);
    println!("ones: {}", ones);
    println!("p_value: {}", p_value);

    // Test 2
    let mut runs = 0;
    let mut last_bit = !key.bit(0);
    for i in 0..2048 {
        let bit = key.bit(i);
        if bit == last_bit {
            continue;
        };
        runs += 1;
        last_bit = bit;
    }

    return key;
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
    let (_, _, aes_key) = blum_blum_shub(256);

    // Generate q and p
    let (q, p, _) = blum_blum_shub(2048);

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

fn get_aes_256_key() -> Key<Aes256> {
    let (_, _, key_bits) = blum_blum_shub(256);
    let key_bytes: [u8; 32] = key_bits
        .to_bytes_be()
        .try_into()
        .expect("failed to convert");
    let key = GenericArray::from(key_bytes);
    return key;
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
    let (_, _, aes_key) = blum_blum_shub(256);
    println!("key: {}", aes_key);

    // Generate q and p
    let (q, p, _) = blum_blum_shub(2048);

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
        _ => {
            panic!("Choose between 1 to 5 for task")
        }
    }
}
