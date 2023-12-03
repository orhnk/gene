use crate::gene::GeneArgs;

pub trait PackageManager {
	/// Translate gene representation to Self
	fn from_gene(args: GeneArgs) -> Self;
}
