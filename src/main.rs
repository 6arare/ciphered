use std::io::Result;

use crate::base64mod::*;
mod base64mod;
// mod ui;
fn main ()->Result<()>{
println!("{}",encode_base64("asd"));
println!("{}",String::from_utf8(decode_base64("YXNk").unwrap()).unwrap());
    Ok(())
}