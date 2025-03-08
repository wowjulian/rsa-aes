use std::str::FromStr;

use num::{BigInt, BigUint, FromPrimitive, One, ToPrimitive, Zero};
use num_bigint::{RandBigInt, ToBigInt, ToBigUint};
use rand::thread_rng;

// d = q

fn miller_rabin_test_q(n: BigUint) -> (u32, BigUint) {
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

    // println!("n: {}", n.clone());
    // println!("k: {}", *k);
    // println!("q: {}", q);
    // println!("a: {}", a);

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
    let (k, q) = miller_rabin_test_q(number.clone());
    for i in 0..56 {
        let inconclusive = miller_rabin_test(&number, &q, &k);
        if !inconclusive {
            return false;
        }
    }
    return true;
}

fn generate_prime_number() -> BigUint {
    let mut rng = rand::thread_rng();
    let mut odd_random_number = rng.gen_biguint(1024) | BigUint::one();
    let mut count = 0;
    while !is_prime(&odd_random_number) {
        count += 1;
        odd_random_number = rng.gen_biguint(1024) | BigUint::one();
    }
    println!("Count is: {}", count);
    println!("Prime is: {}", odd_random_number);
    return odd_random_number;
}

fn find_seed_prime_for_bbs() -> BigUint {
    let mut p: BigUint = generate_prime_number();
    while p.clone() % (4 as u8) != 3.to_biguint().unwrap() {
        println!("% 4 = 3 not passed");
        p = generate_prime_number();
    }
    return p;
}

fn main() {
    let p: BigUint = BigUint::from_str("91122045179318965173533839131368998662772456836316619574148988450969399638066015732396427566243748625301463193721989348160150289310601464760678023543905884939640329370981639669486054016790003739067183295427192269871515101958634419380284904391739809184729932234982543491394799238889453600867187568552286325947").unwrap();
    let q: BigUint = BigUint::from_str("22537017916243391647302294697783847565496398436656731187314486777281589031427811644162172298834653261847630683638084954095389795674883668957087575593936693030294860183039798443433626313894524118928724341320999385717299136781988248405248165926958106413623940227249989571125812352913420301394817441164820406503").unwrap();
    println!("passing: {}", p);
    println!("passing: {}", q);
}
