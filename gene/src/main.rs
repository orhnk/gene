use clap::Parser;
use gene_pmm::{gene::GeneArgs, spm::pacman::*};

fn main() {
	let args = GeneArgs::parse();
	let pacmane_repr = Pacman::from(&args);
	println!("{:#?}", pacmane_repr);
}
