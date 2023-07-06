pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut prefix_index = 0;
    let mut request_index = 0;

    let prefix_bytes = prefix.as_bytes();
    let request_bytes = request_path.as_bytes();

    if request_bytes.len() < prefix_bytes.len() {
        return false;
    }

    if request_bytes.len() > prefix_bytes.len() && request_bytes[prefix_bytes.len()] as char != '/' {
        return false;
    }

    loop {
        let prefix_character = if prefix_index >= prefix_bytes.len() {
            '/'
        } else {
            prefix_bytes[prefix_index] as char
        };
        let request_character = request_bytes[request_index] as char;

        if prefix_character != request_character {
            return false;
        }

        prefix_index += 1;
        request_index += 1;

        if request_index >= request_bytes.len() {
            break;
        }

        if prefix_index >= prefix_bytes.len() {
            break;
        }
    }

    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));
}

#[test]
fn test_mismatches_without_wildcard() {
    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}