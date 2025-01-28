mod test {

    use jscontact::{
        AddressComponentKind, Card, NameComponentKind, PersonalInfoKind, PersonalInfoLevel,
        TitleKind,
    };

    #[test]
    fn test_localizations() {
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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations.json",
            serde_json::to_string_pretty(&card).unwrap(),
        )
        .unwrap();
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
        std::fs::write(
            "tests/localizations/test_localizations_full_replacement_hashtable.json",
            serde_json::to_string_pretty(&card).unwrap(),
        )
        .unwrap();
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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations_name.json",
            serde_json::to_string_pretty(&card).unwrap(),
        )?;
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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_name_path.json",
            serde_json::to_string_pretty(&card)?,
        )?;
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

        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_name_components.json",
            serde_json::to_string_pretty(&card)?,
        )?;
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

        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_name_components_path_object_1.json",
            serde_json::to_string_pretty(&card)?,
        )?;
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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_titles.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_titles_path_object_1.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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

        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_titles_path_object_2.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_nicknames.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_nicknames_path_object_1.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_nicknames_path_object_2.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_addresses.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_addresses_path_object_1.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_addresses_path_object_2.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
    fn test_localizations_addresses_path_object_3() -> Result<(), Box<dyn std::error::Error>> {
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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_addresses_path_object_3.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
    fn test_localizations_addresses_path_object_4() -> Result<(), Box<dyn std::error::Error>> {
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
        let card: Card = serde_json::from_value(json)?;
        std::fs::write(
            "tests/localizations/test_localizations_addresses_path_object_4.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations_personal_info.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations_personal_info_path_object_1.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations_personal_info_path_object_2.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations_personal_info_path_object_3.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations_notes.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations_notes_path_object_1.json",
            serde_json::to_string_pretty(&card)?,
        )?;

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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations_notes_path_object_2.json",
            serde_json::to_string_pretty(&card)?,
        )?;

        let localizations = card.get_localized("en")?;
        let notes = localizations.notes.unwrap();
        let n1 = notes.get("n1").unwrap();
        assert_eq!(n1.note, "This is a note in English.");

        Ok(())
    }

    #[test]
    fn test_localizations_keywords() {
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
        let card: Card = serde_json::from_value(json).unwrap();
        std::fs::write(
            "tests/localizations/test_localizations_keywords.json",
            serde_json::to_string_pretty(&card).unwrap(),
        )
        .unwrap();

        let localizations = card.get_localized("en").unwrap();
        let keywords = localizations.keywords.unwrap();
        let k1 = keywords.get("a_keyword").unwrap();
        assert_eq!(k1, &true);
    }
}
