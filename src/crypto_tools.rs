
pub fn convert_hex_to_base64(hex_s: &str) -> String{

    let s:String = base64::encode(hex::decode(hex_s).unwrap());
    
    s
}