use rand::Rng;
use sha2::Sha512;
use hmac::{Hmac, Mac};
use hex_literal::hex;

fn get_random_bytes() -> [u8; 16] {
    let random_bytes = rand::thread_rng().gen::<[u8; 16]>();

    return random_bytes;
}
   
fn master_private_key(private_key : [u8; 16] ) {
    let mut master_private = [0u8; 16];

    // Create alias for HMAC-SHA256
    type HmacSha512 = Hmac<Sha512>;

    let mac = HmacSha512::new_from_slice(private_key.to_vec().as_slice()).unwrap();
    println!("mac: {:?}", mac);
    
}

fn main() {
    let a = get_random_bytes();
    println!("Random bytes: {:?}", a);
    master_private_key(a)
    //println!("Master private key: {:?}", master_private_key(a));

}

