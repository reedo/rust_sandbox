/// Runs this file. Test Change.
pub fn run() {
	println!("Running `print.rs`");

	// Basic formatting
	println!("	Hello, my name is {} from {}", "Dan", "Bristol");

	// Positional arguments
	println!(
		"	{0} is from {1} and {0} likes to {2}",
		"Dan", "Bristol", "dance"
	);

	// Named arguments
	println!(
		"	{name} likes to {activity}",
		name = "Dan",
		activity = "dance"
	);

	// Placeholder traits
	println!(
		"	{number} can be written as:
		Binary = {number:b}
		Hex    = {number:x}
		Octal  = {number:o}",
		number = 10
	);

	// Placeholder for debug trait
	println!("	A printed out tuple: {:?}", (17, true, 0.6, "hello"));
}
