use std::fs::read_to_string;

struct File {
    values: Vec<i64>,
    indices: Vec<usize>,
}

impl File {
    fn new(input: &str, key: i64) -> Self {
        let values: Vec<_> = input
            .lines()
            .map(|s| s.parse::<i64>().unwrap() * key)
            .collect();
        let indices = (0..values.len()).collect();
        File { values, indices }
    }

    fn mix(&mut self, count: u32) {
        for _ in 0..count {
            for (index, value) in self.values.clone().iter().enumerate() {
                let pos = self.indices.iter().position(|x| *x == index).unwrap();

                self.shift(pos, *value);
            }
        }

        self.values = self.indices.iter().map(|i| self.values[*i]).collect();
    }

    fn get_coords(&self) -> i64 {
        let pos = self.values.iter().position(|x| *x == 0).unwrap();
        let pos1 = (pos + 1000).rem_euclid(self.values.len());
        let pos2 = (pos + 2000).rem_euclid(self.values.len());
        let pos3 = (pos + 3000).rem_euclid(self.values.len());
        self.values[pos1] + self.values[pos2] + self.values[pos3]
    }

    fn shift(&mut self, pos: usize, shift_by: i64) {
        let new_pos = (pos as i64 + shift_by).rem_euclid(self.values.len() as i64 - 1) as usize;
        let value = self.indices.remove(pos);
        self.indices.insert(new_pos, value);
    }
}

const DECRYPTION_KEY: i64 = 811589153;
fn main() {
    let input = read_to_string("input").unwrap();
    let mut file_1 = File::new(&input, 1);
    file_1.mix(1);
    let output_1 = file_1.get_coords();

    let mut file_2 = File::new(&input, DECRYPTION_KEY);
    file_2.mix(10);
    let output_2 = file_2.get_coords();

    println!("part 1: {output_1} part 2: {output_2}");
}
