use base64::prelude::*;
pub fn encode_base64 (input:&str) -> String{
return BASE64_STANDARD.encode(input);

}
pub fn decode_base64 (input:&str) -> Result<Vec<u8>, base64::DecodeError>{
return BASE64_STANDARD.decode(input);

}
