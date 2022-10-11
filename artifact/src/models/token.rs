use num_integer::Integer;

// Token: Database Model
#[derive(Queryable)]
pub struct Token {
    pub name: String,
    pub flags: u32,
}

bitflags! {
    #[derive(Default)]
    struct Flags: u32 {
        const LOCKED = 0b00000001;
        const NAMESPACE = 0b00000010;
    }
}

// Valid Vec: ID <> Name
pub static VALID_CHARACTERS: [char; 27] = [
    '.', 'A', 'B', 'C', 'D',
    'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X',
    'Y', 'Z',
];

// Token Validation Enum
pub enum TokenValidationError {
    InvalidTokenCharacters,
    InvalidTokenLetterCase,
    InvalidTokenNameLength,
}

// Token ID # Generation
// - From String to ID #
pub fn generate_id(name: &str) -> u64 {
    let mut id: u64 = 0;

    for c in name.chars() {
        let n = VALID_CHARACTERS.iter()
            .position(|&p| p == c)
            .unwrap() as u64;

        id *= 27;
        id += n;
    }

    id
}

// Token Name Generation
// - From ID # to String
pub fn generate_name(id: u64) -> String {
    let mut n = id;
    let mut name = vec![];

    while n > 0 {
        let (q, r) = (n).div_rem(&27);
        let c = VALID_CHARACTERS[r as usize];

        name.push(c);
        n = q;
    }
    
    name.into_iter().rev().collect()
}

// Token Name Validation
// 1. Length between 1 and 13.
// 2. Subtokens one level max.
// 3. Not BTC or its subtoken.
// 4. Not ART or its subtoken.
// 5. All characters UPPERCASE.
// 6. All characters ASCII ABC.
// 7. First character NOT ".".
// 8. Final character NOT ".".
pub fn validate_name(name: &str) -> Result<&str, TokenValidationError> {
    if name.len() < 1 || name.len() > 13 {
        Err(TokenValidationError::InvalidTokenNameLength)
    } else if name.split(".").count() > 2 {
        Err(TokenValidationError::InvalidTokenNameLength)
    } else if name == "BTC" || name.len() >= 4 && &name[..4] == "BTC." {
        Err(TokenValidationError::InvalidTokenCharacters)
    } else if name == "ART" || name.len() >= 4 && &name[..4] == "ART." {
        Err(TokenValidationError::InvalidTokenCharacters)
    } else if !name.replace(".", "").chars().all(|c| c.is_ascii_uppercase()) {
        Err(TokenValidationError::InvalidTokenLetterCase)
    } else if !name.replace(".", "").chars().all(|c| c.is_ascii_alphabetic()) {
        Err(TokenValidationError::InvalidTokenCharacters)
    } else if name.chars().next().unwrap() == VALID_CHARACTERS[0] {
        Err(TokenValidationError::InvalidTokenCharacters)
    } else if name.chars().last().unwrap() == VALID_CHARACTERS[0] {
        Err(TokenValidationError::InvalidTokenCharacters)
    } else {
        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_token_name_is_only_between_1_and_13() {
        // Case: Empty Name
        assert_eq!(validate_name("").is_ok(), false);
        // Case: 1 Character
        assert_eq!(validate_name("A").is_ok(), true);
        // Case: 13 Characters
        assert_eq!(validate_name("ZZZZZZZZZZZZZ").is_ok(), true);
        // Case: 13 Characters (Subtoken)
        assert_eq!(validate_name("ZZZZZZZZ.ZZZZ").is_ok(), true);
        // Case: 14 Characters
        assert_eq!(validate_name("XXXXXXXXXXXXXX").is_ok(), false);
        // Case: 14 Characters (Subtoken)
        assert_eq!(validate_name("XXXXXXXXX.XXXX").is_ok(), false);
    }

    #[test]
    fn validate_token_name_is_not_multilevel_subtoken() {
        // Case: No-Levels
        assert_eq!(validate_name("ABC").is_ok(), true);
        // Case: One-Level
        assert_eq!(validate_name("ABC.ABC").is_ok(), true);
        // Case: Two-Levels
        assert_eq!(validate_name("ABC.ABC.ABC").is_ok(), false);
        // Case: Empty Level
        assert_eq!(validate_name("ABC..ABC").is_ok(), false);
        // Case: Empty Level
        assert_eq!(validate_name("ABC...ABC").is_ok(), false);
    }

    #[test]
    fn validate_token_name_is_not_btc_or_xcp() {
        // Case: Mainchain Token
        assert_eq!(validate_name("BTC").is_ok(), false);
        // Case: Federated Token
        assert_eq!(validate_name("ART").is_ok(), false);
    }

    #[test]
    fn validate_token_name_is_not_btc_or_xcp_subtoken() {
        // Case: Mainchain Subtoken
        assert_eq!(validate_name("BTC.A").is_ok(), false);
        // Case: Federated Subtoken
        assert_eq!(validate_name("ART.A").is_ok(), false);
    }

    #[test]
    fn validate_token_name_is_only_uppercase_letters() {
        // Case: Lowercase
        assert_eq!(validate_name("lowercase").is_ok(), false);
        // Case: Lowercase (Subtoken)
        assert_eq!(validate_name("lower.case").is_ok(), false);
        // Case: Uppercase
        assert_eq!(validate_name("UPPERCASE").is_ok(), true);
        // Case: Uppercase (Subtoken)
        assert_eq!(validate_name("UPPER.CASE").is_ok(), true);
        // Case: Mixed case
        assert_eq!(validate_name("TitleCase").is_ok(), false);
        // Case: Mixed case (Subtoken)
        assert_eq!(validate_name("Title.Case").is_ok(), false);
    }

    #[test]
    fn validate_token_name_is_only_ascii_alphabetic() {
        // Case: Alphabetic
        assert_eq!(validate_name("ALPHABETIC").is_ok(), true);
        // Case: Alphabetic (Subtoken)
        assert_eq!(validate_name("ALPHA.BETIC").is_ok(), true);
        // Case: Japanese
        assert_eq!(validate_name("あ").is_ok(), false);
        // Case: Japanese (Subtoken)
        assert_eq!(validate_name("あ.あ").is_ok(), false);
        // Case: Numeric
        assert_eq!(validate_name("123456").is_ok(), false);
        // Case: Numeric (Subtoken)
        assert_eq!(validate_name("123.456").is_ok(), false);
        // Case: Mixed
        assert_eq!(validate_name("ABC123").is_ok(), false);
        // Case: Mixed (Subtoken)
        assert_eq!(validate_name("ABC.123").is_ok(), false);
    }

    #[test]
    fn validate_token_name_does_not_start_with_period() {
        // Case: Starting
        assert_eq!(validate_name(".ABC").is_ok(), false);
        // Case: Starting (Subtoken)
        assert_eq!(validate_name(".ABC.ABC").is_ok(), false);
    }

    #[test]
    fn validate_token_name_does_not_end_with_a_period() {
        // Case: Ending
        assert_eq!(validate_name("ABC.").is_ok(), false);
        // Case: Ending (Subtoken)
        assert_eq!(validate_name("ABC.ABC.").is_ok(), false);
    }
}
