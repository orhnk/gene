use gene_utils::args::GeneArgs;

pub trait PackageManager<'a>: TryFrom<&'a GeneArgs> {
	/// Convert Middle Representation (Self) to String
	fn compile(&'a self) -> String;
}