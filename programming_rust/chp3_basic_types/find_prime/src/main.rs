
// use std::io::Write;
// use std::str::FromStr;


fn find_prime(n: usize) -> Vec<u64> {
    let mut sieve: Vec<bool> = vec![true; n];
    for i in 2..((n as f64).sqrt().floor() as usize) {
        if sieve[i] {
            let mut j = i * i;
            while j < n {
                sieve[j] = false;
                j += i;
            }
        }
    }

    let mut prime_numbers: Vec<u64> = Vec::new();

    for (i, is_prime) in sieve.into_iter().enumerate().skip(2) {
        if is_prime {
            prime_numbers.push(i as u64);
        }
    }

    prime_numbers
}


fn main() {
    let v: Vec<u64> = (0..6).collect();
    let a: [u64; 5] = [1, 2, 4, 7, 11];
    let sv: &[u64] = &v;
    let sa: &[u64] = &a;

    println!("Array operations demo:");
    print(sv);
    print(sa);

    print(&v);
    print(&a);

    print(&v[0..1]);
    println!("{}", v[0]);

    println!("List all prime numbers smaller than 42:");
    print(&find_prime(42));
}


fn print(n: &[u64]) {
    for elem in n {
        println!("{}", elem);
    }
}
