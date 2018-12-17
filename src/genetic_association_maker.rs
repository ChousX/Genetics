
use nucleotide::Nucleotide;
//TODO
/*
    make a a data structor that holds funtions for each level of then of each nucleotide
    save file 
    read file
    random assiner

*/
//genetic association maker
// n^4
/*
    fn foo(number: usize){
        ...
        foo(number-1);
        foo(number-1);
        foo(number-1);
        foo(number-1);
    }
*/
struct GAM{

}

impl GAM{

    pub fn genetic_combinati
    ons_maker(&mut self, num: usize) { 
        let mut combonations: Vec<Vec<Nucleotide>> = Vec::new();
        for a in 0..num{
            for b in 0..num{
                for c in 0..num{
                    for d in 0..num{
                        let mut temp: Vec<Nucleotide> = Vec::new();
                        temp.push(GAM::gcm_helper(a%4));
                        temp.push(GAM::gcm_helper(b%4));
                        temp.push(GAM::gcm_helper(c%4));
                        temp.push(GAM::gcm_helper(d%4));
                        combonations.push(temp);
                    }
                }
            }
        }
    }
    fn gcm_helper(subceject: usize) -> Nucleotide{
        let x = match subceject{
            0 =>{ Nucleotide::A},
            1 =>{ Nucleotide::G},
            2 =>{ Nucleotide::C},
            3 =>{ Nucleotide::T},
            _ =>{ Nucleotide::A}
        };
        x
    }
}