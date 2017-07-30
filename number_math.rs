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

// #[allow(dead_code)]
// fn print_primitive_roots(p: i64) -> Vec<i64>{
//     let mut primitive_roots=vec!();
//     for g in 2..p{
//         // println!("{:?}", g);
//         if (1..(p-1)).all(|x| power_mod(g,x,p)!=1){
//             // println!("#### --> {:?}", g);
//             primitive_roots.push(g);
//         }
//     }
//     // println!("{:?}", primitive_roots);
//     primitive_roots
// }

// #[allow(dead_code)]
// fn print_primitive_roots_of_prime(p: i64){

//     let factors={
//         let mut factors=vec!();
//         let mut s=p-1;
//         let mut i=2;

//         while s!=1{
//             while s%i!=0 {i+=1;}
//             while s%i==0 {s/=i;}
//             factors.push(i);
//         }

//         factors
//     };

//     for g in (2..p)
//     .filter(|g| factors
//         .iter()
//         .all(|factor| 
//             power_mod(*g,(p-1)/factor,p)!=1)
//         ).skip((p/256) as usize).take(20){
//         println!("{:?}", g);
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

pub fn get_modular_inverse(mut a: i64, mut m: i64) -> i64{//gcd(a,m) must be 1
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

pub fn is_primitive_root_of_prime(root: i64, prime: i64) -> bool{
    assert!(prime>2);
    assert!(root>=2);
    let mut s=prime-1;
    let mut factor=2;

    while s!=1{
        while s%factor!=0 {factor+=1;}
        while s%factor==0 {s/=factor;}

        if power_mod(root,(prime-1)/factor,prime)==1 {
            return false;
        }
    }

    return true;
}