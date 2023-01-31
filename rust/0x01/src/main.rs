fn main() {
    let mut multiples: Vec<i32> = Vec::new();

    let mut total_vector_sum : i32 = 0;

    multiples = populate_multiples_vector(3,5,1000);
    println!("total sum {}", calculate_sum(multiples));
}

fn populate_multiples_vector (multiples_of_int_a: i32, multiples_of_int_b: i32, below_int : i32) -> Vec<i32> {

    println!("debug multiple of {} or {}, under {}", multiples_of_int_a, multiples_of_int_b, below_int);  
    let mut multiples : Vec<i32> = Vec::new();

    for i in 1..below_int {
        if i % multiples_of_int_a == 0 || i % multiples_of_int_b == 0
        {
            multiples.push(i);
        }
    }
    return multiples;
}

fn calculate_sum(vector_in : Vec<i32>) -> i32 {
    let mut vector_sum = 0;
    for i in &vector_in {
        // iterate immutably
        let i: &i32 = i; // elements are immutable pointers
        vector_sum = vector_sum + i;
    }
    return vector_sum;
}