mod modul1;
mod modul2;

// use modul1::func1;

fn main() {
	modul1::func1();
	modul2::func1();

	// func1(); // error: unresolved name `func1`
	// func1(); // works when use modul1::func1; is added
}
// called func1 from modul1
// called func1 from modul2