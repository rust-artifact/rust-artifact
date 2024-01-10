use crate::schema::tokens;
use diesel::prelude::*;
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
pub static VALID_CHARACTERS: [char; 38] = [
    '.', '-', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

/// Parsing
pub fn parse(conn: &mut SqliteConnection, token_id: u64, token_flags: i32) -> Result<(), String> {
    // Validate Token ID Used
    validate_id(token_id).map_err(|e| e.to_string())?;

    // Generate Token Name ID
    let token_name = generate_token(token_id);

    // Validate the token name format
    validate_token(&token_name).map_err(|e| e.to_string())?;

    // TODO: Validate Owner
    // TODO: Validate other properties

    match token_exists(conn, &token_name) {
        Ok(exists) => {
            if exists {
                // If the token exists, update it
                update_token(conn, &token_name, &token_flags)
                    .map_err(|e| e.to_string())
            } else {
                // If the token does not exist, create it
                create_token(conn, &token_name, &token_flags)
                    .map_err(|e| e.to_string())
            }
        },
        Err(e) => Err(format!("Error checking token existence: {}", e)),
    }
}

/// Insert DB
fn create_token(conn: &mut SqliteConnection, token: &str, flags: &i32) -> Result<(), diesel::result::Error> {
    let new_token = NewToken { token, flags };

    diesel::insert_into(tokens::table)
        .values(&new_token)
        .execute(conn)
        .map(|_| ())
}

/// Update DB
fn update_token(conn: &mut SqliteConnection, token: &str, new_flags: &i32) -> Result<(), diesel::result::Error> {
    use crate::models::token::tokens::dsl::flags;
    use crate::models::token::tokens::dsl::tokens;

    diesel::update(tokens.find(token))
        .set(flags.eq(new_flags))
        .execute(conn)
        .map(|_| ())
}

/// Filter DB
fn fetch_token(conn: &mut SqliteConnection, token_name: &str) -> Result<Token, diesel::result::Error> {
    use crate::models::token::tokens::dsl::*;

    match tokens.filter(token.eq(token_name)).first::<Token>(conn) {
        Ok(matched_token) => Ok(matched_token),
        Err(diesel::result::Error::NotFound) => Err(diesel::result::Error::NotFound),
        Err(e) => Err(e),
    }
}

/// Filter DB
fn token_exists(conn: &mut SqliteConnection, token_name: &str) -> Result<bool, diesel::result::Error> {
    use crate::models::token::tokens::dsl::*;

    let exists = tokens
        .filter(token.eq(token_name))
        .first::<Token>(conn)
        .optional()?
        .is_some();

    Ok(exists)
}

/// Generation
fn generate_id(token: &str) -> u64 {
    // From Token to ID #
    let mut id: u64 = 0;

    for c in token.chars() {
        let n = VALID_CHARACTERS.iter().position(|&p| p == c).unwrap() as u64;

        id *= 38;
        id += n;
    }

    id
}

/// Translation
fn generate_token(id: u64) -> String {
    use num_integer::Integer;

    // From ID # to Token
    let mut n = id;
    let mut token = vec![];

    while n > 0 {
        let (q, r) = (n).div_rem(&38);
        let c = VALID_CHARACTERS[r as usize];

        token.push(c);
        n = q;
    }

    token.into_iter().rev().collect()
}

/// Validation
fn validate_id(id: u64) -> Result<(), ValidationError> {
    // Validate id range (AAA - 999999999999)
    if id < 2966 || id > 9065737908494995455 {
        Err(ValidationError::new(
            "InvalidTokenLength: Must be between 3 and 12",
        ))
    } else {
        Ok(())
    }
}

/// Validation
fn validate_token(token: &str) -> Result<(), ValidationError> {
    // Length between 3 and 12
    if token.len() < 3 || token.len() > 12 {
        Err(ValidationError::new(
            "InvalidTokenLength: Must be between 3 and 12",
        ))
    // Token minimum length 3
    } else if token.split('.').next().unwrap().len() < 3 {
        Err(ValidationError::new(
            "InvalidTokenLength: Minimum token length is 3",
        ))
    // Subtoken min. length 5
    } else if token.contains('.') && token.len() < 5 {
        Err(ValidationError::new(
            "InvalidTokenLength: Minimum subtoken length is 5",
        ))
    // Subtokens one level max
    } else if token.split('.').count() > 2 {
        Err(ValidationError::new(
            "InvalidTokenLength: Maximum subtoken level is 1",
        ))
    // First character NOT "."
    } else if token.chars().next().unwrap() == VALID_CHARACTERS[0] {
        Err(ValidationError::new(
            "InvalidTokenCharacters: First character cannot be '.'",
        ))
    // Final character NOT "."
    } else if token.chars().last().unwrap() == VALID_CHARACTERS[0] {
        Err(ValidationError::new(
            "InvalidTokenCharacters: Last character cannot be '.'",
        ))
    // Hyphens for ITNs ONLY
    } else if token.contains('-') && (!token.starts_with("XN--") || token.ends_with('-')) {
        return Err(ValidationError::new("InvalidTokenCharacters: Hyphens for IDN only."));
    // Not BTC or its subtoken
    } else if token == "BTC" || token.len() >= 4 && &token[..4] == "BTC." {
        Err(ValidationError::new(
            "InvalidTokenCharacters: Cannot issue BTC as a token.",
        ))
    // Not ART or its subtoken
    } else if token == "ART" || token.len() >= 4 && &token[..4] == "ART." {
        Err(ValidationError::new(
            "InvalidTokenCharacters: Cannot issue ART as a token.",
        ))
    // All characters UPPERCASE
    } else if !token
        .replace('.', "")
        .replace('-', "")
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    {
        Err(ValidationError::new(
            "InvalidTokenLetterCase: Must be uppercase characters.",
        ))
    // All characters ALPHA NUM
    } else if !token
        .replace('.', "")
        .replace('-', "")
        .chars()
        .all(|c| c.is_ascii_alphanumeric())
    {
        Err(ValidationError::new(
            "InvalidTokenCharacters: Must be ascii alphanumeric.",
        ))
    // All characters are valid
    } else if !token.chars().all(|c| VALID_CHARACTERS.contains(&c)) {
        Err(ValidationError::new(
            "InvalidTokenCharacters: Must only use valid characters.",
        ))
    // Valid
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::connection::Connection;
    use diesel::sqlite::SqliteConnection;
    use diesel_migrations::{run_pending_migrations, EmbeddedMigrations, MigrationHarness};

    static MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    fn establish_test_connection() -> SqliteConnection {
        let conn = SqliteConnection::establish(":memory:")
            .expect("Failed to create an in-memory database");

        // Run pending migrations
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");

        conn
    }

    #[test]
    fn test_create_and_update_token() {
        let mut conn = establish_test_connection();

        create_token(&mut conn, "UPDATE", &0);
        update_token(&mut conn, "UPDATE", &1);

        let updated_token = fetch_token(&conn, "UPDATE").unwrap();

        // Case: Updated Flags
        assert_eq!(updated_token.flags, 1);
    }

    #[test]
    fn test_token_exists() {
        let conn = establish_test_connection();

        create_token(&mut conn, "EXISTS", &0);

        // Case: Token Exists
        assert!(token_exists(&conn, "EXISTS").unwrap());
        // Case: Doesnt Exist
        assert!(!token_exists(&conn, "NOTEXISTS").unwrap());
    }

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
    fn validate_token_does_not_use_invalid_hyphens() {
        // Case: Single
        assert_eq!(validate_token("ABC-ABC").is_ok(), false);
        // Case: Double
        assert_eq!(validate_token("ABC--ABC").is_ok(), false);
        // Case: Start
        assert_eq!(validate_token("-ABC").is_ok(), false);
        // Case: Ending
        assert_eq!(validate_token("ABC-").is_ok(), false);
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

    #[test]
    fn validate_token_can_be_internationalized() {
        // Case: "艺术"
        assert_eq!(validate_token("艺术").is_ok(), false);
        // Case: "艺术" (ITN)
        assert_eq!(validate_token("XN--CQV902D").is_ok(), true);
        // Case: Loose Validation
        assert_eq!(validate_token("XN--1234567").is_ok(), false);
    }
}
