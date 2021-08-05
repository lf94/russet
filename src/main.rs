extern crate base64;
extern crate openssl;
extern crate rayon;

use openssl::symm::{decrypt_aead, Cipher};
use rayon::prelude::*;

fn main() {
  let ciphertext_and_tag_base64 = "";
  let salt_base64 = "";
  let iv_base64 = "";

  let salt = &base64::decode(salt_base64).unwrap();
  let ciphertext_and_tag = &base64::decode(ciphertext_and_tag_base64).unwrap();
  let iv_nonce = base64::decode(iv_base64).unwrap();
  let iv_nonce_some = Some(iv_nonce.as_slice());
  let aes_gcm_split_point = ciphertext_and_tag.len() - 16;
  let (ciphertext, tag) = ciphertext_and_tag.split_at(aes_gcm_split_point);

  let aad_empty = &[0; 0];
  let aes_gcm = Cipher::aes_256_gcm();
  let sha256 = openssl::hash::MessageDigest::sha256();

  let passwords = std::fs::read_to_string("wordlist.lst").unwrap();
  passwords.par_lines().for_each(|password| {
    let mut derived_key = [0u8; 32];
    openssl::pkcs5::pbkdf2_hmac(
      password.as_bytes(),
      salt,
      10000,
      sha256,
      &mut derived_key
    );

    if derived_key[0] & 0b11111111 == 0b11111111 {
      println!("progress\t{}", password);
    }

    if let Ok(result) = decrypt_aead(
      aes_gcm,
      &derived_key,
      iv_nonce_some,
      aad_empty,
      ciphertext,
      tag
    ) {
      println!("{}", password);
      println!("{}", std::str::from_utf8(&result).unwrap());
    }
  });
}
