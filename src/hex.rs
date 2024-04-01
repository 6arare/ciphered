use hex::{decode,encode};

pub fn encode_hex (plain_text:&String)-> String{
    return encode(plain_text)
}
pub fn decode_hex   (plain_text:&String)->Vec<u8>{
    return decode(plain_text).unwrap()
}