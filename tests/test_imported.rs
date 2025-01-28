mod test {

    use jscontact::Card;

    #[test]
    fn test_default_separator() {
        let json = include_bytes!(
            "./imported/jscontact-tools/jsCard-addresses_defaultseparator_rfc6868.json"
        );
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_invalid_member() {
        let json = include_bytes!("./imported/jscontact-tools/jsCard-InvalidTypeMemberValue.json");
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_multilingual() {
        let json = include_bytes!("./imported/jscontact-tools/jsCard-Multilingual.json");
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_rfc7483() {
        let json = include_bytes!("./imported/jscontact-tools/jsCard-RFC7483.json");
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_unstructured() {
        let json = include_bytes!("./imported/jscontact-tools/jsCard-Unstructured.json");
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_group() {
        let json = include_bytes!("./imported/jscontact-tools/jsCardGroup.json");
        let res: Result<Vec<Card>, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    const EXCEPTIONS_SHOULD_OK: [&str; 5] = [
        "wrongCaseProp2_bad.json",
        "extra_nested_bad.json",
        "extra_bad.json",
        "wrongCaseProp_bad.json",
        "extra_patched_bad.json",
    ];

    #[test]
    fn test_imported_mapio() {
        let file = file!();
        let current_dir = std::path::Path::new(file).parent().unwrap();
        let tests = current_dir.join("imported/jscontact-tests");
        let tests = tests.read_dir().unwrap();
        for test in tests {
            let test = test.unwrap();
            let path = test.path();
            let json = std::fs::read(path).unwrap();
            let res: Result<Card, _> = serde_json::from_slice(&json);
            let path_str = test.file_name().to_str().unwrap().to_string();

            if path_str.contains("_bad") {
                if EXCEPTIONS_SHOULD_OK.contains(&path_str.as_str()) {
                    assert!(res.is_ok(), "Expected error for {}", path_str);
                } else {
                    assert!(res.is_err(), "Expected error for {}", path_str);
                }
            } else {
                assert!(res.is_ok(), "Expected success for {}", path_str);
            }
        }
    }
}
