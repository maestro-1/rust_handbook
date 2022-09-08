use core::num;

fn simple_datatypes() {
    // Integers
    // Floating point numbers
    // boolean
    // Characters

    // works for hex, dec, bina, Bytes, oct, etc
    let a = 98_222;

    // u means unsigned, only positive
    let f = 255;

    let fl = 2.0;

    let b = true;

    let c = '2';
}

fn compound_types(){
    // tuple
    let tup = ("Learning rust", 1);
    println!("{}",tup.0);

    // arr
    let arr = [200, 404, 300];
    let not_found = arr[1];
    let byte = [0; 8];
}

fn control_workflow_condition(num: i32){
    if num < 10 {
        println!("Number < 10")
    } else if num > 10 {
        println!("Number > 10")
    } else {
        println!("I got nothing bro")
    }
}

fn control_loops(){
    let mut counter = 0;

    // builtin loop alternative to while loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("{}", result);

    // for loop for range
    for number in 1..10{
        println!("{}", number);
    }

    let arr = [200, 404, 300];
    for num in arr.iter(){
        println!("{}", num)
    }
}