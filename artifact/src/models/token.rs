use diesel::prelude::*;
use crate::schema::tokens;
use validator::{Validate, ValidationError};

#[derive(Queryable, Validate)]
#[diesel(primary_key(token))]
pub struct Token {
    #[validate(custom = "validate_token")]
    pub token: String,
    #[validate(range(min = 0, max = 3))]
    pub flags: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tokens)]
pub struct NewToken<'a> {
    pub token: &'a str,
    pub flags: &'a i32,
}

bitflags! {
    #[derive(Default)]
    pub struct Flags: i32 {
        const LOCKED = 0b00000001;
        const NAMESPACE = 0b00000010;
    }
}

/// Valid Vec
pub static VALID_CHARACTERS: [char; 37] = [
    '.', 'A', 'B', 'C', 'D',
    'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '0', '1', '2',
    '3', '4', '5', '6', '7',
    '8', '9'
];

/// Save to DB
pub fn create_token(conn: &mut SqliteConnection, token: &str, flags: &i32) {
    let new_token = NewToken { token, flags };

    diesel::insert_into(tokens::table)
        .values(&new_token)
        .execute(conn)
        .expect("Error saving new token");
}

/// Save to DB
pub fn update_token(conn: &mut SqliteConnection, token: &str, new_flags: &i32) {
    use crate::models::token::tokens::dsl::tokens;
    use crate::models::token::tokens::dsl::flags;

    diesel::update(tokens.find(token))
        .set(flags.eq(new_flags))
        .execute(conn)
        .expect("Error updating token");
}

/// Generation
pub fn generate_id(token: &str) -> u64 {
    // From Token to ID #
    let mut id: u64 = 0;

    for c in token.chars() {
        let n = VALID_CHARACTERS.iter()
            .position(|&p| p == c)
            .unwrap() as u64;

        id *= 37;
        id += n;
    }

    id
}

/// Translation
pub fn generate_token(id: u64) -> String {
    use num_integer::Integer;

    // From ID # to Token
    let mut n = id;
    let mut token = vec![];

    while n > 0 {
        let (q, r) = (n).div_rem(&37);
        let c = VALID_CHARACTERS[r as usize];

        token.push(c);
        n = q;
    }
    
    token.into_iter().rev().collect()
}

/// Validation
pub fn validate_token(token: &str) -> Result<(), ValidationError> {
    // Length between 3 and 12
    if token.len() < 3 || token.len() > 12 {
        Err(ValidationError::new("InvalidTokenLength: Must be between 3 and 12"))
    // Token minimum length 3
    } else if token.split('.').next().unwrap().len() < 3 {
        Err(ValidationError::new("InvalidTokenLength: Minimum token length is 3"))
    // Subtoken min. length 5
    } else if token.contains('.') && token.len() < 5 {
        Err(ValidationError::new("InvalidTokenLength: Minimum subtoken length is 5"))
    // Subtokens one level max
    } else if token.split('.').count() > 2 {
        Err(ValidationError::new("InvalidTokenLength: Maximum subtoken level is 1"))
    // First character NOT "."
    } else if token.chars().next().unwrap() == VALID_CHARACTERS[0] {
        Err(ValidationError::new("InvalidTokenCharacters: First character cannot be '.'"))
    // Final character NOT "."
    } else if token.chars().last().unwrap() == VALID_CHARACTERS[0] {
        Err(ValidationError::new("InvalidTokenCharacters: Last character cannot be '.'"))
    // Not BTC or its subtoken
    } else if token == "BTC" || token.len() >= 4 && &token[..4] == "BTC." {
        Err(ValidationError::new("InvalidTokenCharacters: Cannot issue BTC as a token."))
    // Not ART or its subtoken
    } else if token == "ART" || token.len() >= 4 && &token[..4] == "ART." {
        Err(ValidationError::new("InvalidTokenCharacters: Cannot issue ART as a token."))
    // All characters UPPERCASE
    } else if !token.replace('.', "").chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
        Err(ValidationError::new("InvalidTokenLetterCase: Must be uppercase characters."))
    // All characters ALPHA NUM
    } else if !token.replace('.', "").chars().all(|c| c.is_ascii_alphanumeric()) {
        Err(ValidationError::new("InvalidTokenCharacters: Must be ascii alphanumeric."))
    // All characters are valid
    } else if !token.chars().all(|c| VALID_CHARACTERS.contains(&c)) {
        Err(ValidationError::new("InvalidTokenCharacters: Must only use valid characters."))
    // Valid
    } else {
        Ok(())
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

    #[test]
    fn validate_token_is_between_3_and_13_chars() {
        // Case: Empty Name
        assert_eq!(validate_token("").is_ok(), false);
        // Case: 1 Character
        assert_eq!(validate_token("A").is_ok(), false);
        // Case: 3 Characters
        assert_eq!(validate_token("AAA").is_ok(), true);
        // Case: 12 Characters
        assert_eq!(validate_token("ZZZZZZZZZZZZ").is_ok(), true);
        // Case: 13 Characters
        assert_eq!(validate_token("XXXXXXXXXXXXX").is_ok(), false);
    }

    #[test]
    fn validate_subtoken_is_between_5_and_13_chars() {
        // Case: Empty Name
        assert_eq!(validate_token("").is_ok(), false);
        // Case: 1 Character (Subtoken)
        assert_eq!(validate_token("A.A").is_ok(), false);
        // Case: 3 Characters (Subtoken)
        assert_eq!(validate_token("ABC.1").is_ok(), true);
        // Case: 12 Characters (Subtoken)
        assert_eq!(validate_token("ZZZZZZZZ.ZZZ").is_ok(), true);
        // Case: 13 Characters (Subtoken)
        assert_eq!(validate_token("XXXXXXXXX.XXX").is_ok(), false);
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
        assert_eq!(validate_token("UPPER.case").is_ok(), false);
        // Case: Uppercase (Subtoken)
        assert_eq!(validate_token("UPPER.CASE").is_ok(), true);
        // Case: Mixed case
        assert_eq!(validate_token("TitleCase").is_ok(), false);
        // Case: Mixed case (Subtoken)
        assert_eq!(validate_token("Title.case").is_ok(), false);
    }

    #[test]
    fn validate_token_is_only_ascii_alphanumeric() {
        // Case: Alphabetic
        assert_eq!(validate_token("ALPHABETIC").is_ok(), true);
        // Case: Alphabetic (Subtoken)
        assert_eq!(validate_token("ALPHA.BETIC").is_ok(), true);
        // Case: Japanese
        assert_eq!(validate_token("あ").is_ok(), false);
        // Case: Japanese (Subtoken)
        assert_eq!(validate_token("あ.あ").is_ok(), false);
        // Case: Numeric
        assert_eq!(validate_token("123456").is_ok(), true);
        // Case: Numeric (Subtoken)
        assert_eq!(validate_token("123.456").is_ok(), true);
        // Case: Mixed
        assert_eq!(validate_token("ABC123").is_ok(), true);
        // Case: Mixed (Subtoken)
        assert_eq!(validate_token("ABC.123").is_ok(), true);
    }
}
