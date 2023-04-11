// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefix_parts: Vec<&str> = prefix.split('/').collect();
    let request_parts: Vec<&str> = request_path.split('/').collect();

    if prefix_parts.len() > request_parts.len() {
        return false;
    }

    for (prefix_part, request_part) in prefix_parts.iter().zip(request_parts.iter()) {
        if prefix_part != request_part && prefix_part != &"*" {
            return false;
        }
    }
    true
}

mod tests {
    use super::*;

    #[test]
    fn test_matches_without_wildcard() {
        assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
        assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
        assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));
    
        assert!(!prefix_matches("/v1/publishers", "/v1"));
        assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
        assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
    }
    
    #[test]
    fn test_matches_with_wildcard() {
        assert!(prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/foo/books"
        ));
        assert!(prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/bar/books"
        ));
        assert!(prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/foo/books/book1"
        ));
    
        assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
        assert!(!prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/foo/booksByAuthor"
        ));
    }
}
