//! # Strings Utility Library for Stylus
//! 
//! A Rust implementation of OpenZeppelin's `Strings.sol` library for Arbitrum Stylus.
//! Provides utility functions for converting U256 values to decimal and hexadecimal strings.

use alloy_primitives::U256;

/// Converts a U256 value to its ASCII decimal string representation.
/// 
/// This function mimics the behavior of OpenZeppelin's `toString(uint256)` function.
/// 
/// # Arguments
/// * `value` - The U256 value to convert
/// 
/// # Returns
/// A String containing the decimal representation of the value
/// 
/// # Examples
/// ```rust
/// use alloy_primitives::U256;
/// use strings_utils::to_string;
/// 
/// let result = to_string(U256::from(12345));
/// assert_eq!(result, "12345");
/// 
/// let zero = to_string(U256::ZERO);
/// assert_eq!(zero, "0");
/// ```
pub fn to_string(value: U256) -> String {
    if value.is_zero() {
        return "0".to_string();
    }
    
    let mut v = value;
    let mut digits = Vec::new();
    
    while !v.is_zero() {
        let digit = (v % U256::from(10)).to::<u64>() as u8;
        digits.push(b'0' + digit);
        v /= U256::from(10);
    }
    
    // Digits were pushed in reverse order, so reverse them
    digits.reverse();
    String::from_utf8(digits).expect("Invalid UTF-8 from digits")
}

/// Converts a U256 value to its hexadecimal string representation with "0x" prefix.
/// 
/// This function mimics the behavior of OpenZeppelin's `toHexString(uint256)` function.
/// The output length varies based on the value (no leading zeros except for zero value).
/// 
/// # Arguments
/// * `value` - The U256 value to convert
/// 
/// # Returns
/// A String containing the hexadecimal representation with "0x" prefix
/// 
/// # Examples
/// ```rust
/// use alloy_primitives::U256;
/// use strings_utils::to_hex_string;
/// 
/// let result = to_hex_string(U256::from(255));
/// assert_eq!(result, "0xff");
/// 
/// let zero = to_hex_string(U256::ZERO);
/// assert_eq!(zero, "0x0");
/// ```
pub fn to_hex_string(value: U256) -> String {
    if value.is_zero() {
        return "0x0".to_string();
    }
    
    let mut v = value;
    let mut hex_chars = Vec::new();
    
    while !v.is_zero() {
        let digit = (v % U256::from(16)).to::<u64>() as u8;
        let hex_char = if digit < 10 {
            b'0' + digit
        } else {
            b'a' + (digit - 10)
        };
        hex_chars.push(hex_char);
        v /= U256::from(16);
    }
    
    // Hex digits were pushed in reverse order, so reverse them
    hex_chars.reverse();
    let hex_string = String::from_utf8(hex_chars).expect("Invalid UTF-8 from hex digits");
    format!("0x{}", hex_string)
}

