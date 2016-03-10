use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
//use rand::{ Rng, OsRng };
//use std::str;
//use rustc_serialize::base64::{Config, Newline, FromBase64, ToBase64, STANDARD, URL_SAFE};
use rustc_serialize::base64::{FromBase64, ToBase64, STANDARD};

pub fn aes_encrypt_string(data: &str) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let v=aes_encrypt_bytes(data.as_bytes());
    v
}

pub fn aes_encrypt_bytes(data: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    //let mut key: [u8; 32] = [10; 32];
    //let mut iv: [u8; 16] = [12; 16];
    //let mut rng = OsRng::new().ok().unwrap();
    //rng.fill_bytes(&mut key);
    //rng.fill_bytes(&mut iv);
    let key: [u8; 32] = [10; 32];
    let iv: [u8; 16] = [12; 16];
    
    let mut encryptor = aes::cbc_encryptor( aes::KeySize::KeySize256, &key, &iv, blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    Ok(final_result)
}

pub fn aes_decrypt_to_string(encrypted_data: &[u8]) -> Result<String, symmetriccipher::SymmetricCipherError> {
    let v=aes_decrypt_to_bytes(encrypted_data);
    Ok(String::from_utf8(v.ok().unwrap()).unwrap())
}
pub fn aes_decrypt_to_bytes(encrypted_data: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    //let mut key: [u8; 32] = [10; 32];
    //let mut iv: [u8; 16] = [12; 16];
    //let mut rng = OsRng::new().ok().unwrap();
    //rng.fill_bytes(&mut key);
    //rng.fill_bytes(&mut iv);
    let key: [u8; 32] = [10; 32];
    let iv: [u8; 16] = [12; 16];
    
    let mut decryptor = aes::cbc_decryptor( aes::KeySize::KeySize256, &key, &iv, blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    Ok(final_result)
}


pub fn base64_encode_bytes(data:&[u8])->Option<String>{
    let result=data.to_base64(STANDARD);
    Some(result)
}

pub fn base64_encode_string(data:&str)->Option<String>{
    let result= base64_encode_bytes(data.as_bytes());
    result
}
pub fn base64_decode_to_bytes(decoded_data:&str)->Option<Vec<u8>>{
    let v=decoded_data.from_base64().unwrap();
    Some(v)
}

pub fn base64_decode_to_string(decoded_data:&str)->Option<String>{
    let v=base64_decode_to_bytes(decoded_data);
    match v{
        Some(bytes)=>Some(String::from_utf8(bytes).unwrap()),
        _=>None,
    }
}
/*
fn main() {
    let message = "三巨头72分骑士胜 Hello World!";
    let encrypted_data = aes_encrypt_string(message).ok().unwrap();
    let decrypted_data = aes_decrypt_to_string(&encrypted_data[..]).ok().unwrap();

    println!("{:?}",encrypted_data);
    println!("{}",decrypted_data);

    let encoded_s=base64_encode_string(message).expect("");
    println!("{}",encoded_s);

    //let decoded_s=String::from_utf8(encoded_s.from_base64().unwrap()).unwrap();
    let decoded_s=base64_decode_to_string(&encoded_s).expect("");
    println!("{}",&decoded_s);

}
*/
