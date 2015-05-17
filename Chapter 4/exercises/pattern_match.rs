fn main() {
	// exhaustive match with _:
    let magician = "Gandalf";
    match magician {
    	"Gandalf" => println!("A good magician!"),
      	_         => println!("No magician turned up!"),
    //	"Sauron"  => println!("A magician turned bad!") // error: unreachable pattern [E0001]
    }
}