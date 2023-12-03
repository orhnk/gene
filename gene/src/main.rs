use gene_pmm::gene::GeneParams;
use clap::Parser;
fn main() {
    let args = GeneParams::parse();
    println!("{:#?}", args);
}
