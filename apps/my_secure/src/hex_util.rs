// String literal -> bytes (&[u8]) -> hex String (String)
//      "".as_bytes() -> &[u8]
//      hex::encode(&[u8]) -> String
//
// hex String literal ->  Vec<u8> -> String
//      hex::decode("")? -> Vec<u8>
//      String::from_utf8(Vec<u8>) -> String 


// hex::encode convert a sequence of bytes into hexadecimal [ˌheksəˈdesɪml] string representation. 
fn hex_encode() -> anyhow::Result<()> {
    let data: &[u8] = "abc".as_bytes();

    let hex_data:String = hex::encode(data);
    println!("Hex encoded: {}", hex_data);
    Ok(())
}

fn hex_decode() -> anyhow::Result<()> {
    let data = "616263"; // hex representation of "abc"
    let decoded_data: Vec<u8> = hex::decode(data)?;
    let result_string: String = String::from_utf8(decoded_data)?;
    println!("Decoded: {}", result_string);
    Ok(())
}

pub fn caller() -> anyhow::Result<()> {
    hex_encode()?;
    hex_decode()?;

    Ok(())
} 