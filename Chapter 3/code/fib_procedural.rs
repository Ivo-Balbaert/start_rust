fn main() {     
	let ans = fib(10);     
	println!("{}", ans); 
}

fn fib(x: i64) -> i64 {
     if x == 0 || x == 1 { return x; }
     fib(x - 1) + fib(x - 2) 
} 
// 55