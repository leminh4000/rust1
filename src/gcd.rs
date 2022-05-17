pub(crate) fn run() {
    let m:u64=110323242243240142;
    let n: u64 =11001212342410004;
    println!("GCD of {} and {} is {}",m,n,gcd(m,n));
}

pub(crate) fn gcd(mut n: u64, mut m: u64)-> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
