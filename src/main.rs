use rand::Rng;


fn get_random_bytes() -> [u8; 16] {
    let random_bytes = rand::thread_rng().gen::<[u8; 16]>();

    return random_bytes;
}
   


fn main() {
    let a = get_random_bytes();
    println!("Random bytes: {:?}", a);

}

