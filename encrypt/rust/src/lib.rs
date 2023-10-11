#[macro_use]
extern crate log;
extern crate android_logger;

extern crate jni;

use jni::objects::*;
use jni::sys::{jchar, jint, jlong, jobject, jstring, jdouble, jfloat, jbyte, jboolean, jbyteArray, jsize};
use jni::JNIEnv;
use chrono::prelude::*;

use log::LevelFilter;
use android_logger::{Config, FilterBuilder};


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


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_initLogger(
	env: JNIEnv,
	_class: JClass,
) {
	android_logger::init_once(
		Config::default()
			.with_max_level(LevelFilter::Trace) // limit log level
			.with_tag("chenxh_rust") // logs will show under mytag tag
			.with_filter( // configure messages for specific crate
			              FilterBuilder::new()
				              .parse("debug,hello::crate=error")
				              .build())
	);
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_changeJNIString(
	mut env: JNIEnv,
	_class: JClass,
	data: JString,
) -> jstring {
	let mut input: String = env.get_string(&data).expect("input msg error").into();
	input.push_str(" OVER");
	info!("changeJNIString  = {}", input);

	let name = env
		.new_string("I am Rust")
		.expect("Couldn't create java string!");
	**name
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_changeJNIChar(
	mut env: JNIEnv,
	_class: JClass,
	data: jchar,
) -> jchar {
	info!("this is a changeJNIChar {}", data.to_string());

	let reChar: u16 = '中' as u16;
	reChar
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_changeJNIByte(
	mut env: JNIEnv,
	_class: JClass,
	data: jbyte,
) -> jbyte {
	info!("this is a changeJNIByte {}", data);

	let i8_value: i8 = 8;
	i8_value
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_changeJNIInt(
	mut env: JNIEnv,
	_class: JClass,
	data: jint,
) -> jint {
	info!("this is a changeJNIInt {}", data);

	let i8_value: i32 = 65534;
	i8_value
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_changeJNIBoolean(
	mut env: JNIEnv,
	_class: JClass,
	data: jboolean,
) -> jboolean {
	info!("this is a changeJNIBoolean {}", data);

	let i8_value: u8 = true as u8;
	i8_value
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_changeJNILong(
	env: JNIEnv,
	_class: JClass,
	time: jlong,
) -> jlong {
	info!("this is a changeJNILong {}", time);

	let currTime = time;
	println!("time = {}", currTime);
	let current_datetime = Local::now().timestamp_micros();
	info!("Current datetime: {}", current_datetime);
	current_datetime
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_changeJNIFloat(
	env: JNIEnv,
	_class: JClass,
	time: jfloat,
) -> jfloat {
	info!("this is a changeJNIFloat {}", time);
	let current_datetime: f32 = 33.322;
	current_datetime
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_changeJNIDouble(
	env: JNIEnv,
	_class: JClass,
	time: jdouble,
) -> jdouble {
	info!("this is a changeJNIDouble {}", time);

	let current_datetime: f64 = 11.11;
	current_datetime
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_changeJNIByteArray<'local>(
	env: JNIEnv<'local>,
	_class: JClass,
	input: JByteArray<'local>,
) -> JByteArray<'local>
{

	// 处理接收 字节数组
	let _input = env.convert_byte_array(&input).unwrap();
	let inputContent = std::str::from_utf8(&_input).unwrap();
	info!("this is a changeJNIByteArray inputContent {}", inputContent);

	let sbytes = b"Hello, Kotlin!";
	let mut bytes = Vec::new();
	for b in sbytes {
		bytes.push(b);
	}


	let output = env.byte_array_from_slice(sbytes).unwrap();

	output
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_sm3(
	mut env: JNIEnv,
	_class: JClass,
	data: JString,
) -> jstring {
	let input: String = env.get_string(&data).expect("input msg error").into();

	// info!("待签名数据=[{}]", input);

	let mut hash = Sm3Hash::new(input.as_bytes());
	let digest: [u8; 32] = hash.get_hash();

	// 66c7f0f4 62eeedd9 d1f2d46b dc10e4e2 4167c487 5cf2f7a2 297da02b 8f4ba8e0

	let name = env
		.new_string(digest.iter().map(|&b| format!("{:02x}", b)).collect::<Vec<_>>().join(""))
		.expect("br sm3 expect");

	**name
}


use std::sync::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    static ref SM4KEY: Mutex< [u8; 16] > = Mutex::new([0; 16]);
	// pub static ref SM4KEY: [u8; 16]  = [0; 16];
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_setSm4Key<'local>(
	env: JNIEnv<'local>,
	_class: JClass,
	input: JByteArray<'local>,
) -> jboolean
{
	let _input = env.convert_byte_array(&input).unwrap();

	if _input.len() != 16 {
		return false as u8;
	}

	let mut mutexKey = SM4KEY.lock().unwrap();

	// let mut bytes = Vec::new();
	// for b in _input {
	// 	bytes.push(b);
	// }
	&mutexKey.clone_from_slice(&_input);

	let i8_value: u8 = true as u8;
	i8_value
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_sm4Encrypt<'local>(
	env: JNIEnv<'local>,
	_class: JClass,
	input: JByteArray<'local>,
) -> JByteArray<'local>
{
	let key = (SM4KEY.lock().unwrap());
	let cipher = Cipher::new(&key.to_vec(), Mode::Cfb).unwrap();

	let _input = env.convert_byte_array(&input).unwrap();
// Encryption
	let cipher_text: Vec<u8> = cipher.encrypt(&_input, &key.to_vec()).unwrap();
	let content = cipher_text.iter().map(|&b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
	info!("sm4==>{}", content);

	env.byte_array_from_slice(&cipher_text).unwrap()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_android_encrypt_ProviderJNI_sm4Decrypt<'local>(
	env: JNIEnv<'local>,
	_class: JClass,
	input: JByteArray<'local>,
) -> JByteArray<'local>
{
	let key = (SM4KEY.lock().unwrap());
	let _input = env.convert_byte_array(&input).unwrap();

	let cipher = Cipher::new(&key.to_vec(), Mode::Cfb).unwrap();

	let plain_text: Vec<u8> = cipher.decrypt(&_input[..], &key.to_vec()).unwrap();

	env.byte_array_from_slice(&plain_text).unwrap()
}

