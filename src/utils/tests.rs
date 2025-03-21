#[cfg(test)]
mod tests {
    use crate::utils::auth::{hash_password, verify_password};
    
    #[test]
    fn test_password_hashing_and_verification() {
        let password = "test_password123";
        
        // Hash the password
        let hash = hash_password(password).expect("Failed to hash password");
        
        // Verify the password against the hash
        let is_valid = verify_password(password, &hash).expect("Failed to verify password");
        
        assert!(is_valid, "Password verification should succeed with correct password");
        
        // Verify with wrong password
        let wrong_password = "wrong_password";
        let is_invalid = verify_password(wrong_password, &hash).expect("Failed to verify password");
        
        assert!(!is_invalid, "Password verification should fail with wrong password");
    }
}