/// Converts a U256 value to a fixed-length hexadecimal string with "0x" prefix.
/// 
/// This function mimics the behavior of OpenZeppelin's `toHexString(uint256, uint256)` function.
/// If the value requires fewer hex characters than specified, leading zeros are added.
/// If the value requires more characters, the function will include all necessary characters.
/// 
/// # Arguments
/// * `value` - The U256 value to convert
/// * `length` - The desired number of hex characters (excluding "0x" prefix)
/// 
/// # Returns
/// A String containing the fixed-length hexadecimal representation with "0x" prefix
/// 
/// # Examples
/// ```rust
/// use alloy_primitives::U256;
/// use strings_utils::to_hex_string_fixed;
/// 
/// let result = to_hex_string_fixed(U256::from(255), 4);
/// assert_eq!(result, "0x00ff");
/// 
/// let zero = to_hex_string_fixed(U256::ZERO, 8);
/// assert_eq!(zero, "0x00000000");
/// ```
pub fn to_hex_string_fixed(value: U256, length: usize) -> String {
    if value.is_zero() {
        return format!("0x{}", "0".repeat(length));
    }
    
    let mut v = value;
    let mut hex_chars = Vec::new();
    
    while !v.is_zero() {
        let digit = (v % U256::from(16)).to::<u64>() as u8;
        let hex_char = if digit < 10 {
            b'0' + digit
        } else {
            b'a' + (digit - 10)
        };
        hex_chars.push(hex_char);
        v /= U256::from(16);
    }
    
    // Hex digits were pushed in reverse order, so reverse them
    hex_chars.reverse();
    let hex_string = String::from_utf8(hex_chars).expect("Invalid UTF-8 from hex digits");
    
    // Pad with leading zeros if necessary, or keep full length if longer
    let padded = if hex_string.len() < length {
        format!("{:0>width$}", hex_string, width = length)
    } else {
        hex_string
    };
    
    format!("0x{}", padded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;
    
    #[test]
    fn test_to_string_zero() {
        assert_eq!(to_string(U256::ZERO), "0");
    }
    
    #[test]
    fn test_to_string_small_values() {
        assert_eq!(to_string(U256::from(1)), "1");
        assert_eq!(to_string(U256::from(9)), "9");
        assert_eq!(to_string(U256::from(10)), "10");
        assert_eq!(to_string(U256::from(99)), "99");
        assert_eq!(to_string(U256::from(100)), "100");
    }
    
    #[test]
    fn test_to_string_large_values() {
        assert_eq!(to_string(U256::from(12345)), "12345");
        assert_eq!(to_string(U256::from(1000000)), "1000000");
        assert_eq!(to_string(U256::from(u64::MAX)), u64::MAX.to_string());
    }
    
    #[test]
    fn test_to_hex_string_zero() {
        assert_eq!(to_hex_string(U256::ZERO), "0x0");
    }
    
    #[test]
    fn test_to_hex_string_small_values() {
        assert_eq!(to_hex_string(U256::from(1)), "0x1");
        assert_eq!(to_hex_string(U256::from(15)), "0xf");
        assert_eq!(to_hex_string(U256::from(16)), "0x10");
        assert_eq!(to_hex_string(U256::from(255)), "0xff");
        assert_eq!(to_hex_string(U256::from(256)), "0x100");
    }
    
    #[test]
    fn test_to_hex_string_large_values() {
        assert_eq!(to_hex_string(U256::from(0xdeadbeef_u64)), "0xdeadbeef");
        assert_eq!(to_hex_string(U256::from(u64::MAX)), "0xffffffffffffffff");
    }
    
    #[test]
    fn test_to_hex_string_fixed_zero() {
        assert_eq!(to_hex_string_fixed(U256::ZERO, 1), "0x0");
        assert_eq!(to_hex_string_fixed(U256::ZERO, 4), "0x0000");
        assert_eq!(to_hex_string_fixed(U256::ZERO, 8), "0x00000000");
    }
    
    #[test]
    fn test_to_hex_string_fixed_padding() {
        assert_eq!(to_hex_string_fixed(U256::from(1), 4), "0x0001");
        assert_eq!(to_hex_string_fixed(U256::from(255), 4), "0x00ff");
        assert_eq!(to_hex_string_fixed(U256::from(0xabc), 8), "0x00000abc");
    }
    
    #[test]
    fn test_to_hex_string_fixed_no_truncation() {
        // When value needs more digits than specified length, don't truncate
        assert_eq!(to_hex_string_fixed(U256::from(0x12345), 2), "0x12345");
        assert_eq!(to_hex_string_fixed(U256::from(0xdeadbeef_u64), 4), "0xdeadbeef");
    }
    
    #[test]
    fn test_max_u256_values() {
        let max_u256 = U256::MAX;
        
        // Test that max value converts without panicking
        let decimal_str = to_string(max_u256);
        assert!(decimal_str.len() > 70); // U256::MAX has 78 decimal digits
        assert!(decimal_str.chars().all(|c| c.is_ascii_digit()));
        
        let hex_str = to_hex_string(max_u256);
        assert_eq!(hex_str, "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        
        let hex_fixed = to_hex_string_fixed(max_u256, 64);
        assert_eq!(hex_fixed, "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    }
}