use argon2::{
    password_hash::{
        rand_core::OsRng, Error as PasswordHashError, PasswordHash, PasswordHasher,
        PasswordVerifier, SaltString,
    },
    Argon2,
};

use crate::errors::{hash_error::HashError, AppErrors};

pub fn hash_password(password: &str) -> Result<String, AppErrors> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppErrors::Hash(HashError::FailedHash))?;

    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppErrors> {
    let parsed_hash: PasswordHash<'_> =
        PasswordHash::new(hash).map_err(|e| AppErrors::Hash(HashError::FailedParse(e)))?;
    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(e) => Err(AppErrors::Hash(HashError::VerificationError(e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_success() {
        let password = "SecurePassword123!";
        let result = hash_password(password);

        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_hash_password_different_salts() {
        let password = "SamePassword";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        assert_ne!(
            hash1, hash2,
            "Different salts should produce different hashes"
        );
    }

    #[test]
    fn test_hash_password_special_chars_and_unicode() {
        let long_password = "a".repeat(1000);

        let passwords = vec![
            "",
            "!@#$%^&*()",
            "Українська мова 🔒",
            long_password.as_str(),
        ];

        for password in passwords {
            let result = hash_password(password);
            assert!(result.is_ok(), "Should handle: {}", password);
        }
    }

    #[test]
    fn test_verify_password_correct() {
        let password = "CorrectPassword123";
        let hash = hash_password(password).unwrap();

        let result = verify_password(password, &hash);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_verify_password_incorrect() {
        let password = "CorrectPassword";
        let hash = hash_password(password).unwrap();

        assert_eq!(verify_password("WrongPassword", &hash).unwrap(), false);
        assert_eq!(verify_password("correctpassword", &hash).unwrap(), false);
        assert_eq!(verify_password("", &hash).unwrap(), false);
    }

    #[test]
    fn test_verify_password_special_chars_and_unicode() {
        let passwords = vec!["Test!@#$%^&*()", "Тестовий 🔒", ""];

        for password in passwords {
            let hash = hash_password(password).unwrap();
            assert_eq!(
                verify_password(password, &hash).unwrap(),
                true,
                "Should verify: {}",
                password
            );
        }
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let result = verify_password("password", "not_a_valid_hash");

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AppErrors::Hash(HashError::FailedParse(_))
        ));
    }

    #[test]
    fn test_verify_password_corrupted_hash() {
        let password = "TestPassword";
        let hash = hash_password(password).unwrap();
        let mut corrupted = hash.clone();
        corrupted.replace_range(10..11, "X");

        let result = verify_password(password, &corrupted);

        assert!(
            result.is_err() || result.unwrap() == false,
            "Corrupted hash should fail"
        );
    }
}
