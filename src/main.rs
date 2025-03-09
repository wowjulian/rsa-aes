use std::str::FromStr;

use num::{BigUint, FromPrimitive, Integer, One, Zero};
use num_bigint::{RandBigInt, ToBigUint};

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

fn blum_blum_shub(bit_count: u64) {
    // Calculated with find_prime_for_bbs(1024)
    let p: BigUint = BigUint::from_str("91122045179318965173533839131368998662772456836316619574148988450969399638066015732396427566243748625301463193721989348160150289310601464760678023543905884939640329370981639669486054016790003739067183295427192269871515101958634419380284904391739809184729932234982543491394799238889453600867187568552286325947").unwrap();
    let q: BigUint = BigUint::from_str("22537017916243391647302294697783847565496398436656731187314486777281589031427811644162172298834653261847630683638084954095389795674883668957087575593936693030294860183039798443433626313894524118928724341320999385717299136781988248405248165926958106413623940227249989571125812352913420301394817441164820406503").unwrap();
    let n = &q * &p;
    // Calculated with find_seed_for_bbs(&n)
    let s: BigUint = BigUint::from_str("31014859539989961550294918383817356413418493879093771658452375922104359120667102569009366323919164522605099377825279663351929807917505294701935218680857311298262390049081347900029182479401979716495698170100133522567463748897727626684346537089426314678803347887983463451138683913923091077847002097699134060096550023070041391353562534004921995531378913343828371740290642867352370494852139557774153067247773742850870953756160046998767293918999243554699616857427279841132797123546581787695855219731780677670077017829025666439591748711845589831797051878552680581366882638246772821254901472943218027432998696691499093515377").unwrap();

    println!("p: {}", p);
    println!("q: {}", q);
    println!("n: {}", n);
    println!("s: {}", s);

    let two = BigUint::one() + BigUint::one();
    let mut x = s.modpow(&two, &n);
    for i in 0..bit_count {
        x = x.modpow(&two, &n);
        let bit = x.modpow(&BigUint::one(), &two);
        print!("{}", bit);
    }
}

fn main() {
    blum_blum_shub(1024);
}
