extern crate rand;
use rand::prelude::*;

mod nucleotide;
use nucleotide::Nucleotide::*;

mod enums;
use enums::Mutation;

mod gene;
use gene::Gene;

mod chromosome;
use chromosome::Chromosome;

mod genome;
use genome::Genome;



fn main(){
    let mut rng = thread_rng();
    let mut gene1 = Gene::new();
    let mut gene2 = Gene::new();
    let mut gene3 = Gene::new();
    let mut gene4 = Gene::new();
    for _ in 0..4{
        gene1.push(A);
        gene2.push(G);
        gene3.push(C);
        gene4.push(T);
    }

    let mut chromosome1 = Chromosome::new();
    let mut chromosome2 = Chromosome::new();
    let mut chromosome3 = Chromosome::new();
    let mut chromosome4 = Chromosome::new();
    chromosome1.set_name(String::from("1"));
    chromosome2.set_name(String::from("2"));
    chromosome3.set_name(String::from("3"));
    chromosome4.set_name(String::from("4"));
    for _ in 0..4{
        chromosome1.push(gene1.clone());
        chromosome2.push(gene2.clone());
        chromosome3.push(gene3.clone());
        chromosome4.push(gene4.clone());
    }

    let mut chromosome5 = Chromosome::cross(&mut rng, &chromosome1, &chromosome2).unwrap();
    chromosome5.set_name(String::from("5"));

    let mut genome1 = Genome::new();
    genome1.push(chromosome1.clone());
    genome1.push(chromosome2.clone());
    genome1.push(chromosome3.clone());
    genome1.push(chromosome4.clone());
    genome1.push(chromosome5.clone());

    genome1.print();
    genome1.mutate(&mut rng, 2, &Mutation::Random_Point);
    genome1.print();
}