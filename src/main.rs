use aes::{
    cipher::{
        consts::U16, generic_array::GenericArray, Block, BlockDecrypt, BlockEncrypt, KeyInit,
    },
    Aes128, Aes256,
};
use libm::erfc;
use num::{integer::Roots, BigUint, FromPrimitive, Integer, One, Zero};
use num_bigint::{RandBigInt, ToBigUint};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use std::str::FromStr;

fn miller_rabin_test_k_and_q(n: BigUint) -> (u32, BigUint) {
    let mut k = 0;
    let mut q: BigUint = n - BigUint::one();
    while (&q % 2 as u8) == BigUint::zero() {
        k = k + 1;
        q = q / 2 as u8;
    }
    return (k, q);
}
fn miller_rabin_test(n: &BigUint, q: &BigUint, k: &u32) -> bool {
    let mut rng = rand::thread_rng();
    let lbound = BigUint::one() + BigUint::one();
    let ubound = n - BigUint::one();
    let a: BigUint = rng.gen_biguint_range(&lbound, &ubound);
    let is_inconclusive: bool = (a.modpow(&q, n)) == BigUint::one();
    if is_inconclusive {
        return true;
    }

    let two: u32 = u32::one() + u32::one();
    if *k != 0 {
        for j in 0..*k {
            let exponent = two.pow(j) * q.clone();
            let a_pow_j_q_mod_n = a.modpow(&exponent, n);
            if a_pow_j_q_mod_n == (n - BigUint::one()) {
                return true;
            }
        }
    }
    // is composite
    return false;
}

fn is_prime(number: &BigUint) -> bool {
    let (k, q) = miller_rabin_test_k_and_q(number.clone());
    for _i in 0..56 {
        let inconclusive = miller_rabin_test(&number, &q, &k);
        if !inconclusive {
            return false;
        }
    }
    return true;
}

fn generate_prime_number(bit_size: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut odd_random_number = rng.gen_biguint(bit_size) | BigUint::one();
    let mut count = 0;
    while !is_prime(&odd_random_number) {
        count += 1;
        odd_random_number = rng.gen_biguint(bit_size) | BigUint::one();
    }
    println!("Count is: {}", count);
    println!("Prime is: {}", odd_random_number);
    return odd_random_number;
}

fn find_prime_for_bbs() -> BigUint {
    let mut p: BigUint = generate_prime_number(1024);
    while p.clone() % (4 as u8) != 3.to_biguint().unwrap() {
        println!("% 4 = 3 not passed");
        p = generate_prime_number(1024);
    }
    return p;
}

fn find_seed_for_bbs(n: &BigUint) -> BigUint {
    let mut s_candidate = generate_prime_number(2048);
    // while (s_candidate.g)
    let mut gcd = s_candidate.gcd(n);
    while gcd != BigUint::one() {
        s_candidate = generate_prime_number(2048);
        gcd = s_candidate.gcd(n);
    }
    println!("gcd: {}", gcd);
    return s_candidate;
}

fn blum_blum_shub(bit_count: u64) -> (BigUint, BigUint, BigUint) {
    // Calculated with find_prime_for_bbs(1024)
    let p: BigUint = BigUint::from_str("91122045179318965173533839131368998662772456836316619574148988450969399638066015732396427566243748625301463193721989348160150289310601464760678023543905884939640329370981639669486054016790003739067183295427192269871515101958634419380284904391739809184729932234982543491394799238889453600867187568552286325947").unwrap();
    let q: BigUint = BigUint::from_str("22537017916243391647302294697783847565496398436656731187314486777281589031427811644162172298834653261847630683638084954095389795674883668957087575593936693030294860183039798443433626313894524118928724341320999385717299136781988248405248165926958106413623940227249989571125812352913420301394817441164820406503").unwrap();
    let n = &q * &p;
    // Calculated with find_seed_for_bbs(&n)
    // let s: BigUint = find_seed_for_bbs(&n);
    let s = BigUint::from_str("16751150833723281346546415716076594334989226676891536485489274904983481515358742966589697654499417137468862387388402184459823842220567709616938731358137915608514479877281636657674934515491613725807050807519504523812185201499019018218289119132682700941058887266319869080558059248745161919485889078051880786249395645115940431604392311784385528345978282436907566189961509308578797652821187621757255880707855901282481165771099060247969949561518288779403680268076957377110069337764220111375311868780614276108799613224556262077784946151663844163100007591838503639610808192718051910877580017595649384146691473588376287568681").unwrap();

    // println!("p: {}", p);
    // println!("q: {}", q);
    // println!("n: {}", n);
    // println!("s: {}", s);

    let two = BigUint::one() + BigUint::one();
    let mut x = s.modpow(&two, &n);
    let mut result: BigUint = BigUint::zero();
    for i in 0..bit_count {
        x = x.modpow(&two, &n);
        let bit = x.modpow(&BigUint::one(), &two) & BigUint::one();
        result.set_bit(i, bit.bit(0));
    }

    return (q, p, result);
}

