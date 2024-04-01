use std::{io::{self, Result},
str::from_utf8};

use hex::{encode_hex,decode_hex};
mod base64mod;
mod ui;
mod hex;
fn main ()->Result<()>{
ui::init_ui().unwrap();
// let mut input=String::new();
// io::stdin().read_line(&mut input).unwrap();
// let encoded_hex=encode_hex(&input);
// println!("{}",encoded_hex);
// println!("{}",String::from_utf8(decode_hex(&encoded_hex)).unwrap());
// println!("{}",String::from_utf8(decode_base64("YXNk").unwrap()).unwrap());



    Ok(())
}