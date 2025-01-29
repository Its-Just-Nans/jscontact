mod test {
    use jscontact::Card;
    use serde_json::Value;

    #[test]
    fn test_usage_1() {
        let json = r#"
        {
            "@type": "Card",
            "version": "1.0",
            "uid": "22B2C7DF-9120-4969-8460-05956FE6B065",
            "name": {
            "components": [{
                    "kind": "given",
                    "value": "John",
                    "phonetic": "/ˈdʒɑːn/"
                }, {
                    "kind": "surname",
                    "value": "Smith",
                    "phonetic": "/smɪθ/"
            }],
            "phoneticSystem": "ipa"
            }
        }"#;
        let card: Card = serde_json::from_str(json).unwrap();

        let card_2 = json.parse().unwrap();

        assert_eq!(card, card_2);
    }

    #[test]
    fn test_usage_2() {
        let json = r#"
        {
            "@type": "Card",
            "version": "1.0",
            "uid": "22B2C7DF-9120-4969-8460-05956FE6B065",
            "name": {
            "components": [{
                    "kind": "given",
                    "value": "John",
                    "phonetic": "/ˈdʒɑːn/"
                }, {
                    "kind": "surname",
                    "value": "Smith",
                    "phonetic": "/smɪθ/"
            }],
            "phoneticSystem": "ipa"
            }
        }"#;
        let card_value: Value = serde_json::from_str(json).unwrap();
        let card: Card = serde_json::from_value(card_value.clone()).unwrap();

        let card_into: Card = card_value.try_into().unwrap();
        assert_eq!(card, card_into);
    }

    #[test]
    fn test_localized() {
        use jscontact::{Card, CardVersion, Name};
        use std::collections::HashMap;

        let mut card = Card::new(CardVersion::OneDotZero, "my:uri");
        let mut name = Name::default();
        name.full = Some("John".to_string());
        card.name = Some(name);

        let mut translations: HashMap<String, Value> = HashMap::new();
        let mut name_en = Name::default();
        name_en.full = Some("Johny".to_string());
        translations.insert(
            "name".to_string(),
            serde_json::to_value(name_en).expect("Failed to serialize name"),
        );
        card.add_localization("en", translations);

        let langs = card.get_available_languages();
        assert_eq!(langs, vec!["en"]);
        let localized = card.get_localized(&langs[0]).unwrap();
        assert_eq!(localized.name.unwrap().full.unwrap(), "Johny");
    }
}
