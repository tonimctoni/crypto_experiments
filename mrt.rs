use number_math::power_mod;

pub fn miller_rabin_test(n: i64, witness: i64) -> bool {
    assert!(1<witness && witness<n-1);
    if n%2==0 {
        return false;
    }

    let k=(n-1).trailing_zeros();
    let m=(n-1)>>k;
    // let mut b=witness.pow(m as u32)%n;
    let mut b=power_mod(witness, m, n);
    if b==1 || b==n-1{
        return true;
    }
    for _ in 0..(k-1){
        b=b.pow(2)%n;
        if b==n-1 {
            return true;
        } else if b==1 {
            return false;
        }
    }
    return false;
}

pub fn is_prime(n: i64) -> bool{
    if n<2{
        return false;
    }
    if n==2 || n==3{
        return true;
    }

    (2..(n-1))
    .take(10) // Should use random numbers, but meh
    .all(|i| miller_rabin_test(n,i))
}

pub fn get_next_prime(mut n: i64) -> i64{
    while !is_prime(n){
        n+=1;
    }
    n
}