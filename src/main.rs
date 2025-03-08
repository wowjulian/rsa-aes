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
    let lbound = &BigUint::from_u8(2).unwrap();
    let ubound = n - BigUint::one();
    println!("lbound: {} ubound: {}", lbound, ubound);
    let a: BigUint = rng.gen_biguint_range(&lbound, &ubound);
    let a_pow_q_mod_n: BigUint = a.modpow(&q, n);
    let is_inconclusive: bool = (a_pow_q_mod_n) == BigUint::one();
    if is_inconclusive {
        return true;
    }

    println!("n: {}", n.clone());
    println!("k: {}", *k);
    println!("q: {}", q);
    println!("a: {}", a);

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
    println!("composite");
    return false;
}

fn is_prime(number: &BigUint) -> bool {
    let (k, q) = miller_rabin_test_q(number.clone());
    for i in 0..128 {
        let inconclusive = miller_rabin_test(&number, &q, &k);
        if !inconclusive {
            return false;
        }
    }
    return true;
}

fn generate_prime_number() {
    let mut rng = rand::thread_rng();
    let odd_random_number: BigUint = BigUint::from_i32(5523).unwrap();
    // let odd_random_number = rng.gen_biguint(1024) | BigUint::one();
    println!("odd_random_number {}", odd_random_number);
    let confirmed_prime = is_prime(&odd_random_number);
    if confirmed_prime {
        println!("is prime");
    } else {
        println!("is not prime");
    }
}

fn main() {
    let test = BigInt::from_u64(100).unwrap();
    println!("test: {}", test + 1);
    generate_prime_number();
}
