/* Luhn Module
For defining the structured data of AccountNumber
 */
// SPDX-License-Identifier: Unlicense

use rand::prelude::*;
use std::fmt::Display;
use std::str::FromStr;

/// The account number uses random number generation
/// for the payload. This payload is then calculated
/// to produce the check digit.
#[derive(Debug)]
pub struct AccountNumber {
    /// Randomly generated number. Each digit is within 0..9
    pub(crate) payload: Vec<u8>,
    /// Check digit using the Luhn formula
    pub(crate) check_digit: u8,
}

/* For defining the default length of an account number */

impl Default for AccountNumber {
    fn default() -> Self {
        Self::new(10)
    }
}

/* For using the FromStr trait to transform into account numbers from strings

They are also checked using the verify function so that any invalid account number is rejected. */

impl FromStr for AccountNumber {
    type Err = std::io::ErrorKind;

    fn from_str(s: &str) -> Result<Self, std::io::ErrorKind> {
        if verify(s) {
            let mut payload: Vec<u8> = s
                .chars()
                .map(|d| {
                    d.to_digit(10)
                        .expect("Not able to create u32 from character") as u8
                })
                .collect();
            let check_digit = payload.pop().expect("Cannot pop. Got an empty vector");
            Ok(Self {
                payload,
                check_digit,
            })
        } else {
            eprintln!("{s} is not a valid account number");
            Err(std::io::ErrorKind::InvalidData)
        }
    }
}

/* For printing AccountNumber with Display trait */

impl Display for AccountNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut account_number: String = self.payload.iter().map(|d| d.to_string()).collect();
        account_number.push_str(&self.check_digit.to_string());
        write!(f, "{}", account_number)
    }
}

/* For implementing methods unique to AccountNumber */

impl AccountNumber {
    pub fn new(length: usize) -> Self {
        let mut payload: Vec<u8> = Vec::new();
        for _ in 1..=length - 1 {
            let mut rng = thread_rng();
            let zero_to_nine: u8 = rng.gen_range(0..=9);
            payload.push(zero_to_nine);
        }

        let check_digit = get_check_digit(&payload);

        Self {
            payload,
            check_digit,
        }
    }

    pub fn check_digit(&self) -> u8 {
        get_check_digit(&self.payload)
    }

    pub fn human_readable(&self) -> String {
        let mut payload: String = self.payload.iter().map(|d| d.to_string()).collect();
        payload.push_str(&self.check_digit.to_string());
        payload
    }
}
/* 
Helper functions for AccountNumber
Helper function to verify if string is valid AccountNumber

This is a useful helper function too since AccountNumber uses the FromStr trait.
 */
pub fn verify(account_number: &str) -> bool {
    let account_number = account_number.to_string();
    let digits: Vec<char> = account_number.trim().chars().collect();

    let mut payload: Vec<u8> = digits
        .iter()
        .map(|digit| digit.to_digit(10).expect("Not a number") as u8)
        .collect();

    let check_digit = payload.pop().expect("This shouldn't be an empty iterator");

    get_check_digit(&payload) == check_digit
}
/* 
Helper function to get the "Nonce" or check digit to validate the account number */

fn get_check_digit(payload: &[u8]) -> u8 {
    let mut new_payload = payload.iter().copied();
    let mut luhn_sum = 0;
    let mut index = 0;

    while let Some(item) = new_payload.next_back() {
        let divisible_by_2 = { (index as f32 % 2_f32) == 0_f32 };
        if divisible_by_2 {
            let mul = item * 2;

            let digits: Vec<u8> = mul
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).expect("Not a number character") as u8)
                .collect();

            let sum = {
                let mut x = 0;
                for digit in digits {
                    let y: u8 = digit;
                    x += y;
                }
                x
            };

            luhn_sum += sum;
        } else {
            luhn_sum += item;
        };
        index += 1;
    }

    10 - (luhn_sum % 10)
}
