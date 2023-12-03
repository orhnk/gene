use crate::gene::GeneArgs;

pub trait PackageManager<'a>: From<&'a GeneArgs> {
	/// Convert Middle Representation (Self) to String
	fn compile(&'a self) -> String;
}
