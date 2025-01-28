//! We force the typed feature to be enabled for this test
//! If it's not enabled, the test will fail because some types are equivalent
//! example [`crate::Title`] and [`crate::Nickname`] have very similar fields
#![cfg(feature = "typed")]

mod test {

    use jscontact::{Card, LocalizationObject};

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
}
