fn main() {
    println!("Hello, world!");
    do_a_backflip();
}

fn do_a_backflip() {
    println!("Just some changes to create a release with");
    let mut counter = 0;

    loop { 
        counter += 1;
        if counter == 1_000_000 {
            break
        }
    }

    println!("Goodbye");
}


