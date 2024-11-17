// src/lib.rs
//! A library to check if a string can be parsed as a number
//! Similar to the is-number npm package

/// Checks if the input string represents a valid number
pub fn is_number(input: &str) -> bool {
    // Handle empty string
    if input.is_empty() {
        return false;
    }

    // Handle whitespace
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return false;
    }

    // Try parsing as different number types
    is_integer(trimmed) || is_float(trimmed)
}

/// Checks if the input string represents a valid integer
pub fn is_integer(input: &str) -> bool {
    // Handle signs
    let parsed = if input.starts_with('+') || input.starts_with('-') {
        &input[1..]
    } else {
        input
    };

    // Check if all remaining characters are digits
    !parsed.is_empty() && parsed.chars().all(|c| c.is_ascii_digit())
}

/// Checks if the input string represents a valid float
pub fn is_float(input: &str) -> bool {
    // Handle scientific notation
    if input.contains(['e', 'E']) {
        return parse_scientific_notation(input);
    }

    // Split by decimal point
    let parts: Vec<&str> = input.split('.').collect();
    match parts.len() {
        1 => is_integer(parts[0]),
        2 => {
            let integer_part = if parts[0].is_empty() { "0" } else { parts[0] };
            let decimal_part = parts[1];
            
            // Decimal part should only contain digits
            is_integer(integer_part) && 
            !decimal_part.is_empty() && 
            decimal_part.chars().all(|c| c.is_ascii_digit())
        }
        _ => false,
    }
}

/// Helper function to parse scientific notation
fn parse_scientific_notation(input: &str) -> bool {
    let parts: Vec<&str> = input.split(['e', 'E']).collect();
    if parts.len() != 2 {
        return false;
    }

    let base = parts[0];
    let exponent = parts[1];

    // Base can be integer or float
    if !(is_integer(base) || is_float(base)) {
        return false;
    }

    // Exponent must be an integer
    is_integer(exponent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integers() {
        assert!(is_number("123"));
        assert!(is_number("-123"));
        assert!(is_number("+123"));
        assert!(!is_number("12.3.4"));
        assert!(!is_number("abc"));
    }

    #[test]
    fn test_floats() {
        assert!(is_number("123.456"));
        assert!(is_number("-123.456"));
        assert!(is_number(".456"));
        assert!(is_number("-.456"));
        assert!(!is_number("123."));
        assert!(!is_number("."));
    }

    #[test]
    fn test_scientific() {
        assert!(is_number("1e5"));
        assert!(is_number("1E5"));
        assert!(is_number("1.23e5"));
        assert!(is_number("-1.23E-5"));
        assert!(!is_number("1e5.5"));
        assert!(!is_number("1e"));
    }

    #[test]
    fn test_edge_cases() {
        assert!(!is_number(""));
        assert!(!is_number(" "));
        assert!(!is_number("  "));
        assert!(!is_number("-"));
        assert!(!is_number("+"));
    }
}