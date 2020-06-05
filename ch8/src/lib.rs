use std::collections::HashMap;

pub fn mean_median_mode(input: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(3);
    let sum: u64 = input.iter().sum();
    let mean: u64 = sum / input.len() as u64;
    let median: u64 = 0;
    let num_values = input
        .iter()
        .fold(HashMap::new(), |mut acc, el| {
            let entry = acc.entry(el).or_insert(0);
            *entry += 1;
            acc
        });
    let (mode, _): (u64, _) = num_values
        .iter()
        .fold((0, 0), |(num, greatest_val), (key, val)| {
            if val > &greatest_val {
                (**key, *val)
            } else {
                (num, greatest_val)
            }
        });

    result.push(mean);
    result.push(median);
    result.push(mode);
    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn mmm() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = mean_median_mode(&input);
        assert_eq!(result.len(), 3)
    }
}

