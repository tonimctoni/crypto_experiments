// fn power_mod(base: i64, exponent: i64, modulo: i64) -> i64 {
//     if exponent<0 || modulo<1 {
//         panic!("exponent<0");
//     } else if exponent==0 {
//         1
//     } else if exponent==1 {
//         base%modulo
//     } else {
//         use std::mem;
//         let mut result=1;
//         for i in (0..(mem::size_of_val(&exponent)*8-exponent.leading_zeros() as usize)).rev()
//         {
//             result=(result*result)%modulo;
//             if (exponent>>i)&1!=0{
//                 result=(result*base)%modulo;
//             }
//         }
//         result
//     }
// }

pub fn power_mod(base: i64, exponent: i64, modulo: i64) -> i64 {
    if exponent<0 || modulo<1 {
        panic!("exponent<0");
    } else if exponent==0 {
        1
    } else if exponent==1 {
        base%modulo
    } else {
        let mut base=base%modulo;
        let mut result=1;
        let mut remaining_exponent=exponent;
        while remaining_exponent>0 {
            if remaining_exponent%2==1{
                result=(result*base)%modulo;
            }
            base=(base*base)%modulo;
            remaining_exponent>>=1;
        }
        result
    }
}

pub fn get_modular_inverse(mut a: i64, mut m: i64) -> i64{
    let m0:i64=m;
    let mut t:i64;
    let mut q:i64;

    let mut x0:i64=0;
    let mut x1:i64=1;

    if m==1{
        return 0;
    }

    while a>1{
        q=a/m;

        // let (m,a)=(a%m,m);
        t=m;
        m=a%m;
        a=t;

        // let (x0,x1)=(x1-q*x0, x0);
        t=x0;
        x0=x1-q*x0;
        x1=t;
    }

    if x1<0{
        x1+=m0;
    }

    x1
}