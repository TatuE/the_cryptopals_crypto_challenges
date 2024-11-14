
pub fn convert_hex_to_base64(hex_s: &str) -> String{

    let b64:String = base64::encode(hex::decode(hex_s).unwrap());
    
    b64
}
/* 
pub fn fixed_xor(hex_s_1: &str, hex_s_2: &str) -> String{
    
}
*/