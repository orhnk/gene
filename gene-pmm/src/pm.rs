use crate::gene::GeneArgs;

pub trait PackageManager {
    /// Translate gene representation to Self
    fn from_gene(args: &GeneArgs) -> Self;

    /// Convert Middle Representation (Self) to String
    fn compile(&self) -> String;
}
