fn main() {
    let start =30000500; // Starting point
    let end = 30001500; // Ending point 
    
    //Now check whether the starting point is greater than the finishing point. 
//If this is the case, the function terminates with an error message.
//then calls sieve_of_eratosthenes_in_range to compute the prime 
//numbers in the specified range
    if start > end {
        println!("Invalid range: start ({}) is greater than end ({})", start, end);
        return;
    }

    let primes = sieve_of_eratosthenes_in_range(start, end);

    println!("Prime numbers between {} and {} are: {:?}", start, end, primes);
}

/// Use the Eratosthenes Sieve to generate prime numbers within a specific range.
//If the end is less than two, this technique produces an empty vector since there 
//are no prime numbers fewer than two.
fn sieve_of_eratosthenes_in_range(start: usize, end: usize) -> Vec<usize> {
    if end < 2 {
        return vec![];
    }
//At initialization, A vector vector is constructed and used to 
//indicate if an integer is prime (true) or not prime.
//Goes from two to the square root of the end. For each number num,
// if is_prime[num] is true,it marks all multiples of num as false (non-prime).
    let mut is_prime = vec![true; end + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for num in 2..=((end as f64).sqrt() as usize) {
        if is_prime[num] {
            for multiple in (num * num..=end).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }
//Collects and returns the prime numbers in the specified range
    is_prime.iter()
        .enumerate()
        .filter(|&(num, &prime)| prime && num >= start)
        .map(|(num, _)| num)
        .collect()
}

