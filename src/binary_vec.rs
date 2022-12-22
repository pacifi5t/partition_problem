use rand::random;

#[derive(Clone)]
pub struct BinaryVec {
    bits: Vec<bool>,
}

impl BinaryVec {
    pub fn random(len: usize) -> Self {
        let mut bits = Vec::<bool>::new();

        for _ in 0..len {
            bits.push(random());
        }

        Self { bits }
    }

    pub fn ones(len: usize) -> Self {
        Self {
            bits: vec![true; len],
        }
    }

    pub fn one_flip(&self) -> Vec<BinaryVec> {
        let mut flips = Vec::new();

        for i in 0..self.bits.len() {
            let mut copy = self.clone();
            let elem = copy.bits.get_mut(i).unwrap();
            *elem = !*elem;
            flips.push(copy);
        }

        flips
    }

    pub fn two_flip(&self) -> Vec<BinaryVec> {
        let mut flips = Vec::new();

        for i in 0..self.bits.len() {
            for j in 0..self.bits.len() {
                if i == j {
                    continue;
                }

                let mut copy = self.clone();

                let elem1 = copy.bits.get_mut(i).unwrap();
                *elem1 = !*elem1;

                let elem2 = copy.bits.get_mut(j).unwrap();
                *elem2 = !*elem2;

                flips.push(copy);
            }
        }

        flips
    }

    pub fn as_nums(&self) -> Vec<u8> {
        let mut nums = Vec::new();
        for each in self.bits.iter() {
            nums.push(match each {
                true => 1,
                false => 0,
            })
        }

        nums
    }
}
