use std::f64;

struct Complex {
	real: f64,
	imag: f64
}

impl Complex {
	fn new(re: f64, im: f64) -> Complex {
		Complex{ real: re, imag: im }
	}

	fn to_string(&self) -> String {
		if self.imag > 0.0 { format!("{} + {}i", self.real, self.imag) }
		else if self.imag < 0.0 { format!("{} - {}i", self.real, 
										f64::abs(self.imag)) }
		else { format!("{}", self.real) }
	}

	fn add(&self, c: Complex) -> Complex {
		Complex{ real: self.real + c.real, imag: self.imag + c.imag }
	}

	fn times_ten(&mut self)  {
		self.real = 10.0 * self.real;
		self.imag =  10.0 * self.imag;
	}

	fn abs(&self) -> f64 {
		f64::sqrt((self.real * self.real) + (self.imag * self.imag))
	}

}

fn main() {
	let c1 = Complex{ real: 2.0, imag: 5.0 };
	// same number, but with constructor new:
	let mut c2 = Complex::new(2.0, 5.0);
	// println!("{:?}", c1);
	println!("{}", c2.to_string());  // 2 + 5i
	let c3 = Complex::new(2.0, -5.0);
	println!("{}", c3.to_string());  // 2 - 5i
	let c4 = Complex::new(1.2, 4.2);
	println!("{}", c2.add(c4).to_string());  // 3.2 + 9.2i
	println!("{}", c2.add(c3).to_string());  // 4
	// println!("{}", c2.times_ten().to_string());  // 20 + 50i
	c2.times_ten();
	println!("{}", c2.to_string());  // 20 + 50i
	println!("{}", c1.abs()); // 5.385165
}