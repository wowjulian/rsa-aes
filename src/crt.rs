use num::{BigUint, Integer, One};

pub fn dec_rsa_crt(d: &BigUint, p: &BigUint, q: &BigUint, encrypted_aes_key: &BigUint) -> BigUint {
    let p_minus_one = p - BigUint::one();
    let q_minus_one = q - BigUint::one();
    let dp = d % &p_minus_one;
    let dq = d % &q_minus_one;
    let q_inv = q.modinv(p).unwrap();
    let m1 = (encrypted_aes_key.clone() % p).modpow(&dp, &p);
    let m2 = (encrypted_aes_key.clone() % q).modpow(&dq, &q);
    let h = (q_inv * (m1 - &m2)) % p;
    let m = m2 + h * q;
    return m;
}
