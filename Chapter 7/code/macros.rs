macro_rules! welcome {
    // `()` indicates that the macro takes no argument
    () => (
        // the macro will expand into the contents of this block
        println!("Welcome to the Game!");
    )
}

macro_rules! mac1 {
    ($arg:expr) => (println!("arg is {}", $arg));
}

macro_rules! printall {
    ( $( $arg:expr ), * ) => ( {$( print!("{} / ", $arg) ); *} );
}

macro_rules! create_fn {
    ($fname:ident) => (
        fn $fname() {
            println!("Called the function {:?}()", stringify!($fname))
        }
    )
}

create_fn!(fn1);

macro_rules! massert {
    ($arg:expr) => (
            if $arg {}
            else { panic!("Assertion failed!"); }
    );
}

macro_rules! unless {
    ($arg:expr, $branch:expr) => ( if !$arg { $branch }; );
}

macro_rules! test_eq {
    ($name:ident, $left:expr, $right:expr) => {
        #[test]
        fn $name() {
            assert_eq!($left, $right);
        }
    }
}

test_eq!(seven_times_six_is_forty_two, 7 * 6, 42);
test_eq!(seven_times_six_is_not_forty_three, 7 * 6, 43);

fn main() {
    welcome!();  // Welcome to the Game!
    mac1!(42); // arg is 42
    mac1![42]; // arg is 42
    printall!("hello", 42, 3.14); // hello / 42 / 3.14 /
    fn1();
    // massert!(1 == 42);

    let v = [10, 40, 30];
    massert!(v.contains(&30));
    massert!(!v.contains(&50));
    unless!(v.contains(&25), println!("v does not contain 25"));

}
// Welcome to the Game!
// arg is 42
// arg is 42
// hello / 42 / 3.14 / Called the function "fn1"()
// thread '<main>' panicked at 'Assertion failed!'
// v does not contain 25