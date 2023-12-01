use std::io::BufRead;

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 1", "Running...");
    info!(target: "Day 1", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 1", "Solving...");
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    for line in reader.lines() {
        let acv = AmendedCalibrationValue(line.with_context(|| "Problem reading line")?);
        total_part_1 += acv.calculate_calibration_value_part1();
        total_part_2 += acv.calculate_calibration_value_part2();
    }
    println!("Part 1: Calibration Value Total = {}", total_part_1);
    println!("Part 2: Calibration Value Total = {}", total_part_2);

    Ok(())
}

struct AmendedCalibrationValue(String);

impl AmendedCalibrationValue {
    fn calculate_calibration_value_part1(&self) -> u32 {
        let digits = self.extract_digits();
        let calibration_value_string = self.recover_calibration_value_string(digits);
        calibration_value_string.parse::<u32>().unwrap()
    }

    fn calculate_calibration_value_part2(&self) -> u32 {
        let digits = self.extract_digits_special();
        let calibration_value_string = self.recover_calibration_value_string(digits);
        calibration_value_string.parse::<u32>().unwrap()
    }

    fn recover_calibration_value_string(&self, digits: String) -> String {
        let length = digits.chars().count();

        if length == 1 {
            let mut single_digit_result = String::new();
            single_digit_result.push_str(&digits);
            single_digit_result.push_str(&digits);
            return single_digit_result;
        } else if length == 2 {
            return digits;
        } else if length > 2 {
            let first = digits.chars().nth(0).unwrap();
            let last = digits.chars().last().unwrap();
            let mut n_digit_result = String::new();
            n_digit_result.push(first);
            n_digit_result.push(last);
            return n_digit_result;
        }
        panic!("Unrecoverable Error: No digits found in Amended Calibration Value");
    }

    fn extract_digits(&self) -> String {
        let mut found_digits = String::new();
        for c in self.0.chars() {
            if let '0'..='9' = c {
                found_digits.push(c)
            };
        }
        found_digits
    }

    fn extract_digits_special(&self) -> String {
        let mut found_digits = String::new();
        let mut possible_word = String::new();
        for c in self.0.chars() {
            if let '0'..='9' = c {
                found_digits.push(c);
                possible_word.clear();
            } else {
                possible_word.push(c);
                let real_word = self.contains_digit_word(&possible_word);
                if !real_word.is_empty() {
                    found_digits.push_str(&real_word);
                    possible_word.clear();
                    possible_word.push(c);
                }
            }
        }
        found_digits
    }

    fn word_to_digit(&self, word: &str) -> String {
        let digit: String = match word {
            "one" => String::from("1"),
            "two" => String::from("2"),
            "three" => String::from("3"),
            "four" => String::from("4"),
            "five" => String::from("5"),
            "six" => String::from("6"),
            "seven" => String::from("7"),
            "eight" => String::from("8"),
            "nine" => String::from("9"),
            _ => String::from("")
        };
        digit
    }
    
    fn contains_digit_word(&self, word: &str) -> String {
        let digits: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for digit in digits {
            if word.contains(digit) {
                return self.word_to_digit(digit);
            }
        }
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /* cSpell:disable */
    #[test]
    fn can_extract_all_digits_from_line() {
        let case1 = AmendedCalibrationValue(String::from("1abc2"));
        let case2 = AmendedCalibrationValue(String::from("pqr3stu8vwx"));
        let case3 = AmendedCalibrationValue(String::from("a1b2c3d4e5f"));
        let case4 = AmendedCalibrationValue(String::from("treb7uchet"));
        assert_eq!(case1.extract_digits(), "12");
        assert_eq!(case2.extract_digits(), "38");
        assert_eq!(case3.extract_digits(), "12345");
        assert_eq!(case4.extract_digits(), "7");
    }

    #[test]
    fn can_find_first_and_last_digit_from_extracted_digits() {
        let acv = AmendedCalibrationValue(String::from(""));

        assert_eq!(acv.recover_calibration_value_string(String::from("12")), "12");
        assert_eq!(acv.recover_calibration_value_string(String::from("38")), "38");
        assert_eq!(acv.recover_calibration_value_string(String::from("12345")), "15");
        assert_eq!(acv.recover_calibration_value_string(String::from("7")), "77");   
    }

    #[test]
    fn can_calculate_original_calibration_value() {
        let case1 = AmendedCalibrationValue(String::from("1abc2"));
        let case2 = AmendedCalibrationValue(String::from("pqr3stu8vwx"));
        let case3 = AmendedCalibrationValue(String::from("a1b2c3d4e5f"));
        let case4 = AmendedCalibrationValue(String::from("treb7uchet"));
        assert_eq!(case1.calculate_calibration_value_part1(), 12);
        assert_eq!(case2.calculate_calibration_value_part1(), 38);
        assert_eq!(case3.calculate_calibration_value_part1(), 15);
        assert_eq!(case4.calculate_calibration_value_part1(), 77);
    }

    #[test]
    fn can_convert_word_to_digit() {
        let acv = AmendedCalibrationValue(String::from(""));
        assert_eq!(acv.word_to_digit("one"), String::from("1"));
        assert_eq!(acv.word_to_digit("two"), String::from("2"));
        assert_eq!(acv.word_to_digit("three"), String::from("3"));
        assert_eq!(acv.word_to_digit("four"), String::from("4"));
        assert_eq!(acv.word_to_digit("five"), String::from("5"));
        assert_eq!(acv.word_to_digit("six"), String::from("6"));
        assert_eq!(acv.word_to_digit("seven"), String::from("7"));
        assert_eq!(acv.word_to_digit("eight"), String::from("8"));
        assert_eq!(acv.word_to_digit("nine"), String::from("9"));
        assert_eq!(acv.word_to_digit("asdfasdfh"), String::from(""));
    }

    #[test]
    fn can_find_digits_written_as_words() {
        let case1 = AmendedCalibrationValue(String::from("two1nine"));
        let case2 = AmendedCalibrationValue(String::from("eightwothree"));
        let case3 = AmendedCalibrationValue(String::from("abcone2threexyz"));
        let case4 = AmendedCalibrationValue(String::from("xtwone3four"));
        let case5 = AmendedCalibrationValue(String::from("4nineeightseven2"));
        let case6 = AmendedCalibrationValue(String::from("zoneight234"));
        let case7 = AmendedCalibrationValue(String::from("7pqrstsixteen"));
        assert_eq!(case1.extract_digits_special(), "219");
        assert_eq!(case2.extract_digits_special(), "823");
        assert_eq!(case3.extract_digits_special(), "123");
        assert_eq!(case4.extract_digits_special(), "2134");
        assert_eq!(case5.extract_digits_special(), "49872");
        assert_eq!(case6.extract_digits_special(), "18234");
        assert_eq!(case7.extract_digits_special(), "76");
    }
}