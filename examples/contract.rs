// Demo Stylus contract showcasing the strings utility functions
// Note: This is a simplified example showing the interface.
// For a full Stylus contract, you would need additional dependencies and setup.

use alloy_primitives::U256;
use strings_utils::{to_string, to_hex_string, to_hex_string_fixed};

/// Demo contract that demonstrates string conversion functionality
pub struct StringsDemo;

impl StringsDemo {
    /// Convert a U256 value to its decimal string representation
    /// 
    /// # Arguments
    /// * `value` - The U256 value to convert
    /// 
    /// # Returns
    /// String containing the decimal representation
    pub fn value_to_decimal_string(value: U256) -> String {
        to_string(value)
    }
    
    /// Convert a U256 value to its hexadecimal string representation
    /// 
    /// # Arguments
    /// * `value` - The U256 value to convert
    /// 
    /// # Returns
    /// String containing the hexadecimal representation with "0x" prefix
    pub fn value_to_hex_string(value: U256) -> String {
        to_hex_string(value)
    }
    
    /// Convert a U256 value to a fixed-length hexadecimal string
    /// 
    /// # Arguments
    /// * `value` - The U256 value to convert
    /// * `length` - The desired number of hex characters (excluding "0x")
    /// 
    /// # Returns
    /// String containing the fixed-length hex representation with "0x" prefix
    pub fn value_to_hex_string_fixed(value: U256, length: usize) -> String {
        to_hex_string_fixed(value, length)
    }
    
    /// Generate a formatted token URI using both decimal and hex representations
    /// This mimics a common use case in NFT contracts
    /// 
    /// # Arguments
    /// * `token_id` - The token ID to generate URI for
    /// 
    /// # Returns
    /// Formatted URI string
    pub fn generate_token_uri(token_id: U256) -> String {
        let decimal_id = to_string(token_id);
        let hex_id = to_hex_string_fixed(token_id, 8);
        format!("https://api.example.com/token/{}/metadata?hex={}", decimal_id, hex_id)
    }
    
    /// Demonstrate converting various numeric representations
    /// Returns a formatted string showing different representations of the same value
    /// 
    /// # Arguments
    /// * `value` - The value to represent in multiple formats
    /// 
    /// # Returns
    /// Multi-line string showing decimal, hex, and fixed hex representations
    pub fn multi_format_display(value: U256) -> String {
        let decimal = to_string(value);
        let hex = to_hex_string(value);
        let hex_fixed_8 = to_hex_string_fixed(value, 8);
        let hex_fixed_16 = to_hex_string_fixed(value, 16);
        
        format!(
            "Value representations:\n\
             Decimal: {}\n\
             Hex: {}\n\
             Hex (8 chars): {}\n\
             Hex (16 chars): {}",
            decimal, hex, hex_fixed_8, hex_fixed_16
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;
    
    #[test]
    fn test_demo_contract_functions() {
        let test_value = U256::from(12345);
        
        // Test decimal conversion
        let decimal = StringsDemo::value_to_decimal_string(test_value);
        assert_eq!(decimal, "12345");
        
        // Test hex conversion
        let hex = StringsDemo::value_to_hex_string(test_value);
        assert_eq!(hex, "0x3039");
        
        // Test fixed hex conversion
        let hex_fixed = StringsDemo::value_to_hex_string_fixed(test_value, 8);
        assert_eq!(hex_fixed, "0x00003039");
        
        // Test token URI generation
        let uri = StringsDemo::generate_token_uri(U256::from(42));
        assert_eq!(uri, "https://api.example.com/token/42/metadata?hex=0x0000002a");
        
        // Test multi-format display
        let display = StringsDemo::multi_format_display(U256::from(255));
        assert!(display.contains("Decimal: 255"));
        assert!(display.contains("Hex: 0xff"));
        assert!(display.contains("Hex (8 chars): 0x000000ff"));
    }
    
    #[test]
    fn test_zero_value_handling() {
        let zero = U256::ZERO;
        
        assert_eq!(StringsDemo::value_to_decimal_string(zero), "0");
        assert_eq!(StringsDemo::value_to_hex_string(zero), "0x0");
        assert_eq!(StringsDemo::value_to_hex_string_fixed(zero, 4), "0x0000");
        
        let uri = StringsDemo::generate_token_uri(zero);
        assert_eq!(uri, "https://api.example.com/token/0/metadata?hex=0x00000000");
    }
    
    #[test]
    fn test_large_value_handling() {
        let large_value = U256::from(u64::MAX);
        
        let decimal = StringsDemo::value_to_decimal_string(large_value);
        assert_eq!(decimal, u64::MAX.to_string());
        
        let hex = StringsDemo::value_to_hex_string(large_value);
        assert_eq!(hex, "0xffffffffffffffff");
        
        let hex_fixed = StringsDemo::value_to_hex_string_fixed(large_value, 20);
        assert_eq!(hex_fixed, "0x0000ffffffffffffffff");
        
        // Test that the multi-format display works with large values
        let display = StringsDemo::multi_format_display(large_value);
        assert!(display.contains(&u64::MAX.to_string()));
        assert!(display.contains("0xffffffffffffffff"));
    }
}