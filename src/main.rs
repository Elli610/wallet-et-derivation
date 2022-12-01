
use rand::Rng;
use sha2::{Sha256, Sha512, Digest};
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

fn join_hex(vect : Vec<String>) -> String {
    /* concatenate the elements of vect to build one String */
    let out = vect.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
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

fn hash_sha256(data: String) -> String {
    
    let mut hasher = Sha256::new();


    hasher.update(data.as_bytes());

    
    let result = hasher.finalize();

    //println!("data: {:?}", data);
    //println!("Hash: {:?}", result);

    return format!("{:X}", result);   
}

fn merkle_Tree(data : Vec<String>) -> String {
    /*
    Return the merkle root of the tree
     */
    let mut merkle_tree = data.to_vec();
    let root = "0".to_string();

   //hash all the data of the initial vector
   for i in 0..data.len() {
       let hash = hash_sha256(merkle_tree[i].to_string());
       merkle_tree[i] = hash;
    }
    
    // build the merkle tree
    while merkle_tree.len() > 1 {
        let mut new_merkle_tree = Vec::new();
        let mut i = 0;
        while i < merkle_tree.len() {
            if i == merkle_tree.len() - 1 {
                new_merkle_tree.push(merkle_tree[i].to_string());
            } else {
                let hash = hash_sha256(merkle_tree[i].to_string() + merkle_tree[i + 1].to_string().as_str());
                new_merkle_tree.push(hash);
            }
            i += 2;
        }
        merkle_tree = new_merkle_tree;
    }

    return merkle_tree[0].to_string();

}


fn main() {
    let a = get_random_bytes();
    //println!("Random bytes: {:?}", a);
    //(master_priv,master_chaincode) = master_private_key(a);



    println!("Master private key: {:?}", join_int(master_key(a).0));
    println!("Master public key: {:x?}", "04".to_owned() + &join_int(master_public_key(master_key(a).0)));


    // test de merkleTree 
    let mut data = Vec::new();
    for i in master_key(a).0 {
        data.push(i.to_string());
    }
    println!("merkle : {:?}",merkle_Tree(data));
    //hash_sha256("hello".to_string());
}