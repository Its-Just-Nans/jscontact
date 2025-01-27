mod test {

    use jscontact::Card;

    #[test]
    fn test_default_separator() {
        let json = include_bytes!("./imported/jsCard-addresses_defaultseparator_rfc6868.json");
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_invalid_member() {
        let json = include_bytes!("./imported/jsCard-InvalidTypeMemberValue.json");
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_multilingual() {
        let json = include_bytes!("./imported/jsCard-Multilingual.json");
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_rfc7483() {
        let json = include_bytes!("./imported/jsCard-RFC7483.json");
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_unstructured() {
        let json = include_bytes!("./imported/jsCard-Unstructured.json");
        let res: Result<Card, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_group() {
        let json = include_bytes!("./imported/jsCardGroup.json");
        let res: Result<Vec<Card>, _> = serde_json::from_slice(json);
        assert!(res.is_ok());
    }
}
