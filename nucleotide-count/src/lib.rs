use std::collections::HashMap;

static NUCLEOTIDES: &str = "ACGT";


pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = 0;

    if !NUCLEOTIDES.contains(nucleotide) {
        return Err(nucleotide)
    }

    for c in dna.chars() {
        if !NUCLEOTIDES.contains(c) {
            return Err(c)
        }

        if c == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::<char, usize>::new();
    for n in NUCLEOTIDES.chars() {
        result.insert(n, count(n, dna)?);
    }
    Ok(result)
}
