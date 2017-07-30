mod number_math;
mod mrt;
mod my_rng;
mod rsa;

// use my_rng::MyLogRandomIntGenerator;
use rsa::{get_rsa_keys, rsa_encrypt, rsa_decrypt};


fn main() {
    let (public_key, private_key)=get_rsa_keys(13636);
    let message=7071;
    let ciphertext=rsa_encrypt(message, public_key);

    println!("{:?}", private_key);
    println!("{:?}", public_key);
    println!("{:?}", ciphertext);
    assert!(rsa_decrypt(ciphertext, private_key)==message);
}