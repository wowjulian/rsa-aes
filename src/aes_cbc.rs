use aes::{
    cipher::{generic_array::GenericArray, Block, BlockDecrypt, BlockEncrypt},
    Aes256,
};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

// Got these two padding functions from GPT. I know how it works though, commenting on my own to demonstrate
// Basically it fills last block (crates a new block if all blocks are full) with number of u8s to pad for empty block elements.
fn pad_pkcs7(data: &[u8], block_size: usize) -> Vec<u8> {
    // Calculates how many u8s need to be padded
    let pad_len = block_size - (data.len() % block_size);
    // Creates a vector with data
    let mut padded: Vec<u8> = data.to_vec();
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

pub fn cbc_enc_with_key_and_iv(cipher: &Aes256, iv: u128, block: &mut Block<Aes256>) {
    for i in 0..block.len() {
        let iv_bit = ((iv >> (i * 8)) & 0xFF) as u8;
        block[i] = block[i] ^ iv_bit;
    }
    cipher.encrypt_block(block);
}

pub fn cbc_dec_with_key_and_iv(cipher: &Aes256, iv: u128, block: &mut Block<Aes256>) {
    cipher.decrypt_block(block);
    for i in 0..block.len() {
        let iv_bit = ((iv >> (i * 8)) & 0xFF) as u8;
        block[i] = block[i] ^ iv_bit;
    }
}

pub fn get_iv() -> u128 {
    let mut rng = ChaCha20Rng::from_seed(Default::default());
    let iv = (rng.next_u64() as u128) << 64 | rng.next_u64() as u128;
    return iv;
}

pub fn enc_cbc(cipher: &Aes256, iv: u128, data: &[u8]) -> Vec<Block<Aes256>> {
    let padded_data: Vec<u8> = pad_pkcs7(&data, 16);
    let mut encrypted_blocks: Vec<Block<Aes256>> = Vec::new();
    for chunk in padded_data.chunks_exact(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        // cipher.encrypt_block(&mut block);
        cbc_enc_with_key_and_iv(&cipher, iv, &mut block);
        encrypted_blocks.push(block);
    }
    return encrypted_blocks;
}

pub fn dec_cbc(cipher: &Aes256, iv: u128, encrypted_blocks: Vec<Block<Aes256>>) -> Vec<u8> {
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
    let decrypted_unpadded: Vec<u8> = unpad_pkcs7(&decrypted_bytes);
    return decrypted_unpadded;
}
