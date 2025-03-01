use regex::Regex;

pub fn extract_numbers(input: &str) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<u32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        let sample_text = "The Fibonacci sequence for numbers 5, 8, and 13 is requested.";
        let numbers = extract_numbers(sample_text);
        println!("{:?}", numbers);
        assert_eq!(numbers, vec![5, 8, 13]);
    }

    #[test]
    fn test_no_numbers() {
        let sample_text = "This text has no numbers.";
        let numbers = extract_numbers(sample_text);
        assert_eq!(numbers, Vec::<u32>::new());
    }

    #[test]
    fn test_mixed_text() {
        let sample_text = "There are 3 apples and 7 oranges in the basket.";
        let numbers = extract_numbers(sample_text);
        assert_eq!(numbers, vec![3, 7]);
    }
}
