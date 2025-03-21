// Export utility modules
pub mod auth;
#[cfg(test)]
mod tests;

// Re-export common utility functions
pub use auth::{hash_password, verify_password, generate_token, validate_token};
