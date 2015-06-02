use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let mut magicians = Arc::new(Mutex::new(vec!["Morgan".to_string(),
  	"Allanon".to_string(), "Jafar".to_string()]));

    for i in 0..3 {
    	let magicians = magicians.clone();
        thread::spawn(move || {
    	    let mut mags = magicians.lock();
    	    match mags {
	 			Ok(mut mags) => 
	 			{ 
	 				let mut temp = mags[i].to_string();
	 				temp.push_str("ius");
	 				mags[i] = temp;
 	 			},
	 			Err(str) => println!("{}", str)
			}
        }).join();
    }
   	println!("{:?} - ", *magicians);
    print!("{:?} - ", *magicians.lock().unwrap());
}
// Mutex { data: ["Morganius", "Allanonius", "Jafarius"] } - [Finished in 29.6s]
// ["Morganius", "Allanonius", "Jafarius"]