fn main() {
    let fibonacci_a = 1;
    let fibonacci_b = 2;
    let fibonacci_max = 4000000;
    println!("total even sum starting with {} and {} under {} is {}", fibonacci_a, fibonacci_b, fibonacci_max, calculate_even_fibonacci_sum(fibonacci_a,fibonacci_b,fibonacci_max));
}

fn calculate_even_fibonacci_sum(fibonacci_a : i128, fibonacci_b : i128, fibonacci_max : i128) -> i128 {
    
    let mut even_fibonacci_sum = 0;
    let mut fibonacci_last = fibonacci_a;
    let mut fibonacci_current = fibonacci_b;
    let mut fibonacci_temp = 0;

    if fibonacci_last % 2 == 0 {
        even_fibonacci_sum = even_fibonacci_sum + fibonacci_last;    
    }

    while fibonacci_current < fibonacci_max {
        if fibonacci_current % 2 == 0 {
            even_fibonacci_sum = even_fibonacci_sum + fibonacci_current;    
        }
        fibonacci_temp = fibonacci_current;
        fibonacci_current = fibonacci_current + fibonacci_last;
        fibonacci_last = fibonacci_temp;
    }
    return even_fibonacci_sum;
}