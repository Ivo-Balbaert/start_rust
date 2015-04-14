fn main() {
	let sum = (0..101).fold(0, |sum, n| sum + n);
	println!("{}", sum); // 5050

	let prcub = (1..6).fold(1, |prcub, n| prcub * (n * n * n));
	println!("{}", prcub); // 1728000

	let arr = [1, 9, 2, 3, 14, 12]; 
	let res = arr.iter().fold(0, |acc, item| acc - *item);
	println!("{}", res) // -41
}