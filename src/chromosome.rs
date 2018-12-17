use std::fmt;
use enums::Mutation;
use gene::Gene;
use rand::prelude::*;

#[derive(Clone)]
pub struct Chromosome{
    genes: Vec<Gene>,
    size: usize,
    name: String
}
impl Chromosome{
    pub fn new()->Chromosome{
        Chromosome{
            genes: Vec::new(),
            size: 0,
            name: String::from("")
        }
    }
    pub fn push(&mut self, new_gene: Gene){
        self.genes.push(new_gene);
        self.size += 1;
    }
    pub fn pop(&mut self){
        self.genes.pop();
        self.size -= 1;
    }
    pub fn insert_gene(&mut self, gene_number: usize, new_gene: Gene){
        self.genes.insert(gene_number, new_gene);
        self.size += 1;
    }
    pub fn remove_gene(&mut self, gene_number: usize){
        self.genes.remove(gene_number);
        self.size -= 1;
    }
    pub fn size(&self) -> usize{
        self.size
    }
    pub fn get_gene(&self, gene_number: usize) -> Result<Gene, &'static str>{
        if gene_number > self.size {
            Result::Err("[Error][Chromosome][get_gene]
             gene_number is larger than the size of the chromosome")
        }
        else{
            Result::Ok(self.genes[gene_number].clone())
        }
    }
    pub fn set_gene(&mut self, gene_number: usize, new_gene: Gene){
        if gene_number > self.size{}
        else{
            self.genes[gene_number] = new_gene;
        }
    }
    pub fn get_name(&self)-> String{
        self.name.clone()
    }
    pub fn set_name(&mut self, new_name: String){
        self.name = new_name;
    }
    pub fn print(&self){
        let temp = self.genes.clone();
        for x in temp{
            print!("{}", x);
        }
        println!("");
    }
    pub fn cross(rng: &mut ThreadRng, chromosome1: &Chromosome, chromosome2: &Chromosome)->Result<Chromosome, &'static str>{
    if chromosome1.size == chromosome2.size{
        let mut new_chromosome: Chromosome = chromosome1.clone();
        let mut i: usize = 0;
        let genes = new_chromosome.genes.clone();
            for x in genes{
                let mut tmp = chromosome2.get_gene(i)?;
                let temp_gene = Gene::cross(rng, &x, &tmp);
                match temp_gene{
                    Result::Ok(x)=>{new_chromosome.set_gene(i, x)}
                    Result::Err(x)=>{println!("{}",x);}
                }
                i += 1;
            }
            Result::Ok(new_chromosome)
        }
        else{
            Result::Err("[Error][Chromosome][cross] chromosome1 != chromosome2")
        }    
    }
    pub fn mutate(&mut self, rng: &mut ThreadRng, chans: usize, mutation: &Mutation){
        for x in &mut self.genes{
            x.mutate(rng, chans, &mutation);
        }
    }

    //TODO macro! self-genrating self-setting
}

