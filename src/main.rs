
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

fn master_key(private_key : [u8; 16] ) -> (String,String) {
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
    
    
    println!("mac: {:?}", mac);
    println!("Master private key: {:?}", master_private_key);
    println!("Master chain code: {:?}", master_chaincode);
    

    (join_int(master_private_key),join_int(master_chaincode))


}





fn main() {
    let a = get_random_bytes();
    println!("Random bytes: {:?}", a);
    //(master_priv,master_chaincode) = master_private_key(a);
    println!("Master private key: {:?}", master_key(a).0);

}


