use enums::Mutation;
use chromosome::Chromosome;
use rand::prelude::*;


pub struct Genome{
    chromosomes: Vec<Chromosome>,
    size: usize
}
impl Genome{
    pub fn new()->Genome{
        Genome{
            chromosomes: Vec::new(),
            size: 0
        }
    }
        pub fn push(&mut self, new_chromosome: Chromosome){
        self.chromosomes.push(new_chromosome);
        self.size += 1;
    }
    pub fn pop(&mut self){
        self.chromosomes.pop();
        self.size -= 1;
    }
    pub fn insert_chromosome(&mut self, chromosome_number: usize, new_chromosome: Chromosome){
        self.chromosomes.insert(chromosome_number, new_chromosome);
        self.size += 1;
    }
    pub fn remove_chromosome(&mut self, chromosome_number: usize){
        self.chromosomes.remove(chromosome_number);
        self.size -= 1;
    }
    pub fn size(&self) -> usize{
        self.size
    }
    pub fn get_chromosome(&self, chromosome_number: usize) -> Result<Chromosome, &'static str>{
        if chromosome_number > self.size {
            Result::Err("[Error][Genome][get_chromosome]
             chromosome_number is larger than the size of the genome")
        }
        else{
            Result::Ok(self.chromosomes[chromosome_number].clone())
        }
    }
    pub fn get_ref_chromosome(&self, chromosome_number: usize) -> Result<&Chromosome, &'static str>{
        if chromosome_number > self.size {
            Result::Err("[Error][Genome][get_ref_chromosome]
             chromosome_number is larger than the size of the genome")
        }
        else{
            Result::Ok(&self.chromosomes[chromosome_number])
        }
    }
    pub fn find_chromosome_by_name(&self, name: String)->Result<&Chromosome, &'static str>{
        for x in &self.chromosomes{
            if x.get_name() == name{
                return Result::Ok(&x);
            }
        }
        Result::Err("[Warning][Genome][find_chromosome_by_name]Did not find Chromosome name")
    }
    pub fn set_chromosome(&mut self, chromosome_number: usize, new_chromosome: Chromosome){
        if chromosome_number > self.size{}
        else{
            self.chromosomes[chromosome_number] = new_chromosome;
        }
    }
    pub fn print(&self){
        let temp = self.chromosomes.clone();
        
        for x in temp{
            println!("Chromosome: {}", x.get_name());
            x.print();
            println!("");
        }
        println!("");
    }
}
impl Genome{
    pub fn cross(rng: &mut ThreadRng, genome1: &Genome, genome2: &Genome)->Result<Genome, &'static str>{
        let mut new_genome = Genome::new();  
        let size = if(genome1.size() >= genome2.size()){
            genome1.size()
        }else{
            genome2.size()
        };
            for i in 0..size{
                if i > genome1.size() && i > genome2.size(){
                    new_genome.push(Chromosome::cross(rng, &genome1.get_chromosome(i)?, &genome2.get_chromosome(i)?)?);
                }else{
                    if genome1.size() > genome2.size(){
                        new_genome.push(genome2.get_chromosome(i)?);
                    }else{
                        new_genome.push(genome1.get_chromosome(i)?);
                    }
                }
            }
        
    Result::Ok(new_genome)
    }
    pub fn mutate(&mut self, rng: &mut ThreadRng, chans: usize, mutation: &Mutation){
        for x in &mut self.chromosomes{
            x.mutate(rng, chans, mutation);
        }
    }
}