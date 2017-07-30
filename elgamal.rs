use my_rng::MyLogRandomIntGenerator;
use number_math::{power_mod, is_primitive_root_of_prime, get_modular_inverse};
use mrt::{get_next_prime};

#[derive(Debug,Copy,Clone)]
pub struct ElGamalPublicKey(i64, i64, i64);
#[derive(Debug,Copy,Clone)]
pub struct ElGamalPrivateKey(i64, i64, i64);
#[derive(Debug,Copy,Clone)]
pub struct ElGamalEncryptedData(i64, i64);

pub fn gen_elgamal_keys(seed: i64) -> (ElGamalPublicKey, ElGamalPrivateKey) {
    let mut random_generator=MyLogRandomIntGenerator::new(0,seed);

    let prime=get_next_prime(random_generator.get(10)|1024);
    let primitive_root={
        let mut primitive_root;
        loop{
            primitive_root=random_generator.get(10);
            if primitive_root<2{
                continue;
            }

            if is_primitive_root_of_prime(primitive_root, prime){
                break;
            }
        }
        primitive_root
    };

    let private_key=random_generator.get(9)+2;//must be < prime
    // let private_key=2;
    assert!(private_key<prime);
    let public_key=power_mod(primitive_root, private_key, prime);

    (ElGamalPublicKey(public_key, primitive_root, prime), ElGamalPrivateKey(private_key, primitive_root, prime))
}

fn get_bits(mut n: i64) -> i64{
    let mut result=0;
    while n!=0{
        n>>=1;
        result+=1;
    }
    result
}

pub fn elgamal_encrypt(message:i64, public_key: ElGamalPublicKey, seed: i64) -> ElGamalEncryptedData{
    let mut random_generator=MyLogRandomIntGenerator::new(0,seed);

    let k=random_generator.get(get_bits(public_key.2)-1)+2;//must be < prime
    assert!(k<public_key.2);

    let key=power_mod(public_key.0, k, public_key.2);

    ElGamalEncryptedData(power_mod(public_key.1, k, public_key.2), message*key%public_key.2)
}

pub fn elgamal_decrypt(encrypted_data: ElGamalEncryptedData, private_key: ElGamalPrivateKey) -> i64{
    let den=get_modular_inverse(power_mod(encrypted_data.0, private_key.0, private_key.2), private_key.2);
    encrypted_data.1*den%private_key.2
}