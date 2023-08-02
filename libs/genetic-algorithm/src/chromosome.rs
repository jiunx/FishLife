use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
    }
}

impl Chromosome {
    pub fn new(genes: Vec<f32>) -> Self {
        Self { genes }
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
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
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Chromosome;

    fn chromosome() -> Chromosome {
        Chromosome {
            genes: vec![0.0, 1.0, 2.0, 3.0],
        }
    }

    mod len {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let actual = chromosome.len();
            let expected = 4;

            assert_eq!(actual, expected);
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let actual = chromosome.iter().copied().collect::<Vec<_>>();
            let expected = vec![0.0, 1.0, 2.0, 3.0];

            assert_eq!(actual, expected);
        }
    }

    mod iter_mut {
        use super::*;

        #[test]
        fn test() {
            let mut chromosome = chromosome();
            chromosome.iter_mut().for_each(|x| *x += 1.0);
            let actual = chromosome.iter().copied().collect::<Vec<_>>();
            let expected = vec![1.0, 2.0, 3.0, 4.0];

            assert_eq!(actual, expected);
        }
    }

    mod index {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let actual = chromosome[2];
            let expected = 2.0;

            assert_eq!(actual, expected);
        }
    }

    mod from_iter {
        use super::*;

        #[test]
        fn test() {
            let chromosome = vec![0.0, 1.0, 2.0, 3.0].into_iter().collect::<Chromosome>();
            let actual = chromosome.iter().copied().collect::<Vec<_>>();
            let expected = vec![0.0, 1.0, 2.0, 3.0];

            assert_eq!(actual, expected);
        }
    }

    mod into_iter {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let actual = chromosome.into_iter().collect::<Vec<_>>();
            let expected = vec![0.0, 1.0, 2.0, 3.0];

            assert_eq!(actual, expected);
        }
    }
}
