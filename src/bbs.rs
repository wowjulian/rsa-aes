use std::str::FromStr;

use num::{BigUint, Integer, One, Zero};
use num_bigint::{RandBigInt, ToBigUint};

pub fn miller_rabin_test_k_and_q(n: BigUint) -> (u32, BigUint) {
    let mut k = 0;
    let mut q: BigUint = n - BigUint::one();
    while (&q % 2 as u8) == BigUint::zero() {
        k = k + 1;
        q = q / 2 as u8;
    }
    return (k, q);
}
pub fn miller_rabin_test(n: &BigUint, q: &BigUint, k: &u32) -> bool {
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

pub fn is_prime(number: &BigUint) -> bool {
    let (k, q) = miller_rabin_test_k_and_q(number.clone());
    for _i in 0..56 {
        let inconclusive = miller_rabin_test(&number, &q, &k);
        if !inconclusive {
            return false;
        }
    }
    return true;
}

pub fn generate_prime_number(bit_size: u64) -> BigUint {
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

pub fn find_prime_for_bbs() -> BigUint {
    let mut p: BigUint = generate_prime_number(1024);
    while p.clone() % (4 as u8) != 3.to_biguint().unwrap() {
        println!("% 4 = 3 not passed");
        p = generate_prime_number(1024);
    }
    return p;
}

pub fn find_seed_for_bbs(n: &BigUint) -> BigUint {
    let mut s_candidate = generate_prime_number(2048);
    let mut gcd = s_candidate.gcd(n);
    while gcd != BigUint::one() {
        s_candidate = generate_prime_number(2048);
        gcd = s_candidate.gcd(n);
    }
    println!("gcd: {}", gcd);
    return s_candidate;
}

pub fn blum_blum_shub(bit_count: u64) -> (BigUint, BigUint, BigUint) {
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
