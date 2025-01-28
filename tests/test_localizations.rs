mod test {

    #[cfg(feature = "typed")]
    use jscontact::{AddressComponentKind, Card, LocalizationObject};

    // We force the typed feature to be enabled for this test
    // If it's not enabled, the test will fail because some types are equivalent
    // example [`crate::Title`] and [`crate::Nickname`] have very similar fields
    #[cfg(feature = "typed")]
    #[test]
    fn test_localizations() {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "localizations": {
                "en": {
                    "titles": {
                        "@type": "Title", // we need to differentiate from nickname
                        "name": "Mr."
                    },
                    "name/full": "Okubo Masahito",
                    "nicknames/k391": {
                        "@type": "Nickname",
                        "name": "Johnny"
                    },
                    "addresses/k26": {
                        "components": [
                            { "kind": "region", "value": "東京都" },
                            { "kind": "locality", "value": "千代田区" },
                            { "kind": "district", "value": "丸ノ内" },
                            { "kind": "block", "value": "2-7" },
                            { "kind": "separator", "value": "-" },
                            { "kind": "number", "value": "2" },
                            { "kind": "postcode", "value": "〒100-8994" }
                        ],
                        "defaultSeparator": "",
                        "full": "〒100-8994東京都千代田区丸ノ内2-7-2",
                        "isOrdered": true
                    }
                }
            }
        });
        let card: Card = serde_json::from_value(json).unwrap();
        let localizations = card.localizations.unwrap();
        let patch_object = localizations.get("en").unwrap();
        let titles = match patch_object.get("titles") {
            Some(LocalizationObject::Title(titles)) => titles,
            e => panic!("{}", format!("Invalid type: {:?}", e)),
        };
        let name = titles.name.clone();
        assert_eq!(name, "Mr.");
        let nickname = match patch_object.get("nicknames/k391") {
            Some(LocalizationObject::Nickname(nicknames)) => nicknames,
            e => panic!("{}", format!("Invalid type: {:?}", e)),
        };
        let name = nickname.name.clone();
        assert_eq!(name, "Johnny");

        let full_name = match patch_object.get("name/full") {
            Some(LocalizationObject::String(name)) => name,
            e => panic!("{}", format!("Invalid type: {:?}", e)),
        };
        assert_eq!(full_name, "Okubo Masahito");
    }

    #[cfg(feature = "typed")]
    #[test]
    fn test_localizations_full_replacement_hashtable() {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "localizations": {
                "en": {
                    "addresses": {
                        "k25": {
                            "@type": "Address", // we need to differentiate
                            "components": [
                                { "kind": "number", "value": "46" },
                                { "kind": "name", "value": "1 Sukhumvit 51 Alley" },
                                { "kind": "subdistrict", "value": "Khlong Tan Nuea" },
                                { "kind": "district", "value": " Watthana" },
                                { "kind": "locality", "value": "Bangkok" },
                                { "kind": "country", "value": "Thailand" },
                                { "kind": "postcode", "value": "10110" }
                            ],
                            "defaultSeparator": ", ",
                            "isOrdered": true
                        }
                    }
                }
            }
        });
        let card: Card = serde_json::from_value(json).unwrap();
        let localizations = card.localizations.unwrap();
        let patch_object = localizations.get("en").unwrap();
        let address = match patch_object.get("addresses") {
            Some(LocalizationObject::AddressReplacement(address)) => address,
            e => panic!("{}", format!("Invalid type: {:?}", e)),
        };
        let one_address = address.get("k25").unwrap();
        let components = one_address.components.as_ref().unwrap();
        assert_eq!(components.len(), 7);
        assert_eq!(components[0].kind, AddressComponentKind::Number);
        assert_eq!(components[0].value, "46");
        assert_eq!(components[1].kind, AddressComponentKind::Name);
        assert_eq!(components[1].value, "1 Sukhumvit 51 Alley");
        assert_eq!(components[2].kind, AddressComponentKind::Subdistrict);
        assert_eq!(components[2].value, "Khlong Tan Nuea");
        assert_eq!(components[3].kind, AddressComponentKind::District);
        assert_eq!(components[3].value, " Watthana");
        assert_eq!(components[4].kind, AddressComponentKind::Locality);
        assert_eq!(components[4].value, "Bangkok");
        assert_eq!(components[5].kind, AddressComponentKind::Country);
        assert_eq!(components[5].value, "Thailand");
        assert_eq!(components[6].kind, AddressComponentKind::Postcode);
        assert_eq!(components[6].value, "10110");
    }
}
