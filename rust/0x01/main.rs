fn main() {
    //let mut multiplesThree : Vec<i32> = Vec::new();
    //let mut multiplesFive : Vec<i32> = Vec::new();
    //multiplesThree = populateMultiplesVector();
    //multiplesFive = populateMultiplesVector();
    populateMultiplesVector(5,10);
}

fn populateMultiplesVector(multiplesOfInt: i32, belowInt : i32) {
    let mut multiples : Vec<i32> = Vec::new();
    //let multiple = 0;
    for i in (1..belowInt){

        //let does_not_exist = &v[100];
        //let does_not_exist = v.get(100);

        //for i in &v {
        //    println!("{i}");
        //}

        multiples.push(multiplesOfInt * i);
        //let multiple = multiples[i];
        println!("debug {}", i);

        //let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
        //println!("Pangram: {}", pangram);
    
    }

    
}

//fn calculateSum(vectorA : , vectorB: ) {
//    println!("Another function.");
//}