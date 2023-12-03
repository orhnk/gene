use clap::Parser;
use gene_utils::args::GeneArgs;
use gene_pmm::spm::pacman::*;


// NOTE: I wanted to use `anyhow` for error handling, but it doesn't work with limited lifetimes
// (dynamic dispatch makes it impossible to know what the lifetime of the variable is)

// TODO: improve overall error handling

fn main() {
    let args: GeneArgs = GeneArgs::parse();
    let pacman_repr = Pacman::try_from(&args).expect("failed to create pacman");
    println!("{:#?}", pacman_repr);
}
