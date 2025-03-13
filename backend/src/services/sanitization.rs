//! Service to do input/output sanitization on data that is consumed by the frontend, ie to remove script tags possibly containing malicious Javascript by a user

use ammonia::Builder;

/// Remove from a String HTML code, such as scripts
///
/// # Arguments
/// * `src` - string to sanitize
///
/// # Returns
/// A sanitized string where HTML elements have been removed
///
pub fn clean_all_html(src: &str) -> String {
    Builder::default().clean(src).to_string()
}
