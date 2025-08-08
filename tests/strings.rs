use strings_utils::{to_string, to_hex_string, to_hex_string_fixed};
use alloy_primitives::U256;

#[test]
fn test_to_string_comprehensive() {
    // Test zero
    assert_eq!(to_string(U256::ZERO), "0");
    
    // Test single digits
    for i in 1..=9 {
        assert_eq!(to_string(U256::from(i)), i.to_string());
    }
    
    // Test powers of 10
    assert_eq!(to_string(U256::from(10)), "10");
    assert_eq!(to_string(U256::from(100)), "100");
    assert_eq!(to_string(U256::from(1000)), "1000");
    assert_eq!(to_string(U256::from(10000)), "10000");
    
    // Test arbitrary values
    assert_eq!(to_string(U256::from(12345)), "12345");
    assert_eq!(to_string(U256::from(987654321)), "987654321");
    assert_eq!(to_string(U256::from(u64::MAX)), u64::MAX.to_string());
    
    // Test large U256 values
    let large = U256::from(u128::MAX);
    assert_eq!(to_string(large), u128::MAX.to_string());
}

#[test]
fn test_to_hex_string_comprehensive() {
    // Test zero
    assert_eq!(to_hex_string(U256::ZERO), "0x0");
    
    // Test single hex digits
    assert_eq!(to_hex_string(U256::from(1)), "0x1");
    assert_eq!(to_hex_string(U256::from(10)), "0xa");
    assert_eq!(to_hex_string(U256::from(15)), "0xf");
    
    // Test powers of 16
    assert_eq!(to_hex_string(U256::from(16)), "0x10");
    assert_eq!(to_hex_string(U256::from(256)), "0x100");
    assert_eq!(to_hex_string(U256::from(4096)), "0x1000");
    
    // Test common hex values
    assert_eq!(to_hex_string(U256::from(0xff)), "0xff");
    assert_eq!(to_hex_string(U256::from(0x1234)), "0x1234");
    assert_eq!(to_hex_string(U256::from(0xabcdef)), "0xabcdef");
    assert_eq!(to_hex_string(U256::from(0xdeadbeef_u64)), "0xdeadbeef");
    
    // Test maximum values for different sizes
    assert_eq!(to_hex_string(U256::from(u8::MAX)), "0xff");
    assert_eq!(to_hex_string(U256::from(u16::MAX)), "0xffff");
    assert_eq!(to_hex_string(U256::from(u32::MAX)), "0xffffffff");
    assert_eq!(to_hex_string(U256::from(u64::MAX)), "0xffffffffffffffff");
}

#[test]
fn test_to_hex_string_fixed_comprehensive() {
    // Test zero with various lengths
    assert_eq!(to_hex_string_fixed(U256::ZERO, 1), "0x0");
    assert_eq!(to_hex_string_fixed(U256::ZERO, 2), "0x00");
    assert_eq!(to_hex_string_fixed(U256::ZERO, 4), "0x0000");
    assert_eq!(to_hex_string_fixed(U256::ZERO, 8), "0x00000000");
    assert_eq!(to_hex_string_fixed(U256::ZERO, 16), "0x0000000000000000");
    
    // Test padding with leading zeros
    assert_eq!(to_hex_string_fixed(U256::from(1), 1), "0x1");
    assert_eq!(to_hex_string_fixed(U256::from(1), 2), "0x01");
    assert_eq!(to_hex_string_fixed(U256::from(1), 4), "0x0001");
    assert_eq!(to_hex_string_fixed(U256::from(1), 8), "0x00000001");
    
    // Test values that fit exactly
    assert_eq!(to_hex_string_fixed(U256::from(0xff), 2), "0xff");
    assert_eq!(to_hex_string_fixed(U256::from(0x1234), 4), "0x1234");
    assert_eq!(to_hex_string_fixed(U256::from(0xabcdef), 6), "0xabcdef");
    
    // Test values that need more space than specified (no truncation)
    assert_eq!(to_hex_string_fixed(U256::from(0x12345), 4), "0x12345");
    assert_eq!(to_hex_string_fixed(U256::from(0xdeadbeef_u64), 4), "0xdeadbeef");
    assert_eq!(to_hex_string_fixed(U256::from(u64::MAX), 8), "0xffffffffffffffff");
    
    // Test with common contract address length (40 hex chars = 20 bytes)
    assert_eq!(
        to_hex_string_fixed(U256::from(0x1234567890abcdef_u64), 40),
        "0x00000000000000000000000000000000000000001234567890abcdef"
    );
}

#[test]
fn test_edge_cases() {
    // Test boundary values
    let one = U256::from(1);
    let max_u64 = U256::from(u64::MAX);
    let max_u128 = U256::from(u128::MAX);
    
    // Decimal conversions
    assert_eq!(to_string(one), "1");
    assert_eq!(to_string(max_u64), u64::MAX.to_string());
    assert_eq!(to_string(max_u128), u128::MAX.to_string());
    
    // Hex conversions
    assert_eq!(to_hex_string(one), "0x1");
    assert_eq!(to_hex_string(max_u64), "0xffffffffffffffff");
    
    // Fixed hex conversions with edge length
    assert_eq!(to_hex_string_fixed(one, 0), "0x1"); // Should still show the value even with 0 length
    assert_eq!(to_hex_string_fixed(max_u64, 16), "0xffffffffffffffff");
}

#[test]
fn test_consistency_between_functions() {
    let test_values = vec![
        U256::ZERO,
        U256::from(1),
        U256::from(15),
        U256::from(16),
        U256::from(255),
        U256::from(256),
        U256::from(0x1234),
        U256::from(0xdeadbeef_u64),
        U256::from(u64::MAX),
    ];
    
    for value in test_values {
        // Regular hex string should be a suffix of fixed-length with sufficient length
        let regular_hex = to_hex_string(value);
        let fixed_hex = to_hex_string_fixed(value, 64);
        
        assert!(fixed_hex.ends_with(&regular_hex[2..]), 
               "Fixed hex should end with regular hex digits for value {}", value);
        
        // Both should start with "0x"
        assert!(regular_hex.starts_with("0x"));
        assert!(fixed_hex.starts_with("0x"));
        
        // Decimal string should only contain digits
        let decimal = to_string(value);
        assert!(decimal.chars().all(|c| c.is_ascii_digit()));
        
        // For non-zero values, strings shouldn't be empty after prefix
        if !value.is_zero() {
            assert!(regular_hex.len() > 2);
            assert!(fixed_hex.len() > 2);
            assert!(!decimal.is_empty());
        }
    }
}

#[test]
fn test_large_u256_max() {
    let max_u256 = U256::MAX;
    
    // Test decimal conversion doesn't panic
    let decimal_str = to_string(max_u256);
    assert!(decimal_str.len() > 70); // U256::MAX has 78 decimal digits
    assert!(decimal_str.chars().all(|c| c.is_ascii_digit()));
    assert!(decimal_str.starts_with("1157")); // First few digits of 2^256 - 1
    
    // Test hex conversion
    let hex_str = to_hex_string(max_u256);
    assert_eq!(hex_str, "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    assert_eq!(hex_str.len(), 66); // "0x" + 64 hex digits
    
    // Test fixed hex with exact length
    let hex_fixed = to_hex_string_fixed(max_u256, 64);
    assert_eq!(hex_fixed, hex_str);
    
    // Test fixed hex with shorter length (should not truncate)
    let hex_short = to_hex_string_fixed(max_u256, 32);
    assert_eq!(hex_short, hex_str);
}