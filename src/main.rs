
use rand::Rng;
use sha2::Sha512;
use hmac::{Hmac, Mac};


fn get_random_bytes() -> [u8; 16] { 
    /*
    *  This function returns a random 16 byte array
    */
    let random_bytes = rand::thread_rng().gen::<[u8; 16]>();

    return random_bytes;
}

fn join_int(vect : Vec<u8>) -> String {
    /* concatenate the elements of vect to build one String */
    let out = vect.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("");
    return out;
}

fn master_key(private_key : [u8; 16] ) -> (Vec<u8>,Vec<u8>) {
    /* 
    * extract the master private key and the master chaincode from the private key
    */

    // Create alias for HMAC-SHA
    type HmacSha512 = Hmac<Sha512>;

    let mac = HmacSha512::new_from_slice(private_key.to_vec().as_slice()).unwrap().finalize().into_bytes();

    // master_private_key
    let master_private_key = mac[..32].to_vec();

    // master_chaincode
    let master_chaincode = mac[32..].to_vec();
    
    /*
    println!("mac: {:?}", mac);
    println!("Master private key: {:?}", master_private_key);
    println!("Master chain code: {:?}", master_chaincode);
    */

    (master_private_key,master_chaincode)


}

fn master_public_key(master_private_key : Vec<u8>) -> Vec<u8> {
    /*
    *  This function returns the master public key
    */
    type HmacSha512 = Hmac<Sha512>;
    let master_private_key = HmacSha512::new_from_slice(master_private_key.to_vec().as_slice()).unwrap().finalize().into_bytes();

    return Vec::from_iter(master_private_key.to_vec()[0..32].iter().cloned());
    
    
}



fn main() {
    let a = get_random_bytes();
    //println!("Random bytes: {:?}", a);
    //(master_priv,master_chaincode) = master_private_key(a);
    println!("Master private key: {:?}", join_int(master_key(a).0));
    println!("Master public key: {:?}", join_int(master_public_key(master_key(a).0)));
}

