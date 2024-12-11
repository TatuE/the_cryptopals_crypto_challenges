pub fn convert_hex_to_base64(hex_s: &str) -> String{

    base64::encode(hex::decode(hex_s).unwrap())
}
 
pub fn fixed_xor(hex_s_a: &str, hex_s_b: &str) -> String{
    let hex_a: Vec<u8> = hex::decode(hex_s_a).unwrap();
    let hex_b: Vec<u8> = hex::decode(hex_s_b).unwrap();
    let mut h_xor: Vec<u8> = Vec::new();

    for (a, b) in hex_a.iter().zip(hex_b.iter()) {
        h_xor.push(a ^ b);
    }
    
    hex::encode(h_xor)
}