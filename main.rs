mod number_math;
mod mrt;
mod my_rng;
mod rsa;
mod elgamal;

use rsa::*;
use elgamal::*;


fn main() {
    let (public_key, private_key)=get_rsa_keys(13636);
    let message=7071;
    let ciphertext=rsa_encrypt(message, public_key);

    println!("{:?}", private_key);
    println!("{:?}", public_key);
    println!("{:?}", ciphertext);
    assert!(rsa_decrypt(ciphertext, private_key)==message);


    let (public_key, private_key)=gen_elgamal_keys(7467456);
    let message=1093;
    let encrypted_data=elgamal_encrypt(message, public_key, 9102);

    println!("{:?}", private_key);
    println!("{:?}", public_key);
    println!("{:?}", encrypted_data);
    assert!(elgamal_decrypt(encrypted_data, private_key)==message);
}