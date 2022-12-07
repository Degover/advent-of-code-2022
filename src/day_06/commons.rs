use std::collections::HashMap;

pub struct MarkerFinder {
    curr: usize,
    marker_size: usize,
    data_stream: Vec<char>,
}

impl MarkerFinder {
    pub fn new(data_stream_input: String, size: usize) -> Self {
        Self {
            curr: size - 1,
            marker_size: size,
            data_stream: data_stream_input.chars().collect(),
        }
    }
}

impl Iterator for MarkerFinder {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.curr + 1..self.data_stream.len() {
            let chunk: Vec<char> = self.data_stream[i - self.marker_size..i]
                .try_into()
                .expect("Slice with incorrect length");

            let has_duplicated = chunk
                .iter()
                .fold(HashMap::new(), |mut hash: HashMap<&char, usize>, c| {
                    let count = hash.entry(c).or_insert(0);
                    *count += 1;
                    return hash;
                })
                .values()
                .any(|x| x > &1);

            if !has_duplicated {
                self.curr = i;
                return Some(self.curr);
            }
        }

        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn marker_finder_with_size_4_should_find_correctly() {
        let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let mut finder = MarkerFinder::new(example, 4);

        assert_eq!(finder.next(), Some(7));
        assert_eq!(finder.next(), Some(8));
    }

    #[test]
    fn marker_finder_with_size_14_should_find_correctly() {
        let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let mut finder = MarkerFinder::new(example, 14);

        assert_eq!(finder.next(), Some(19));
    }
}
