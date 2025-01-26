mod test {
    use std::collections::HashMap;

    use jscontact::{Card, Name, Nickname};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_figure_1() {
        let json = r#"{
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

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct CustomNameType {
            pub name: Option<Name>,
        }

        let decoded: CustomNameType = serde_json::from_str(json).unwrap();
        let name = decoded.name.unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].kind, "given");
        assert_eq!(components[0].value, "John");
        assert_eq!(components[0].phonetic, Some("/ˈdʒɑːn/".to_string()));
        assert_eq!(components[1].kind, "surname");
        assert_eq!(components[1].value, "Smith");
        assert_eq!(components[1].phonetic, Some("/smɪθ/".to_string()));
        assert_eq!(name.phonetic_system, Some("ipa".to_string()));
    }

    #[test]
    fn test_figure_17() {
        let json = r#"
        {
            "components": [
                { "kind": "given", "value": "Diego" },
                { "kind": "surname", "value": "Rivera" },
                { "kind": "surname2", "value": "Barrientos" }
            ],
            "isOrdered": true
        }"#;
        let name: Name = serde_json::from_str(json).unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 3);
        assert_eq!(components[0].kind, "given");
        assert_eq!(components[0].value, "Diego");
        assert_eq!(components[1].kind, "surname");
        assert_eq!(components[1].value, "Rivera");
        assert_eq!(components[2].kind, "surname2");
        assert_eq!(components[2].value, "Barrientos");
        assert_eq!(name.is_ordered, Some(true));
    }

    #[test]
    fn test_figure_19() {
        let json = r#"
        {
            "components": [
                { "kind": "given", "value": "Robert" },
                { "kind": "given2", "value": "Pau" },
                { "kind": "surname", "value": "Shou Chang" }
            ],
            "sortAs": {
                "surname": "Pau Shou Chang",
                "given": "Robert"
            },
            "isOrdered": true
        }"#;
        let name: Name = serde_json::from_str(json).unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 3);
        assert_eq!(components[0].kind, "given");
        assert_eq!(components[0].value, "Robert");
        assert_eq!(components[1].kind, "given2");
        assert_eq!(components[1].value, "Pau");
        assert_eq!(components[2].kind, "surname");
        assert_eq!(components[2].value, "Shou Chang");
        assert_eq!(name.is_ordered, Some(true));
        let sort_as = name.sort_as.unwrap();
        assert_eq!(sort_as.get("surname"), Some(&"Pau Shou Chang".to_string()));
        assert_eq!(sort_as.get("given"), Some(&"Robert".to_string()));
    }

    #[test]
    fn test_figure_20() {
        let json = r#"{
            "@type": "Card",
            "language": "zh-Hant",
            "name": {
                "components": [
                    { "kind": "surname", "value": "孫" },
                    { "kind": "given", "value": "中山" },
                    { "kind": "given2", "value": "文" },
                    { "kind": "given2", "value": "逸仙" }
                ]
            },
            "localizations": {
                "yue": {
                    "name/phoneticSystem": "jyut",
                    "name/phoneticScript": "Latn",
                    "name/components/0/phonetic": "syun1",
                    "name/components/1/phonetic": "zung1saan1",
                    "name/components/2/phonetic": "man4",
                    "name/components/3/phonetic": "jat6sin1"
                }
            }
        }"#;
        let card: Card = serde_json::from_str(json).unwrap();
        assert_eq!(card.language, Some("zh-Hant".to_string()));
        let name = card.name.unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 4);
        assert_eq!(components[0].kind, "surname");
        assert_eq!(components[0].value, "孫");
        assert_eq!(components[1].kind, "given");
        assert_eq!(components[1].value, "中山");
        assert_eq!(components[2].kind, "given2");
        assert_eq!(components[2].value, "文");
        assert_eq!(components[3].kind, "given2");
        assert_eq!(components[3].value, "逸仙");
        let localizations = card.localizations.unwrap();
        let yue = localizations.get("yue").unwrap();
        assert_eq!(yue.get("name/phoneticSystem"), Some(&"jyut".to_string()));
        assert_eq!(yue.get("name/phoneticScript"), Some(&"Latn".to_string()));
        assert_eq!(
            yue.get("name/components/0/phonetic"),
            Some(&"syun1".to_string())
        );
        assert_eq!(
            yue.get("name/components/1/phonetic"),
            Some(&"zung1saan1".to_string())
        );
        assert_eq!(
            yue.get("name/components/2/phonetic"),
            Some(&"man4".to_string())
        );
        assert_eq!(
            yue.get("name/components/3/phonetic"),
            Some(&"jat6sin1".to_string())
        );
    }

    #[test]
    fn test_figure_21() {
        let json = r#"{
            "nicknames": {
                "k391": {
                    "name": "Johnny"
                }
            }
        }"#;

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct CustomNicknameType {
            pub nicknames: Option<HashMap<String, Nickname>>,
        }

        let nicknames_decoded: CustomNicknameType = serde_json::from_str(json).unwrap();
        let nicknames = nicknames_decoded.nicknames.unwrap();
        let k391 = nicknames.get("k391").unwrap();
        assert_eq!(k391.name, "Johnny");
    }
}
