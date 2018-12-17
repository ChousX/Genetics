use std::fmt;

use nucleotide::Nucleotide;
use nucleotide::Nucleotide::*;

use enums::Mutation;

use rand::prelude::*;

#[derive(Clone)]
pub struct Gene{
    nucleotides: Vec<Nucleotide>,
    size: usize
}
impl Gene{ //basic funtions
    pub fn new() -> Gene{
        Gene{
            nucleotides: Vec::new(),
            size: 0
        }
    }
    pub fn push(&mut self, new_nucleotide: Nucleotide){
        self.nucleotides.push(new_nucleotide);
        self.size += 1;
    }
    pub fn pop(&mut self){
        self.nucleotides.pop();
        self.size -= 1;
    }
    pub fn insert_nucleotide(&mut self, nucleotide_number: usize, new_nucleotide: Nucleotide){
        self.nucleotides.insert(nucleotide_number, new_nucleotide);
        self.size += 1;
    }
    pub fn remove_nucleotide(&mut self, nucleotide_number: usize){
        self.nucleotides.remove(nucleotide_number);
        self.size -= 1;
    }
    pub fn size(&self) -> usize{
        self.size
    }
    pub fn get_nucleotide(&self, nucleotide_number: usize) -> Result<Nucleotide, &'static str>{
        if nucleotide_number > self.size {
            Result::Err("[Error][Gene][get_nucleotide]
             nucleotide_number is larger than the size of the gene")
        }
        else{
            Result::Ok(self.nucleotides[nucleotide_number])
        }
    }
    pub fn set_nucleotide(&mut self, nucleotide_number: usize, new_nucleotide: Nucleotide){
        if nucleotide_number > self.size{}
        else{
            self.nucleotides[nucleotide_number] = new_nucleotide;
        }
    }
    pub fn print(&self){
        let temp = self.nucleotides.clone();
        for x in temp{
            print!("{}", x);
        }
        println!("");
    }
}
impl Gene{
    pub fn mutate(&mut self, rng: &mut ThreadRng, chans: usize, mutation: &Mutation){
        match mutation{
            Mutation::Point_shift =>{

            },
            Mutation::Random_Point =>{
                for x in 0..self.size{
                    if rng.gen_range(0,chans) == 0{
                    let m = rng.gen_range(1,4);
                        match m{
                            1 => self.nucleotides[x] = A,
                            2 => self.nucleotides[x] = G,
                            3 => self.nucleotides[x] = C,
                            4 => self.nucleotides[x] = T,
                            _ =>{}
                            };
                        }    
                }
            }
        };
    }
    pub fn cross(rng: &mut ThreadRng, gene1: &Gene, gene2: &Gene) -> Result<Gene, &'static str>{
        if gene1.size == gene2.size{
            let mut new_gene: Gene = gene1.clone();
            let first = rng.gen_range(0, gene1.size);
            let secont= rng.gen_range(first, gene1.size);
            for x in first..secont{
                new_gene.set_nucleotide(x, gene2.get_nucleotide(x)?);
            }
            Result::Ok(new_gene)
        }
        else{
            Result::Err("[Error][Gene][cross] gene1 != gene2")
        }
    }
}
impl fmt::Display for Gene{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut summised_string = String::from("");
        let temp_nucleotides= self.nucleotides.clone();
        let mut converted;
        for n in temp_nucleotides{
            
            match n{
                A => converted = 'A',
                G => converted = 'G',
                C => converted = 'C',
                T => converted = 'T'
            };
            summised_string.push(converted);
        }

        write!(f, "({})", summised_string)
    }
}
