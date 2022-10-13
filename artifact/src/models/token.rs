use num_integer::Integer;

#[derive(Queryable)]
pub struct Token {
    pub token: String,
    pub flags: u32,
}

bitflags! {
    #[derive(Default)]
    struct Flags: u32 {
        const LOCKED = 0b00000001;
        const NAMESPACE = 0b00000010;
    }
}

// Valid Vec
pub static VALID_CHARACTERS: [char; 27] = [
    '.', 'A', 'B', 'C', 'D',
    'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X',
    'Y', 'Z',
];

// Error Enum
pub enum TokenValidationError {
    InvalidTokenCharacters,
    InvalidTokenLetterCase,
    InvalidTokenLength,
}

// Generation
pub fn generate_id(token: &str) -> u64 {
    // From Token to ID #
    let mut id: u64 = 0;

    for c in token.chars() {
        let n = VALID_CHARACTERS.iter()
            .position(|&p| p == c)
            .unwrap() as u64;

        id *= 27;
        id += n;
    }

    id
}

// Generation
pub fn generate_token(id: u64) -> String {
    // From ID # to Token
    let mut n = id;
    let mut token = vec![];

    while n > 0 {
        let (q, r) = (n).div_rem(&27);
        let c = VALID_CHARACTERS[r as usize];

        token.push(c);
        n = q;
    }
    
    token.into_iter().rev().collect()
}

// Validation
pub fn validate_token(token: &str) -> Result<&str, TokenValidationError> {
    // Length between 1 and 13
    if token.len() < 1 || token.len() > 13 {
        Err(TokenValidationError::InvalidTokenLength)
    // Subtokens one level max
    } else if token.split(".").count() > 2 {
        Err(TokenValidationError::InvalidTokenLength)
    // Not BTC or its subtoken
    } else if token == "BTC" || token.len() >= 4 && &token[..4] == "BTC." {
        Err(TokenValidationError::InvalidTokenCharacters)
    // Not ART or its subtoken
    } else if token == "ART" || token.len() >= 4 && &token[..4] == "ART." {
        Err(TokenValidationError::InvalidTokenCharacters)
    // All characters UPPERCASE
    } else if !token.replace(".", "").chars().all(|c| c.is_ascii_uppercase()) {
        Err(TokenValidationError::InvalidTokenLetterCase)
    // All characters ASCII ABC
    } else if !token.replace(".", "").chars().all(|c| c.is_ascii_alphabetic()) {
        Err(TokenValidationError::InvalidTokenCharacters)
    // First character NOT "."
    } else if token.chars().next().unwrap() == VALID_CHARACTERS[0] {
        Err(TokenValidationError::InvalidTokenCharacters)
    // Final character NOT "."
    } else if token.chars().last().unwrap() == VALID_CHARACTERS[0] {
        Err(TokenValidationError::InvalidTokenCharacters)
    // Valid
    } else {
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_token_is_not_btc_or_native_token() {
        // Case: Mainchain Token
        assert_eq!(validate_token("BTC").is_ok(), false);
        // Case: Federated Token
        assert_eq!(validate_token("ART").is_ok(), false);
        // Case: Mainchain Subtoken
        assert_eq!(validate_token("BTC.A").is_ok(), false);
        // Case: Federated Subtoken
        assert_eq!(validate_token("ART.A").is_ok(), false);
    }

    #[test]
    fn validate_token_is_not_multilevel_subtoken() {
        // Case: No-Levels
        assert_eq!(validate_token("ABC").is_ok(), true);
        // Case: One-Level
        assert_eq!(validate_token("ABC.ABC").is_ok(), true);
        // Case: Two-Levels
        assert_eq!(validate_token("ABC.ABC.ABC").is_ok(), false);
        // Case: Empty Level
        assert_eq!(validate_token("ABC..ABC").is_ok(), false);
        // Case: Empty Level
        assert_eq!(validate_token("ABC...ABC").is_ok(), false);
    }

    #[test]
    fn validate_token_is_between_1_and_13_chars() {
        // Case: Empty Name
        assert_eq!(validate_token("").is_ok(), false);
        // Case: 1 Character
        assert_eq!(validate_token("A").is_ok(), true);
        // Case: 13 Characters
        assert_eq!(validate_token("ZZZZZZZZZZZZZ").is_ok(), true);
        // Case: 13 Characters (Subtoken)
        assert_eq!(validate_token("ZZZZZZZZ.ZZZZ").is_ok(), true);
        // Case: 14 Characters
        assert_eq!(validate_token("XXXXXXXXXXXXXX").is_ok(), false);
        // Case: 14 Characters (Subtoken)
        assert_eq!(validate_token("XXXXXXXXX.XXXX").is_ok(), false);
    }

    #[test]
    fn validate_token_is_only_ascii_uppercase() {
        // Case: Lowercase
        assert_eq!(validate_token("lowercase").is_ok(), false);
        // Case: Lowercase (Subtoken)
        assert_eq!(validate_token("lower.case").is_ok(), false);
        // Case: Uppercase
        assert_eq!(validate_token("UPPERCASE").is_ok(), true);
        // Case: Uppercase (Subtoken)
        assert_eq!(validate_token("UPPER.CASE").is_ok(), true);
        // Case: Mixed case
        assert_eq!(validate_token("TitleCase").is_ok(), false);
        // Case: Mixed case (Subtoken)
        assert_eq!(validate_token("Title.Case").is_ok(), false);
    }

    #[test]
    fn validate_token_is_only_ascii_alphabetic() {
        // Case: Alphabetic
        assert_eq!(validate_token("ALPHABETIC").is_ok(), true);
        // Case: Alphabetic (Subtoken)
        assert_eq!(validate_token("ALPHA.BETIC").is_ok(), true);
        // Case: Japanese
        assert_eq!(validate_token("あ").is_ok(), false);
        // Case: Japanese (Subtoken)
        assert_eq!(validate_token("あ.あ").is_ok(), false);
        // Case: Numeric
        assert_eq!(validate_token("123456").is_ok(), false);
        // Case: Numeric (Subtoken)
        assert_eq!(validate_token("123.456").is_ok(), false);
        // Case: Mixed
        assert_eq!(validate_token("ABC123").is_ok(), false);
        // Case: Mixed (Subtoken)
        assert_eq!(validate_token("ABC.123").is_ok(), false);
    }

    #[test]
    fn validate_token_does_not_start_with_period() {
        // Case: Starting
        assert_eq!(validate_token(".ABC").is_ok(), false);
        // Case: Starting (Subtoken)
        assert_eq!(validate_token(".ABC.ABC").is_ok(), false);
    }

    #[test]
    fn validate_token_does_not_end_with_a_period() {
        // Case: Ending
        assert_eq!(validate_token("ABC.").is_ok(), false);
        // Case: Ending (Subtoken)
        assert_eq!(validate_token("ABC.ABC.").is_ok(), false);
    }
}
