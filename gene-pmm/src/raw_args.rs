/// Read-only wrapper for the raw arguments passed to the program.
#[derive(Debug)]
pub struct RawArgs<'a> {
	pub args: &'a Vec<String>,
}

impl<'a> From<&'a Vec<String>> for RawArgs<'a> {
	#[inline(always)]
	fn from(args: &'a Vec<String>) -> Self {
		Self { args }
	}
}

impl ToString for RawArgs<'_> {
	#[inline(always)]
	fn to_string(&self) -> String {
		self.args.join(" ")
	}
}
