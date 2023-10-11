use chrono::prelude::*;
use crate::sm3::hash::Sm3Hash;


pub mod sm3;
pub mod sm4;

extern crate byteorder;
extern crate rand;

extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate yasna;

#[macro_use]
extern crate lazy_static;

// sm4
use rand::RngCore;
use crate::sm4::{Cipher, Mode};

fn rand_block() -> [u8; 16] {
	let mut rng = rand::thread_rng();
	let mut block: [u8; 16] = [0; 16];
	rng.fill_bytes(&mut block[..]);
	block
}

fn main() {
	// sm3 签名后 https://github.com/citahub/libsm/tree/master
	let string = String::from("abc");
	let mut hash = Sm3Hash::new(string.as_bytes());
	let digest: [u8; 32] = hash.get_hash();

	// 66c7f0f4 62eeedd9 d1f2d46b dc10e4e2 4167c487 5cf2f7a2 297da02b 8f4ba8e0
	let content = digest.iter().map(|&b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
	println!("{:?}", digest);
	println!("{}", content);


	// SM4
	let key = rand_block();

	let cipher = Cipher::new(&key, Mode::Cfb).unwrap();

	let iv = rand_block();
	let plain_text = String::from("plain text");

	// Encryption
	let cipher_text: Vec<u8> = cipher.encrypt(&plain_text.into_bytes(), &iv).unwrap();
	let content = cipher_text.iter().map(|&b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
	println!("sm4==>{}", content);

	// Decryption
	let plain_text: Vec<u8> = cipher.decrypt(&cipher_text[..], &iv).unwrap();

	println!("{}", String::from_utf8(plain_text).unwrap());
}




