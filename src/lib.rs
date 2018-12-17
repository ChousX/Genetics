mod nucleotide;
mod enums;
mod gene;//dep: nucleotide, rand, enums
mod chromosome;//dep: gene, rand, enums
mod genome;//dep: genome, rand, enums
//mod genetic_association_maker;

extern crate rand;