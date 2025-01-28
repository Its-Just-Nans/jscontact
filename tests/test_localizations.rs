mod test {

    #[cfg(feature = "typed")]
    use jscontact::{AddressComponentKind, Card};

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
            "addresses": {
                "k26": {
                    "components": []
                }
            },
            "localizations": {
                "en": {
                    "titles/t1": {
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
        let localizations = card.get_localized("en").unwrap();
        let titles = localizations.titles.unwrap();
        let t1 = titles.get("t1").unwrap();
        assert_eq!(t1.name, "Mr.");
        let name = localizations.name.unwrap();
        let fullname = name.full.unwrap();
        assert_eq!(fullname, "Okubo Masahito");
        assert_eq!(
            localizations.nicknames.unwrap().get("k391").unwrap().name,
            "Johnny"
        );
        let addresses = localizations.addresses.unwrap();
        let address = addresses.get("k26").unwrap();
        let components = address.components.as_ref().unwrap();
        assert_eq!(components.len(), 7);
        assert_eq!(components[0].kind, AddressComponentKind::Region);
        assert_eq!(components[0].value, "東京都");
        assert_eq!(components[1].kind, AddressComponentKind::Locality);
        assert_eq!(components[1].value, "千代田区");
        assert_eq!(components[2].kind, AddressComponentKind::District);
        assert_eq!(components[2].value, "丸ノ内");
        assert_eq!(components[3].kind, AddressComponentKind::Block);
        assert_eq!(components[3].value, "2-7");
        assert_eq!(components[4].kind, AddressComponentKind::Separator);
        assert_eq!(components[4].value, "-");
        assert_eq!(components[5].kind, AddressComponentKind::Number);
        assert_eq!(components[5].value, "2");
        assert_eq!(components[6].kind, AddressComponentKind::Postcode);
        assert_eq!(components[6].value, "〒100-8994");
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
        let localizations = card.get_localized("en").unwrap();
        let addresses = localizations.addresses.unwrap();
        let address = addresses.get("k25").unwrap();
        let components = address.components.as_ref().unwrap();
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