fn task_1() -> BigUint {
    // Task 1
    let (_q, _p, key) = blum_blum_shub(2048);
    println!("result: {:02048b}", key);
    println!("bits: {}", key.bits());

    // Task 1 - Test 1
    let zeros = key.bits() - key.count_ones();
    let ones = key.count_ones();
    let s_obs: f64 = (ones - zeros) as f64 / (key.bits() as f64).sqrt();
    let p_value = erfc(s_obs.abs() / (2 as f64).sqrt());

    println!("zeroes: {}", zeros);
    println!("ones: {}", ones);
    println!("p_value: {}", p_value);

    return key;
}

fn cbc_enc_with_key_and_iv(cipher: &Aes256, iv: u128, block: &mut Block<Aes256>) {
    for i in 0..block.len() {
        let iv_bit = ((iv >> (i * 8)) & 0xFF) as u8;
        block[i] = block[i] ^ iv_bit;
    }
    cipher.encrypt_block(block);
}

fn cbc_dec_with_key_and_iv(cipher: &Aes256, iv: u128, block: &mut Block<Aes256>) {
    cipher.decrypt_block(block);
    for i in 0..block.len() {
        let iv_bit = ((iv >> (i * 8)) & 0xFF) as u8;
        block[i] = block[i] ^ iv_bit;
    }
}

fn task_2() {
    let (_, _, aes_key) = blum_blum_shub(256);
    println!("aes_key: {:0256b}", aes_key);
    let key_bytes: [u8; 32] = aes_key.to_bytes_be().try_into().expect("failed to convert");
    let key = GenericArray::from(key_bytes);
    let cipher = Aes256::new(&key);

    let mut rng = ChaCha20Rng::from_seed(Default::default());
    let iv = (rng.next_u64() as u128) << 64 | rng.next_u64() as u128;

    let plaintext = b"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";
    let padded_plaintext = pad_pkcs7(plaintext, 16);

    let mut encrypted_blocks = Vec::new();

    for chunk in padded_plaintext.chunks_exact(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        // cipher.encrypt_block(&mut block);
        cbc_enc_with_key_and_iv(&cipher, iv, &mut block);
        encrypted_blocks.push(block);
    }
    println!("Encrypted Blocks:");
    for (i, block) in encrypted_blocks.iter().enumerate() {
        println!("Block {}: {:?}", i + 1, block);
    }

    let mut decrypted_blocks = Vec::new();
    for block in encrypted_blocks {
        let mut block_decrypted = block.clone();
        // cipher.decrypt_block(&mut block_decrypted);
        cbc_dec_with_key_and_iv(&cipher, iv, &mut block_decrypted);
        decrypted_blocks.push(block_decrypted);
    }
    let mut decrypted_bytes = Vec::new();
    for block in decrypted_blocks {
        decrypted_bytes.extend_from_slice(&block);
    }
    let decrypted_text = unpad_pkcs7(&decrypted_bytes);
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted_text));
}

// Got these two padding functions from GPT. I know how it works though, commenting on my own to demonstrate
// Basically it fills last block with number of u8s to pad for empty block elements. If the blocks are already full and divisble by block size, it creates a new block to extend.
fn pad_pkcs7(data: &[u8], block_size: usize) -> Vec<u8> {
    // Calculates how many u8s need to be padded
    let pad_len = block_size - (data.len() % block_size);
    // Creates a vector with given data
    let mut padded = data.to_vec();
    // Adds padding, last bytes will be number of padding length
    padded.extend(std::iter::repeat(pad_len as u8).take(pad_len));
    // Returns
    return padded;
}
fn unpad_pkcs7(data: &[u8]) -> Vec<u8> {
    // Grabs the padding length from the last u8
    let pad_len = *data.last().unwrap() as usize;
    // Return data without padding u8s
    return data[..data.len() - pad_len].to_vec();
}

fn task_3() {
    // Create a AES symm key
    let (_, _, aes_key) = blum_blum_shub(256);

    let (q, p, key) = blum_blum_shub(2048);
    let n = q.clone() * p.clone();
    let totient_n = (q - BigUint::one()) * (p - BigUint::one());
    let mut e = BigUint::one() + BigUint::one();
    loop {
        if e >= totient_n {
            panic!("could not e");
        }
        if e.gcd(&totient_n) == BigUint::one() {
            break;
        }

        e = e + BigUint::one();
    }
    let d = e.modinv(&totient_n).unwrap();
    println!("aes_key: {}", aes_key);
    // Encrpyt
    let encrypted_aes_key = aes_key.modpow(&e, &n);
    println!("encrypted_aes_key: {}", encrypted_aes_key);

    // Decrypt
    let decrypted_aes_key = encrypted_aes_key.modpow(&d, &n);
    println!("decrypted_aes_key: {}", decrypted_aes_key);
}

fn main() {
    // task_1();
    task_2();
    println!("======== [TASK 3] ========");
    task_3();
}
