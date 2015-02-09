fn main() {
	on_windows();
}

#[cfg(target_os = "windows")]
fn on_windows() {
    println!("This machine has Windows as its OS.")
}
// This machine has Windows as its OS.