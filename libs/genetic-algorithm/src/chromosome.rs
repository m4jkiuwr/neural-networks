use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Index<usize> for Chromosome {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}
impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}
impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;
    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

impl Chromosome {
    pub fn new(genes: Vec<f32>) -> Self {
        Self { genes }
    }

    pub fn test(size: usize) -> Self {
        let genes = (0..size).map(|n| n as f32).collect();
        Self { genes }
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}
