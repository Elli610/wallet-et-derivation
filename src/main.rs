
use rand::Rng;
use sha2::Sha512;
use hmac::{Hmac, Mac};
//use hex_literal::hex;

fn get_random_bytes() -> [u8; 16] {
    let random_bytes = rand::thread_rng().gen::<[u8; 16]>();

    return random_bytes;
}
   
fn master_private_key(private_key : [u8; 16] ) -> (Vec<u8>,Vec<u8>) {

    // Create alias for HMAC-SHA
    type HmacSha512 = Hmac<Sha512>;

    let mac = HmacSha512::new_from_slice(private_key.to_vec().as_slice()).unwrap().finalize().into_bytes();

    // master_private_key
    let master_private_key = mac[..32].to_vec();

    // master_chaincode
    let master_chaincode = mac[32..].to_vec();

    println!("mac: {:?}", mac);
    println!("Master private key: {:?}", master_private_key);
    println!("Master chain code: {:?}", master_chaincode);
    
    (master_private_key,master_chaincode)
}

fn main() {
    let a = get_random_bytes();
    println!("Random bytes: {:?}", a);
    //(master_priv,master_chaincode) = master_private_key(a);
    //println!("Master private key: {:?}", master_private_key(a));

}


