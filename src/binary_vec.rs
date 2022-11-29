use rand::random;

#[derive(Clone)]
pub struct BinaryVec {
    pub data: Box<Vec<bool>>,
}

impl BinaryVec {
    pub fn random(len: usize) -> BinaryVec {
        let mut data = Box::new(Vec::<bool>::new());

        for _ in 0..len {
            data.push(random());
        }

        BinaryVec { data }
    }

    pub fn one_flip(&self) -> Vec<BinaryVec> {
        let mut flips = Vec::new();

        for i in 0..self.data.len() {
            let mut copy = self.clone();
            let elem = copy.data.get_mut(i).unwrap();
            *elem = !*elem;
            flips.push(copy);
        }

        flips
    }
}
