use clap::Parser;
use gene_pmm::{gene::GeneArgs, pm::PackageManager, spm::pacman::*};

fn main() {
    let args = GeneArgs::parse();
    let pacman_tranlation = Pacman::from_gene(&args);
    println!("{:#?}", pacman_tranlation);
}
