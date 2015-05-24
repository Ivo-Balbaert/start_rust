struct Big {
	field1: i32,
	field2: i32,
	field3: i32,
	field4: i32,
	field5: i32,
	field6: i32,
	field7: i32,
	field8: i32,
	field9: i32
} 

fn foo(b: Big) {
	let Big { field6: x, field3: y, ..} = b;
	println!("pulled out {} and {}", x, y);
	let Big { field6, field3, ..} = b;
	println!("pulled out {} and {}", field3, field6);
	let Big { field3: ref x, ref field6, ..} = b;
	println!("pulled out {} and {}", *x, *field6);
}

fn main() {
	let big = Big {field1: 1, field2: 2, field3: 3, field4: 4,
				   field5: 5, field6: 6, field7: 7,	field8: 8, field9: 9};
	foo(big); 
}
// pulled out 6 and 3
// pulled out 3 and 6
// pulled out 3 and 6