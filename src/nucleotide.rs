use std::fmt;

#[derive(Copy,Clone)]
pub enum Nucleotide{
    A,G,C,T
}
impl fmt::Display for Nucleotide{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = match *self {
            Nucleotide::A => 'A',
            Nucleotide::G => 'G',
            Nucleotide::C => 'C',
            Nucleotide::T => 'T'
        };
        write!(f, "({})", n)
    }
}
