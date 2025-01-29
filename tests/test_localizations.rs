mod test {

    use jscontact::{
        AddressComponentKind, CalendarKind, Card, DirectoryKind, LinkKind, MediaKind,
        NameComponentKind, PersonalInfoKind, PersonalInfoLevel, TitleKind,
    };

    #[test]
    fn test_localizations() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "addresses": {
                "k26": {
                    "components": [
                        { "kind": "region", "value": "reg" }
                    ]
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
        std::fs::write(
            "tests/localizations/test_localizations.json",
            serde_json::to_string_pretty(&json)?,
        )?;
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
        Ok(())
    }

    #[test]
    fn test_localizations_full_replacement_hashtable() -> Result<(), Box<dyn std::error::Error>> {
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
        std::fs::write(
            "tests/localizations/test_localizations_full_replacement_hashtable.json",
            serde_json::to_string_pretty(&json)?,
        )?;
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
        Ok(())
    }

    #[test]
    fn test_localizations_name() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "name": {
                "full": "Okubo Masahito Motohiro"
            },
            "localizations": {
                "en": {
                    "name": {
                        "full": "Okubo Masahito"
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_name.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json).unwrap();

        let localizations = card.get_localized("en")?;
        let name = localizations.name.unwrap();
        let fullname = name.full.unwrap();
        assert_eq!(fullname, "Okubo Masahito");
        Ok(())
    }

    #[test]
    fn test_localizations_name_path() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "name": {
                "full": "Okubo Masahito Motohiro"
            },
            "localizations": {
                "en": {
                    "name/full": "Okubo Masahito"
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_name_path.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
        let name = localizations.name.unwrap();
        let fullname = name.full.unwrap();
        assert_eq!(fullname, "Okubo Masahito");
        Ok(())
    }

    #[test]
    fn test_localizations_name_components() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "name": {
                "full": "Okubo Masahito Motohiro"
            },
            "localizations": {
                "en": {
                    "name": {
                        "components": [
                            { "kind": "given", "value": "Masahito" },
                            { "kind": "given2", "value": "Okubo" }
                        ],
                        "full": "Okubo Masahito"
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_name_components.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
        let name = localizations.name.unwrap();
        let components = name.components.as_ref().unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].kind, NameComponentKind::Given);
        assert_eq!(components[0].value, "Masahito");
        assert_eq!(components[1].kind, NameComponentKind::Given2);
        assert_eq!(components[1].value, "Okubo");
        let fullname = name.full.unwrap();
        assert_eq!(fullname, "Okubo Masahito");
        Ok(())
    }

    #[test]
    fn test_localizations_name_components_path_object_1() -> Result<(), Box<dyn std::error::Error>>
    {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "name": {
                "full": "Okubo Masahito Motohiro"
            },
            "localizations": {
                "en": {
                    "name/components": [
                        { "kind": "given", "value": "Masahito" },
                        { "kind": "given2", "value": "Okubo" }
                    ],
                    "name/full": "Okubo Masahito"
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_name_components_path_object_1.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
        let name = localizations.name.unwrap();
        let components = name.components.as_ref().unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].kind, NameComponentKind::Given);
        assert_eq!(components[0].value, "Masahito");
        assert_eq!(components[1].kind, NameComponentKind::Given2);
        assert_eq!(components[1].value, "Okubo");
        let fullname = name.full.unwrap();
        assert_eq!(fullname, "Okubo Masahito");
        Ok(())
    }

    #[test]
    fn test_localizations_titles() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "titles": {
                "t1": {
                    "name": "Monsieur",
                    "kind": "role"
                }
            },
            "localizations": {
                "en": {
                    "titles": {
                        "t1": {
                            "name": "Mr.",
                            "kind": "role"
                        }
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_titles.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
        let titles = localizations.titles.unwrap();
        let t1 = titles.get("t1").unwrap();
        assert_eq!(t1.name, "Mr.");
        assert_eq!(t1.kind, Some(TitleKind::Role));
        Ok(())
    }

    #[test]
    fn test_localizations_titles_path() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "titles": {
                "t1": {
                    "name": "Monsieur",
                    "kind": "role"
                }
            },
            "localizations": {
                "en": {
                    "titles/t1": {
                        "name": "Senor",
                        "kind": "role"
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_titles_path_object_1.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
        let titles = localizations.titles.unwrap();
        let t1 = titles.get("t1").unwrap();
        assert_eq!(t1.name, "Senor");
        assert_eq!(t1.kind, Some(TitleKind::Role));
        Ok(())
    }

    #[test]
    fn test_localizations_titles_path_object_2() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "titles": {
                "t1": {
                    "name": "Monsieur",
                    "kind": "role"
                }
            },
            "localizations": {
                "en": {
                    "titles/t1/name": "Mister",
                    "titles/t1/kind": "role"
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_titles_path_object_2.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
        let titles = localizations.titles.unwrap();
        let t1 = titles.get("t1").unwrap();
        assert_eq!(t1.name, "Mister");
        assert_eq!(t1.kind, Some(TitleKind::Role));
        Ok(())
    }

    #[test]
    fn test_localizations_nicknames() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "nicknames": {
                "k391": {
                    "name": "Johnny"
                }
            },
            "localizations": {
                "en": {
                    "nicknames": {
                        "k391": {
                            "name": "Steve"
                        }
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_nicknames.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
        let nicknames = localizations.nicknames.unwrap();
        let k391 = nicknames.get("k391").unwrap();
        assert_eq!(k391.name, "Steve");

        Ok(())
    }

    #[test]
    fn test_localizations_nicknames_path_object_1() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "nicknames": {
                "k391": {
                    "name": "Johnny"
                }
            },
            "localizations": {
                "en": {
                    "nicknames/k391": {
                        "name": "Steve"
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_nicknames_path_object_1.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
        let nicknames = localizations.nicknames.unwrap();
        let k391 = nicknames.get("k391").unwrap();
        assert_eq!(k391.name, "Steve");

        Ok(())
    }

    #[test]
    fn test_localizations_nicknames_path_object_2() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "nicknames": {
                "k391": {
                    "name": "Johnny"
                }
            },
            "localizations": {
                "en": {
                    "nicknames/k391/name": "Steve"
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_nicknames_path_object_2.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
        let nicknames = localizations.nicknames.unwrap();
        let k391 = nicknames.get("k391").unwrap();
        assert_eq!(k391.name, "Steve");

        Ok(())
    }

    #[test]
    fn test_localizations_addresses() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "addresses": {
                "k26": {
                    "components": [
                        { "kind": "region", "value": "reg" }
                    ]
                }
            },
            "localizations": {
                "en": {
                    "addresses": {
                        "k26": {
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
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_addresses.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
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

        let full = address.full.as_ref().unwrap();
        assert_eq!(full, "〒100-8994東京都千代田区丸ノ内2-7-2");
        Ok(())
    }

    #[test]
    fn test_localizations_addresses_path_object_1() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "addresses": {
                "k26": {
                    "components": [
                        { "kind": "region", "value": "reg" }
                    ]
                }
            },
            "localizations": {
                "en": {
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
        std::fs::write(
            "tests/localizations/test_localizations_addresses_path_object_1.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
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

        let full = address.full.as_ref().unwrap();
        assert_eq!(full, "〒100-8994東京都千代田区丸ノ内2-7-2");
        Ok(())
    }

    #[test]
    fn test_localizations_addresses_path_object_2() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "addresses": {
                "k26": {
                    "components": [
                        { "kind": "region", "value": "reg" }
                    ]
                }
            },
            "localizations": {
                "en": {
                    "addresses/k26/components": [
                        { "kind": "region", "value": "東京都" },
                        { "kind": "locality", "value": "千代田区" },
                        { "kind": "district", "value": "丸ノ内" },
                        { "kind": "block", "value": "2-7" },
                        { "kind": "separator", "value": "-" },
                        { "kind": "number", "value": "2" },
                        { "kind": "postcode", "value": "〒100-8994" }
                    ],
                    "addresses/k26/defaultSeparator": "",
                    "addresses/k26/full": "〒100-8994東京都千代田区丸ノ内2-7-2",
                    "addresses/k26/isOrdered": true
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_addresses_path_object_2.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
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

        let full = address.full.as_ref().unwrap();
        assert_eq!(full, "〒100-8994東京都千代田区丸ノ内2-7-2");
        Ok(())
    }

    /// The card in the test is not valid as the RFC states
    /// Because we cannot replace inexistent components
    /// But we still handle it
    #[test]
    fn test_localizations_addresses_path_object_3_invalid() -> Result<(), Box<dyn std::error::Error>>
    {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "addresses": {
                "k26": {
                    "components": [
                        { "kind": "region", "value": "reg" }
                    ]
                }
            },
            "localizations": {
                "en": {
                    "addresses/k26/components/0": {
                        "kind": "region",
                        "value": "東京都"
                    },
                    "addresses/k26/components/1": {
                        "kind": "locality",
                        "value": "千代田区"
                    },
                    "addresses/k26/components/2": {
                        "kind": "district",
                        "value": "丸ノ内"
                    },
                    "addresses/k26/components/3": {
                        "kind": "block",
                        "value": "2-7"
                    },
                    "addresses/k26/components/4": {
                        "kind": "separator",
                        "value": "-"
                    },
                    "addresses/k26/components/5": {
                        "kind": "number",
                        "value": "2"
                    },
                    "addresses/k26/components/6": {
                        "kind": "postcode",
                        "value": "〒100-8994"
                    },
                    "addresses/k26/defaultSeparator": "",
                    "addresses/k26/full": "〒100-8994東京都千代田区丸ノ内2-7-2",
                    "addresses/k26/isOrdered": true
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_addresses_path_object_3_invalid.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
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

        let full = address.full.as_ref().unwrap();
        assert_eq!(full, "〒100-8994東京都千代田区丸ノ内2-7-2");
        Ok(())
    }

    /// The card in the test is not valid as the RFC states
    /// Because we cannot replace inexistent components
    /// But we still handle it
    #[test]
    fn test_localizations_addresses_path_object_4_invalid() -> Result<(), Box<dyn std::error::Error>>
    {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "addresses": {
                "k26": {
                    "components": [
                        { "kind": "region", "value": "reg" }
                    ]
                }
            },
            "localizations": {
                "en": {
                    "addresses/k26/components/0/kind": "region",
                    "addresses/k26/components/0/value": "東京都",
                    "addresses/k26/components/1/kind": "locality",
                    "addresses/k26/components/1/value": "千代田区",
                    "addresses/k26/components/2/kind": "district",
                    "addresses/k26/components/2/value": "丸ノ内",
                    "addresses/k26/components/3/kind": "block",
                    "addresses/k26/components/3/value": "2-7",
                    "addresses/k26/components/4/kind": "separator",
                    "addresses/k26/components/4/value": "-",
                    "addresses/k26/components/5/kind": "number",
                    "addresses/k26/components/5/value": "2",
                    "addresses/k26/components/6/kind": "postcode",
                    "addresses/k26/components/6/value": "〒100-8994",
                    "addresses/k26/defaultSeparator": "",
                    "addresses/k26/full": "〒100-8994東京都千代田区丸ノ内2-7-2",
                    "addresses/k26/isOrdered": true
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_addresses_path_object_4_invalid.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json)?;

        let localizations = card.get_localized("en")?;
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

        let full = address.full.as_ref().unwrap();
        assert_eq!(full, "〒100-8994東京都千代田区丸ノ内2-7-2");
        Ok(())
    }

    #[test]
    fn test_localizations_personal_info() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "personalInfo": {
                "k26": {
                    "kind": "expertise",
                    "value": "ch",
                    "level": "high"
                },
                "pi1": {
                    "kind": "hobby",
                    "value": "lire",
                    "level": "high"
                },
                "pi6": {
                    "kind": "interest",
                    "value": "musique r&b",
                    "level": "medium"
                }
            },
            "localizations": {
                "en": {
                    "personalInfo": {
                        "k26": {
                            "kind": "expertise",
                            "value": "chemistry",
                            "level": "high"
                        },
                        "pi1": {
                            "kind": "hobby",
                            "value": "reading",
                            "level": "high"
                        },
                        "pi6": {
                            "kind": "interest",
                            "value": "r&b music",
                            "level": "medium"
                        }
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_personal_info.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json).unwrap();

        let localizations = card.get_localized("en")?;
        let personal_infos = localizations.personal_info.unwrap();
        let k26 = personal_infos.get("k26").unwrap();
        assert_eq!(&k26.value, "chemistry");
        assert_eq!(k26.level.as_ref().unwrap(), &PersonalInfoLevel::High);
        assert_eq!(k26.kind, PersonalInfoKind::Expertise);
        let pi1 = personal_infos.get("pi1").unwrap();
        assert_eq!(&pi1.value, "reading");
        assert_eq!(pi1.level.as_ref().unwrap(), &PersonalInfoLevel::High);
        assert_eq!(pi1.kind, PersonalInfoKind::Hobby);
        let pi6 = personal_infos.get("pi6").unwrap();
        assert_eq!(&pi6.value, "r&b music");
        assert_eq!(pi6.level.as_ref().unwrap(), &PersonalInfoLevel::Medium);
        assert_eq!(pi6.kind, PersonalInfoKind::Interest);

        Ok(())
    }

    #[test]
    fn test_localizations_personal_info_path_object_1() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "personalInfo": {
                "k26": {
                    "kind": "expertise",
                    "value": "ch",
                    "level": "high"
                },
                "pi1": {
                    "kind": "hobby",
                    "value": "lire",
                    "level": "high"
                },
                "pi6": {
                    "kind": "interest",
                    "value": "musique r&b",
                    "level": "medium"
                }
            },
            "localizations": {
                "en": {
                    "personalInfo/k26": {
                        "kind": "expertise",
                        "value": "chemistry",
                        "level": "high"
                    },
                    "personalInfo/pi1": {
                        "kind": "hobby",
                        "value": "reading",
                        "level": "high"
                    },
                    "personalInfo/pi6": {
                        "kind": "interest",
                        "value": "r&b music",
                        "level": "medium"
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_personal_info_path_object_1.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json).unwrap();

        let localizations = card.get_localized("en")?;
        let personal_infos = localizations.personal_info.unwrap();
        assert_eq!(personal_infos.len(), 3);
        let k26 = personal_infos.get("k26").unwrap();
        assert_eq!(&k26.value, "chemistry");
        assert_eq!(k26.level.as_ref().unwrap(), &PersonalInfoLevel::High);
        assert_eq!(k26.kind, PersonalInfoKind::Expertise);
        let pi1 = personal_infos.get("pi1").unwrap();
        assert_eq!(&pi1.value, "reading");
        assert_eq!(pi1.level.as_ref().unwrap(), &PersonalInfoLevel::High);
        assert_eq!(pi1.kind, PersonalInfoKind::Hobby);
        let pi6 = personal_infos.get("pi6").unwrap();
        assert_eq!(&pi6.value, "r&b music");
        assert_eq!(pi6.level.as_ref().unwrap(), &PersonalInfoLevel::Medium);
        assert_eq!(pi6.kind, PersonalInfoKind::Interest);

        Ok(())
    }

    #[test]
    fn test_localizations_personal_info_path_object_2() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "personalInfo": {
                "k26": {
                    "kind": "expertise",
                    "value": "ch",
                    "level": "high"
                },
                "pi1": {
                    "kind": "hobby",
                    "value": "lire",
                    "level": "high"
                },
                "pi6": {
                    "kind": "interest",
                    "value": "musique r&b",
                    "level": "medium"
                }
            },
            "localizations": {
                "en": {
                    "personalInfo/k26": {
                        "kind": "expertise",
                        "value": "chemistry",
                        "level": "high"
                    },
                    "personalInfo/pi1": {
                        "kind": "hobby",
                        "value": "reading",
                        "level": "high"
                    },
                    "personalInfo/pi6": {
                        "kind": "interest",
                        "value": "r&b music",
                        "level": "medium"
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_personal_info_path_object_2.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json).unwrap();

        let localizations = card.get_localized("en")?;
        let personal_infos = localizations.personal_info.unwrap();
        assert_eq!(personal_infos.len(), 3);
        let k26 = personal_infos.get("k26").unwrap();
        assert_eq!(&k26.value, "chemistry");
        assert_eq!(k26.level.as_ref().unwrap(), &PersonalInfoLevel::High);
        assert_eq!(k26.kind, PersonalInfoKind::Expertise);
        let pi1 = personal_infos.get("pi1").unwrap();
        assert_eq!(&pi1.value, "reading");
        assert_eq!(pi1.level.as_ref().unwrap(), &PersonalInfoLevel::High);
        assert_eq!(pi1.kind, PersonalInfoKind::Hobby);
        let pi6 = personal_infos.get("pi6").unwrap();
        assert_eq!(&pi6.value, "r&b music");
        assert_eq!(pi6.level.as_ref().unwrap(), &PersonalInfoLevel::Medium);
        assert_eq!(pi6.kind, PersonalInfoKind::Interest);

        Ok(())
    }

    #[test]
    fn test_localizations_personal_info_path_object_3() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "personalInfo": {
                "k26": {
                    "kind": "expertise",
                    "value": "ch",
                    "level": "high"
                },
                "pi1": {
                    "kind": "hobby",
                    "value": "lire",
                    "level": "high"
                },
                "pi6": {
                    "kind": "interest",
                    "value": "musique r&b",
                    "level": "medium"
                }
            },
            "localizations": {
                "en": {
                    "personalInfo/k26/kind": "expertise",
                    "personalInfo/k26/value": "chemistry",
                    "personalInfo/k26/level": "high",
                    "personalInfo/pi1/kind": "hobby",
                    "personalInfo/pi1/value": "reading",
                    "personalInfo/pi1/level": "high",
                    "personalInfo/pi6/kind": "interest",
                    "personalInfo/pi6/value": "r&b music",
                    "personalInfo/pi6/level": "medium"
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_personal_info_path_object_3.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json).unwrap();

        let localizations = card.get_localized("en")?;
        let personal_infos = localizations.personal_info.unwrap();
        assert_eq!(personal_infos.len(), 3);
        let k26 = personal_infos.get("k26").unwrap();
        assert_eq!(&k26.value, "chemistry");
        assert_eq!(k26.level.as_ref().unwrap(), &PersonalInfoLevel::High);
        assert_eq!(k26.kind, PersonalInfoKind::Expertise);
        let pi1 = personal_infos.get("pi1").unwrap();
        assert_eq!(&pi1.value, "reading");
        assert_eq!(pi1.level.as_ref().unwrap(), &PersonalInfoLevel::High);
        assert_eq!(pi1.kind, PersonalInfoKind::Hobby);
        let pi6 = personal_infos.get("pi6").unwrap();
        assert_eq!(&pi6.value, "r&b music");
        assert_eq!(pi6.level.as_ref().unwrap(), &PersonalInfoLevel::Medium);
        assert_eq!(pi6.kind, PersonalInfoKind::Interest);

        Ok(())
    }

    #[test]
    fn test_localizations_notes() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "notes": {
                "n1": {
                    "note": "This is a note."
                }
            },
            "localizations": {
                "en": {
                    "notes": {
                        "n1": {
                            "note": "This is a note in English."
                        }
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_notes.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json).unwrap();

        let localizations = card.get_localized("en")?;
        let notes = localizations.notes.unwrap();
        let n1 = notes.get("n1").unwrap();
        assert_eq!(n1.note, "This is a note in English.");

        Ok(())
    }

    #[test]
    fn test_localizations_notes_path_object_1() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "notes": {
                "n1": {
                    "note": "This is a note."
                }
            },
            "localizations": {
                "en": {
                    "notes/n1": {
                        "note": "This is a note in English."
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_notes_path_object_1.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json).unwrap();

        let localizations = card.get_localized("en")?;
        let notes = localizations.notes.unwrap();
        let n1 = notes.get("n1").unwrap();
        assert_eq!(n1.note, "This is a note in English.");

        Ok(())
    }

    #[test]
    fn test_localizations_notes_path_object_2() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "notes": {
                "n1": {
                    "note": "This is a note."
                }
            },
            "localizations": {
                "en": {
                    "notes/n1/note": "This is a note in English."
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_notes_path_object_2.json",
            serde_json::to_string_pretty(&json)?,
        )?;
        let card: Card = serde_json::from_value(json).unwrap();

        let localizations = card.get_localized("en")?;
        let notes = localizations.notes.unwrap();
        let n1 = notes.get("n1").unwrap();
        assert_eq!(n1.note, "This is a note in English.");

        Ok(())
    }

    #[test]
    fn test_localizations_keywords() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "keywords": {
                "un_mot": true
            },
            "localizations": {
                "en": {
                    "keywords": {
                        "a_keyword": true
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_keywords.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();

        let localizations = card.get_localized("en").unwrap();
        let keywords = localizations.keywords.unwrap();
        let k1 = keywords.get("a_keyword").unwrap();
        assert_eq!(k1, &true);
        Ok(())
    }

    #[test]
    fn test_localizations_media() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "media": {
                "res45": {
                    "kind": "sound",
                    "uri": "CID:JOHNQ.part8.19960229T080000.xyzMail@example.com"
                },
                "res47": {
                    "kind": "logo",
                    "uri": "https://www.example.com/pub/logos/abccorp.jpg"
                },
                "res1": {
                    "kind": "photo",
                    "uri": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."
                }
            },
            "localizations": {
                "en": {
                    "media": {
                        "res45": {
                            "kind": "sound",
                            "uri": "CID:"
                        },
                        "res47": {
                            "kind": "logo",
                            "uri": "https://www.example.com/pub/logos/abccorp.jpg"
                        },
                        "res1": {
                            "kind": "photo",
                            "uri": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."
                        }
                    }
                }
            }
        });

        std::fs::write(
            "tests/localizations/test_localizations_media.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let media = localized.media.unwrap();
        let res45 = media.get("res45").unwrap();
        assert_eq!(res45.kind, MediaKind::Sound);
        assert_eq!(res45.uri, "CID:");
        let res47 = media.get("res47").unwrap();
        assert_eq!(res47.kind, MediaKind::Logo);
        assert_eq!(res47.uri, "https://www.example.com/pub/logos/abccorp.jpg");
        let res1 = media.get("res1").unwrap();
        assert_eq!(res1.kind, MediaKind::Photo);
        assert_eq!(
            res1.uri,
            "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."
        );
        Ok(())
    }

    #[test]
    fn test_localizations_media_path_object_1() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "media": {
                "res45": {
                    "kind": "sound",
                    "uri": "CID:JOHNQ.part8.19960229T080000.xyzMail@example.com"
                },
                "res47": {
                    "kind": "logo",
                    "uri": "https://www.example.com/pub/logos/abccorp.jpg"
                },
                "res1": {
                    "kind": "photo",
                    "uri": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."
                }
            },
            "localizations": {
                "en": {
                    "media/res45": {
                        "kind": "sound",
                        "uri": "CID:"
                    },
                    "media/res47": {
                        "kind": "logo",
                        "uri": "https://www.example.com/pub/logos/abccorp.jpg"
                    },
                    "media/res1": {
                        "kind": "photo",
                        "uri": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."
                    }
                }
            }
        });

        std::fs::write(
            "tests/localizations/test_localizations_media_path_object_1.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let media = localized.media.unwrap();
        let res45 = media.get("res45").unwrap();
        assert_eq!(res45.kind, MediaKind::Sound);
        assert_eq!(res45.uri, "CID:");
        let res47 = media.get("res47").unwrap();
        assert_eq!(res47.kind, MediaKind::Logo);
        assert_eq!(res47.uri, "https://www.example.com/pub/logos/abccorp.jpg");
        let res1 = media.get("res1").unwrap();
        assert_eq!(res1.kind, MediaKind::Photo);
        assert_eq!(
            res1.uri,
            "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."
        );
        Ok(())
    }

    #[test]
    fn test_localizations_media_path_object_2() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "media": {
                "res45": {
                    "kind": "sound",
                    "uri": "CID:JOHNQ.part8.19960229T080000.xyzMail@example.com"
                },
                "res47": {
                    "kind": "logo",
                    "uri": "https://www.example.com/pub/logos/abccorp.jpg"
                },
                "res1": {
                    "kind": "photo",
                    "uri": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."
                }
            },
            "localizations": {
                "en": {
                    "media/res45/kind": "sound",
                    "media/res45/uri": "CID:",
                    "media/res47/kind": "logo",
                    "media/res47/uri": "https://www.example.com/pub/logos/abccorp.jpg",
                    "media/res1/kind": "photo",
                    "media/res1/uri": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."

                }
            }
        });

        std::fs::write(
            "tests/localizations/test_localizations_media_path_object_2.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let media = localized.media.unwrap();
        let res45 = media.get("res45").unwrap();
        assert_eq!(res45.kind, MediaKind::Sound);
        assert_eq!(res45.uri, "CID:");
        let res47 = media.get("res47").unwrap();
        assert_eq!(res47.kind, MediaKind::Logo);
        assert_eq!(res47.uri, "https://www.example.com/pub/logos/abccorp.jpg");
        let res1 = media.get("res1").unwrap();
        assert_eq!(res1.kind, MediaKind::Photo);
        assert_eq!(
            res1.uri,
            "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."
        );
        Ok(())
    }

    #[test]
    fn test_localizations_links() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "links": {
                "link3": {
                    "kind": "contact",
                    "uri": "mailto:contact@example.com",
                    "pref": 1
                }
            },
            "localizations": {
                "en": {
                    "links": {
                        "link3": {
                            "kind": "contact",
                            "uri": "mailto:",
                            "pref": 1
                        }
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_links.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let links = localized.links.unwrap();
        let link3 = links.get("link3").unwrap();
        assert_eq!(link3.kind, Some(LinkKind::Contact));
        assert_eq!(link3.uri, "mailto:");
        assert_eq!(link3.pref, Some(1));
        Ok(())
    }

    #[test]
    fn test_localizations_links_path_object_1() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "links": {
                "link3": {
                    "kind": "contact",
                    "uri": "mailto:contact@example.com",
                    "pref": 1
                }
            },
            "localizations": {
                "en": {
                    "links/link3": {
                        "kind": "contact",
                        "uri": "mailto:",
                        "pref": 1
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_links_path_object_1.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let links = localized.links.unwrap();
        let link3 = links.get("link3").unwrap();
        assert_eq!(link3.kind, Some(LinkKind::Contact));
        assert_eq!(link3.uri, "mailto:");
        assert_eq!(link3.pref, Some(1));
        Ok(())
    }

    #[test]
    fn test_localizations_links_path_object_2() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "links": {
                "link3": {
                    "kind": "contact",
                    "uri": "mailto:contact@example.com",
                    "pref": 1
                }
            },
            "localizations": {
                "en": {
                    "links/link3/kind": "contact",
                    "links/link3/uri": "mailto:",
                    "links/link3/pref": 1
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_links_path_object_2.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let links = localized.links.unwrap();
        let link3 = links.get("link3").unwrap();
        assert_eq!(link3.kind, Some(LinkKind::Contact));
        assert_eq!(link3.uri, "mailto:");
        assert_eq!(link3.pref, Some(1));
        Ok(())
    }

    #[test]
    fn test_localizations_directory() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "directories": {
                "dir1": {
                    "kind": "entry",
                    "uri": "https://dir.example.com/addrbook/jdoe/Jean%20Dupont.vcf",
                    "label": "http",
                    "listAs": 1,
                },
                "dir2": {
                    "kind": "directory",
                    "uri": "ldap://ldap.example/o=Example%20Tech,ou=Engineering",
                    "pref": 1,
                    "label": "LDAP Directory"
                }
            },
            "localizations": {
                "en": {
                    "directories": {
                        "dir1": {
                            "kind": "entry",
                            "uri": "https://dir.example.com/other",
                            "label": "http en",
                            "listAs": 1,
                        },
                        "dir2": {
                            "kind": "directory",
                            "uri": "ldap://ldap.example/other_lang",
                            "label": "ldap en",
                            "pref": 1
                        }
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_directory.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let directories = localized.directories.unwrap();
        let dir1 = directories.get("dir1").unwrap();
        assert_eq!(dir1.kind, Some(DirectoryKind::Entry));
        assert_eq!(dir1.uri, "https://dir.example.com/other");
        assert_eq!(dir1.label, Some("http en".to_string()));
        assert_eq!(dir1.list_as, Some(1));
        let dir2 = directories.get("dir2").unwrap();
        assert_eq!(dir2.kind, Some(DirectoryKind::Directory));
        assert_eq!(dir2.uri, "ldap://ldap.example/other_lang");
        assert_eq!(dir2.label, Some("ldap en".to_string()));
        assert_eq!(dir2.pref, Some(1));
        Ok(())
    }

    #[test]
    fn test_localizations_directory_path_object_1() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "directories": {
                "dir1": {
                    "kind": "entry",
                    "uri": "https://dir.example.com/addrbook/jdoe/Jean%20Dupont.vcf",
                    "label": "http",
                    "listAs": 1,
                },
                "dir2": {
                    "kind": "directory",
                    "uri": "ldap://ldap.example/o=Example%20Tech,ou=Engineering",
                    "pref": 1,
                    "label": "LDAP Directory"
                }
            },
            "localizations": {
                "en": {
                    "directories/dir1": {
                        "kind": "entry",
                        "uri": "https://dir.example.com/other",
                        "label": "http en",
                        "listAs": 1,
                    },
                    "directories/dir2": {
                        "kind": "directory",
                        "uri": "ldap://ldap.example/other_lang",
                        "label": "ldap en",
                        "pref": 1
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_directory_path_object_1.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let directories = localized.directories.unwrap();
        let dir1 = directories.get("dir1").unwrap();
        assert_eq!(dir1.kind, Some(DirectoryKind::Entry));
        assert_eq!(dir1.uri, "https://dir.example.com/other");
        assert_eq!(dir1.label, Some("http en".to_string()));
        assert_eq!(dir1.list_as, Some(1));
        let dir2 = directories.get("dir2").unwrap();
        assert_eq!(dir2.kind, Some(DirectoryKind::Directory));
        assert_eq!(dir2.uri, "ldap://ldap.example/other_lang");
        assert_eq!(dir2.label, Some("ldap en".to_string()));
        assert_eq!(dir2.pref, Some(1));
        Ok(())
    }

    #[test]
    fn test_localizations_directory_path_object_2() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "directories": {
                "dir1": {
                    "kind": "entry",
                    "uri": "https://dir.example.com/addrbook/jdoe/Jean%20Dupont.vcf",
                    "label": "http",
                    "listAs": 1,
                },
                "dir2": {
                    "kind": "directory",
                    "uri": "ldap://ldap.example/o=Example%20Tech,ou=Engineering",
                    "pref": 1,
                    "label": "LDAP Directory"
                }
            },
            "localizations": {
                "en": {
                    "directories/dir1/kind": "entry",
                    "directories/dir1/uri": "https://dir.example.com/other",
                    "directories/dir1/label": "http en",
                    "directories/dir1/listAs": 1,
                    "directories/dir2/kind": "directory",
                    "directories/dir2/uri": "ldap://ldap.example/other_lang",
                    "directories/dir2/label": "ldap en",
                    "directories/dir2/pref": 1
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_directory_path_object_2.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let directories = localized.directories.unwrap();
        let dir1 = directories.get("dir1").unwrap();
        assert_eq!(dir1.kind, Some(DirectoryKind::Entry));
        assert_eq!(dir1.uri, "https://dir.example.com/other");
        assert_eq!(dir1.label, Some("http en".to_string()));
        assert_eq!(dir1.list_as, Some(1));
        let dir2 = directories.get("dir2").unwrap();
        assert_eq!(dir2.kind, Some(DirectoryKind::Directory));
        assert_eq!(dir2.uri, "ldap://ldap.example/other_lang");
        assert_eq!(dir2.label, Some("ldap en".to_string()));
        assert_eq!(dir2.pref, Some(1));
        Ok(())
    }

    #[test]
    fn test_localizations_calendars() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "calendars": {
                "calA": {
                    "kind": "calendar",
                    "uri": "webcal://calendar.example.com/calA.ics",
                    "label": "Calendar A"
                },
                "project-a": {
                    "kind": "freeBusy",
                    "uri": "https://calendar.example.com/busy/project-a",
                    "pref": 1
                }
            },
            "localizations": {
                "en": {
                    "calendars": {
                        "calA": {
                            "kind": "calendar",
                            "uri": "webcal://calendar.example.com/en.ics",
                        },
                        "project-a": {
                            "kind": "freeBusy",
                            "uri": "https://busy.com",
                            "label": "Busy"
                        }
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_calendars.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let calendars = localized.calendars.unwrap();
        let cal_a = calendars.get("calA").unwrap();
        assert_eq!(cal_a.kind, Some(CalendarKind::Calendar));
        assert_eq!(cal_a.uri, "webcal://calendar.example.com/en.ics");
        assert_eq!(cal_a.label, None);
        let project_a = calendars.get("project-a").unwrap();
        assert_eq!(project_a.kind, Some(CalendarKind::FreeBusy));
        assert_eq!(project_a.uri, "https://busy.com");
        assert_eq!(project_a.label, Some("Busy".to_string()));
        Ok(())
    }

    #[test]
    fn test_localizations_calendars_path_object_1() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "calendars": {
                "calA": {
                    "kind": "calendar",
                    "uri": "webcal://calendar.example.com/calA.ics",
                    "label": "Calendar A"
                },
                "project-a": {
                    "kind": "freeBusy",
                    "uri": "https://calendar.example.com/busy/project-a",
                    "pref": 1
                }
            },
            "localizations": {
                "en": {
                    "calendars/calA": {
                        "kind": "calendar",
                        "uri": "webcal://calendar.example.com/en.ics",
                    },
                    "calendars/project-a": {
                        "kind": "freeBusy",
                        "uri": "https://busy.com",
                        "label": "Busy"
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_calendars_path_object_1.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let calendars = localized.calendars.unwrap();
        let cal_a = calendars.get("calA").unwrap();
        assert_eq!(cal_a.kind, Some(CalendarKind::Calendar));
        assert_eq!(cal_a.uri, "webcal://calendar.example.com/en.ics");
        assert_eq!(cal_a.label, None);
        let project_a = calendars.get("project-a").unwrap();
        assert_eq!(project_a.kind, Some(CalendarKind::FreeBusy));
        assert_eq!(project_a.uri, "https://busy.com");
        assert_eq!(project_a.label, Some("Busy".to_string()));
        Ok(())
    }

    #[test]
    fn test_localizations_calendars_path_object_2() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "calendars": {
                "calA": {
                    "kind": "calendar",
                    "uri": "webcal://calendar.example.com/calA.ics",
                    "label": "Calendar A"
                },
                "project-a": {
                    "kind": "freeBusy",
                    "uri": "https://calendar.example.com/busy/project-a",
                    "pref": 1
                }
            },
            "localizations": {
                "en": {
                    "calendars/calA/kind": "calendar",
                    "calendars/calA/uri": "webcal://calendar.example.com/en.ics",
                    "calendars/project-a/kind": "freeBusy",
                    "calendars/project-a/uri": "https://busy.com",
                    "calendars/project-a/label": "Busy"
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_calendars_path_object_2.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let calendars = localized.calendars.unwrap();
        let cal_a = calendars.get("calA").unwrap();
        assert_eq!(cal_a.kind, Some(CalendarKind::Calendar));
        assert_eq!(cal_a.uri, "webcal://calendar.example.com/en.ics");
        assert_eq!(cal_a.label, Some("Calendar A".to_string()));
        let project_a = calendars.get("project-a").unwrap();
        assert_eq!(project_a.kind, Some(CalendarKind::FreeBusy));
        assert_eq!(project_a.uri, "https://busy.com");
        assert_eq!(project_a.label, Some("Busy".to_string()));
        Ok(())
    }

    #[test]
    fn test_localizations_scheduling_addresses() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "schedulingAddresses": {
                "sched1": {
                    "uri": "mailto:janedoe@example.com",
                    "label": "Jane Doe"
                }
            },
            "localizations": {
                "en": {
                    "schedulingAddresses": {
                        "sched1": {
                            "uri": "mailto:",
                            "label": "Jane Doe english"
                        }
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_scheduling_addresses.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let scheduling_addresses = localized.scheduling_addresses.unwrap();
        let sched1 = scheduling_addresses.get("sched1").unwrap();
        assert_eq!(sched1.uri, "mailto:");
        assert_eq!(sched1.label, Some("Jane Doe english".to_string()));
        Ok(())
    }

    #[test]
    fn test_localizations_scheduling_addresses_path_object_1(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "schedulingAddresses": {
                "sched1": {
                    "uri": "mailto:janedoe@example.com",
                    "label": "Jane Doe"
                }
            },
            "localizations": {
                "en": {
                    "schedulingAddresses/sched1": {
                        "uri": "mailto:",
                        "label": "Jane Doe english"
                    }
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_scheduling_addresses_path_object_1.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let scheduling_addresses = localized.scheduling_addresses.unwrap();
        let sched1 = scheduling_addresses.get("sched1").unwrap();
        assert_eq!(sched1.uri, "mailto:");
        assert_eq!(sched1.label, Some("Jane Doe english".to_string()));
        Ok(())
    }

    #[test]
    fn test_localizations_scheduling_addresses_path_object_2(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "@type": "Card",
            "version": "1.0",
            "uid": "1234",
            "schedulingAddresses": {
                "sched1": {
                    "uri": "mailto:janedoe@example.com",
                    "label": "Jane Doe"
                }
            },
            "localizations": {
                "en": {
                    "schedulingAddresses/sched1/uri": "mailto:",
                    "schedulingAddresses/sched1/label": "Jane Doe english"
                }
            }
        });
        std::fs::write(
            "tests/localizations/test_localizations_scheduling_addresses_path_object_2.json",
            serde_json::to_string_pretty(&json).unwrap(),
        )?;
        let card: Card = serde_json::from_value(json).unwrap();
        let localized = card.get_localized("en").unwrap();
        let scheduling_addresses = localized.scheduling_addresses.unwrap();
        let sched1 = scheduling_addresses.get("sched1").unwrap();
        assert_eq!(sched1.uri, "mailto:");
        assert_eq!(sched1.label, Some("Jane Doe english".to_string()));
        Ok(())
    }
}
