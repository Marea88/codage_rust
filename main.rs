use std::collections::HashMap;
struct Fibonacci {
    curr: u64,
    next: u64,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    type Item = u64;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there's no endpoint to a Fibonnaci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(self.curr)
    }
}

fn frequency(s: &str) -> HashMap<char, i32> {
    let mut h = HashMap::new();
    for char in s.chars() {
        let counter = h.entry(char).or_insert(0);
        *counter += 1;
    }
    h
}

const FIB: Fibonacci = Fibonacci { curr: 1, next: 1 };

fn fib_code(num: u64) -> String {
    let mut suite = FIB
        .take_while(|&x| num > x)
        .collect::<Vec<_>>()
        .into_iter()
        .rev(); //We use the iterator FIB to produce the value we need then reverse it to start with the biggest value.

    let mut rest = num - suite.next().unwrap();
    let mut code = "1".to_string();
    
    for i in suite {
        if rest > i && code.chars().last().unwrap() == '0' {
            rest -= i;
            code.push('1');
        } else {
            code.push('0');
        }
    }
    code.chars().rev().chain("1".chars()).collect() // We reverse the chain and return it.
}


fn fib_decode(x: &str) -> u64{
    x[..x.len()-1].chars()
    .zip(FIB)
    .filter(|x| x.0 == '1')
    .fold(0, |acc, (_, x)| acc + x)
}

fn main() {
    let _f = frequency("abaabcd");
    //println!("{:?}", f);
    let code = fib_code(30);
    println!("{}", code);
    println!("{:?}", fib_decode(&code));
}
