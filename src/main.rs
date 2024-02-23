#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;
use std::io::{Bytes, Write};
use std::ops::Div;
use aes::{Aes128, Aes128Enc, Aes256};
use aes::cipher::generic_array::{GenericArray};
use aes::cipher::{BlockDecrypt, BlockEncrypt, Key, KeyInit};
use aes::cipher::consts::U16;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use hex::FromHex;
use libaes::Cipher;

/// auth flow:
/// auth
/// profiles
/// getSecureLevelInfo


// #[cfg(not(target_arch = "wasm32"))]
// fn main() -> eframe::Result<()> {
//     env_logger::init();
//
//     let native_options = eframe::NativeOptions {
//         initial_window_size: Some(egui::vec2(640.0, 480.0)),
//         decorated: false,
//         ..Default::default()
//     };
//     eframe::run_native(
//         "Dreamfinity Launcher",
//         native_options,
//         Box::new(|cc| Box::new(launcher::LauncherApp::new(cc))),
//     )
// }

fn str_to_bytes(string: &str) -> Vec<u8> {
    let length = string.len() / 2;
    let mut result = vec![];
    for i in 0..length {
        let c = hex::decode(string.chars().skip(i * 2).take(2).collect::<String>()).unwrap()[0];
        result.push(c);
    }
    return result;
}

fn main() {
    const IV_VECTOR: &[u8] = b"ivvector";
    const IV_VECTOR_STR: &str = "ivvectorstr";
    const KEY: &str = "secretkey";
    let mut ivvec = vec![];
    for i in 0..IV_VECTOR_STR.len() {
        ivvec.push(IV_VECTOR[i]);
    }
    let kp128 = str_to_bytes(KEY);
    let iv_bytes: &[u8] = ivvec.as_slice();
    let key_bytes: &[u8] = kp128.as_slice();
    println!("{:?}", iv_bytes);

    let password = "mysecretpassword";
    let size = (1 + password.len() / 16) * 16;
    let fill = std::char::from_u32((size - password.len()) as u32).unwrap();
    println!("password len is '{}' bytes, aligned size is '{}', padding byte is '{}'('{}')", password.len(), size, fill, size - password.len());

    let padding: String = std::iter::repeat(fill).take(size - password.len()).collect();
    let padded_password = format!("{password}{padding}");
    println!("padded password as bytes: {:?}", padded_password.as_bytes());

    let key = GenericArray::from_slice(key_bytes);

    let mut encrypt_block_128 = GenericArray::clone_from_slice(padded_password.as_bytes().clone());
    let encrypt_cipher_128 = Aes128::new(key);

    encrypt_cipher_128.encrypt_block(&mut encrypt_block_128);


    // println!("b64 encoded aes128 encrypted password: {:?}", general_purpose::STANDARD.encode(encrypt_block_128));

    let cipher = Cipher::new_128(<&[u8; 16]>::try_from(key_bytes).unwrap());
    let encrypted = cipher.cbc_encrypt(ivvec.as_slice(), password.as_bytes());
    println!("b64 encoded aes128 encrypted password: {:?}", general_purpose::STANDARD.encode(encrypted));


}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(launcher::LauncherApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
