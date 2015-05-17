fn main() {
    // while loop:
	let max_power = 10;
	let mut power = 1;
	while power < max_power {
        print!("{} ", power);  // prints without newline
        power += 1;	// increment counter
    }

	// infinite loop with continue and break:
    println!("I gain Infinite power!");
    loop {
        power += 1;
        if power == 42 {
            // Skip the rest of this iteration
            continue;
        }
        print!("{} ", power);
        if power == 50 {
            print!("OK, that's enough for today");
            break;  // exit the loop
        }
    }

    println!("");
    // use of labels:
    'outer: loop {
        print!("Entered the outer dungeon - ");
        'inner: loop {
            println!("Entered the inner dungeon - ");
            // break; // breaks only from the inner loop
            break 'outer; // breaks to the outer loop
        }
        // error: unreachable statement
        // println!("This treasure can sadly never be reached - ");
    }
    println!("Exited the outer dungeon!");

    // for in loop:
    for n in 1..11 { 	
    	println!("The square of {} is {}", n, n * n);
    }

    let mut x = 10;
    for _ in 1 .. x { x -= 1; print!("."); } 
    
}
// 1 2 3 4 5 6 7 8 9 I gain Infinite power!
// 11  12  13  14  15  16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  31  32  33  34  35  36  37  38  39  40  41  43  44  45  46  47  48  49  50 OK, that's enough for today
//
// Entered the outer dungeon
// Entered the inner dungeon
// Exited the outer dungeon!
//
// The square of 1 is 1
// The square of 2 is 4
// The square of 3 is 9
// The square of 4 is 16
// The square of 5 is 25
// The square of 6 is 36
// The square of 7 is 49
// The square of 8 is 64
// The square of 9 is 81
// The square of 10 is 100
// .........