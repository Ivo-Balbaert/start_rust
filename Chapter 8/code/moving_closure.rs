use std::thread;

fn main() {
    read();
}

fn read() {
    let book = read_book("book1.txt");
// error: closure may outlive the current function, but it borrows `book`, 
// which is owned by the current function 
    // thread::spawn(|| {
    //     println!("{:?}", book);
    // });

    thread::spawn(move || {
        println!("{:?}", book);
    });
}

fn read_book(s: &str) { }
