use my_rng::MyLogRandomIntGenerator;
use mrt::get_next_prime;
use number_math::get_modular_inverse;
use number_math::power_mod;

#[derive(Debug,Copy,Clone)]
pub struct RSAPublicKey(i64, i64);
#[derive(Debug,Copy,Clone)]
pub struct RSAPrivateKey(i64, i64);

pub fn get_rsa_keys(seed: i64) -> (RSAPublicKey, RSAPrivateKey){
    let mut random_generator=MyLogRandomIntGenerator::new(0,seed);

    let p=get_next_prime(random_generator.get(9)|512);
    let q=get_next_prime(random_generator.get(10)|1024);
    let n=p*q;

    let e={
        let mut e;
        loop{
            e=get_next_prime(random_generator.get(10));
            if (p-1)*(q-1)%e!=0{
                break;
            }
        }
        e
    };

    let d=get_modular_inverse(e, (p-1)*(q-1));

    (RSAPublicKey(e,n), RSAPrivateKey(d,n))
}

pub fn rsa_encrypt(message:i64, public_key: RSAPublicKey) -> i64{
    power_mod(message, public_key.0, public_key.1)
}

pub fn rsa_decrypt(ciphertext:i64, private_key: RSAPrivateKey) -> i64{
    power_mod(ciphertext, private_key.0, private_key.1)
}