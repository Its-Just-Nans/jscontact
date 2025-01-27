mod test {

    use jscontact::{
        AddressComponentKind, CalendarKind, Card, CardKind, DateObject, DirectoryKind, LinkKind,
        LocalizationObject, MediaKind, NameComponentKind, PersonalInfoLevel, TitleKind,
    };

    #[test]
    fn test_figure_01() {
        let json = include_bytes!("./rfc9553/figure_01.json");

        let decoded: Card = serde_json::from_slice(json).unwrap();
        let name = decoded.name.unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].kind, NameComponentKind::Given);
        assert_eq!(components[0].value, "John");
        assert_eq!(components[0].phonetic, Some("/ˈdʒɑːn/".to_string()));
        assert_eq!(components[1].kind, NameComponentKind::Surname);
        assert_eq!(components[1].value, "Smith");
        assert_eq!(components[1].phonetic, Some("/smɪθ/".to_string()));
        assert_eq!(name.phonetic_system, Some("ipa".to_string()));
    }

    #[test]
    fn test_figure_06() {
        let json = include_bytes!("./rfc9553/figure_06.json");
        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(card.version, "1.0");
        assert_eq!(card.uid, "22B2C7DF-9120-4969-8460-05956FE6B065");
        assert_eq!(card.kind, Some(CardKind::Individual));
        let name = card.name.unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].kind, NameComponentKind::Given);
        assert_eq!(components[0].value, "John");
        assert_eq!(components[1].kind, NameComponentKind::Surname);
        assert_eq!(components[1].value, "Doe");
        assert_eq!(name.is_ordered, Some(true));
    }

    #[test]
    fn test_figure_07() {
        let json = include_bytes!("./rfc9553/figure_07.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(card.version, "1.0");
    }

    #[test]
    fn test_figure_08() {
        let json = include_bytes!("./rfc9553/figure_08.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(card.created, Some("2022-09-30T14:35:10Z".to_string()));
    }

    #[test]
    fn test_figure_09() {
        let json = include_bytes!("./rfc9553/figure_09.json");
        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(card.kind, Some(CardKind::Individual));
    }

    #[test]
    fn test_figure_10() {
        let json = include_bytes!("./rfc9553/figure_10.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(card.language, Some("de-AT".to_string()));
    }

    #[test]
    fn test_figure_11() {
        let json = include_bytes!("./rfc9553/figure_11.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(card.kind, Some(CardKind::Group));
        let name = card.name.unwrap();
        assert_eq!(name.full, Some("The Doe family".to_string()));
        assert_eq!(card.uid, "urn:uuid:ab4310aa-fa43-11e9-8f0b-362b9e155667");
        let members = card.members.unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(
            members.get("urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af"),
            Some(&true)
        );
        assert_eq!(
            members.get("urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519"),
            Some(&true)
        );
    }

    #[test]
    fn test_figure_12() {
        let json = include_bytes!("./rfc9553/figure_12.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(
            card.prod_id,
            Some("ACME Contacts App version 1.23.5".to_string())
        );
    }

    #[test]
    fn test_figure_13() {
        let json = include_bytes!("./rfc9553/figure_13.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let related_to = card.related_to.unwrap();
        assert_eq!(related_to.len(), 2);
        let f81d4fae = related_to
            .get("urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6")
            .unwrap();
        let relation = f81d4fae.relation.as_ref().unwrap();
        assert_eq!(relation.get("friend").unwrap(), &true);
        let email = related_to.get("8cacdfb7d1ffdb59@example.com").unwrap();
        let relation = email.relation.as_ref().unwrap();
        assert_eq!(relation.len(), 0);
    }

    #[test]
    fn test_figure_14() {
        let json = include_bytes!("./rfc9553/figure_14.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(card.uid, "urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6");
    }

    #[test]
    fn test_figure_15() {
        let json = include_bytes!("./rfc9553/figure_15.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(card.updated, Some("2021-10-31T22:27:10Z".to_string()));
    }

    #[test]
    fn test_figure_16() {
        let json = include_bytes!("./rfc9553/figure_16.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let name = card.name.unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].kind, NameComponentKind::Given);
        assert_eq!(components[0].value, "Vincent");
        assert_eq!(components[1].kind, NameComponentKind::Surname);
        assert_eq!(components[1].value, "van Gogh");
        assert_eq!(name.is_ordered, Some(true));
    }

    #[test]
    fn test_figure_17() {
        let json = include_bytes!("./rfc9553/figure_17.json");
        let card: Card = serde_json::from_slice(json).unwrap();
        let name = card.name.unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 3);
        assert_eq!(components[0].kind, NameComponentKind::Given);
        assert_eq!(components[0].value, "Diego");
        assert_eq!(components[1].kind, NameComponentKind::Surname);
        assert_eq!(components[1].value, "Rivera");
        assert_eq!(components[2].kind, NameComponentKind::Surname2);
        assert_eq!(components[2].value, "Barrientos");
        assert_eq!(name.is_ordered, Some(true));
    }

    #[test]
    fn test_figure_18() {
        let json = include_bytes!("./rfc9553/figure_18.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let name = card.name.unwrap();
        assert_eq!(name.full, Some("Mr. John Q. Public, Esq.".to_string()));
    }

    #[test]
    fn test_figure_19() {
        let json = include_bytes!("./rfc9553/figure_19.json");
        let card: Card = serde_json::from_slice(json).unwrap();
        let name = card.name.unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 3);
        assert_eq!(components[0].kind, NameComponentKind::Given);
        assert_eq!(components[0].value, "Robert");
        assert_eq!(components[1].kind, NameComponentKind::Given2);
        assert_eq!(components[1].value, "Pau");
        assert_eq!(components[2].kind, NameComponentKind::Surname);
        assert_eq!(components[2].value, "Shou Chang");
        assert_eq!(name.is_ordered, Some(true));
        let sort_as = name.sort_as.unwrap();
        assert_eq!(sort_as.get("surname"), Some(&"Pau Shou Chang".to_string()));
        assert_eq!(sort_as.get("given"), Some(&"Robert".to_string()));
    }

    #[test]
    fn test_figure_20() {
        let json = include_bytes!("./rfc9553/figure_20.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        assert_eq!(card.language, Some("zh-Hant".to_string()));
        let name = card.name.unwrap();
        let components = name.components.unwrap();
        assert_eq!(components.len(), 4);
        assert_eq!(components[0].kind, NameComponentKind::Surname);
        assert_eq!(components[0].value, "孫");
        assert_eq!(components[1].kind, NameComponentKind::Given);
        assert_eq!(components[1].value, "中山");
        assert_eq!(components[2].kind, NameComponentKind::Given2);
        assert_eq!(components[2].value, "文");
        assert_eq!(components[3].kind, NameComponentKind::Given2);
        assert_eq!(components[3].value, "逸仙");
        let localizations = card.localizations.unwrap();
        let yue = localizations.get("yue").unwrap();
        let yue = match yue {
            LocalizationObject::PatchObject(patch) => patch,
            _ => panic!("Expected PatchObject"),
        };
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
        let json = include_bytes!("./rfc9553/figure_21.json");

        let nicknames_decoded: Card = serde_json::from_slice(json).unwrap();
        let nicknames = nicknames_decoded.nicknames.unwrap();
        let k391 = nicknames.get("k391").unwrap();
        assert_eq!(k391.name, "Johnny");
    }

    #[test]
    fn test_figure_22() {
        let json = include_bytes!("./rfc9553/figure_22.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let organizations = card.organizations.unwrap();
        assert_eq!(organizations.len(), 1);
        let o1 = organizations.get("o1").unwrap();
        assert_eq!(o1.name, Some("ABC, Inc.".to_string()));
        let units = o1.units.as_ref().unwrap();
        assert_eq!(units.len(), 2);
        assert_eq!(units[0].name, "North American Division");
        assert_eq!(units[1].name, "Marketing");
        assert_eq!(o1.sort_as, Some("ABC".to_string()));
    }

    #[test]
    fn test_figure_23() {
        let json = include_bytes!("./rfc9553/figure_23.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let speak_to_as = card.speak_to_as.unwrap();
        assert_eq!(speak_to_as.grammatical_gender, Some("neuter".to_string()));
        let pronouns = speak_to_as.pronouns.unwrap();
        assert_eq!(pronouns.len(), 2);
        let k19 = pronouns.get("k19").unwrap();
        assert_eq!(k19.pronouns, "they/them");
        assert_eq!(k19.pref, Some(2));
        let k32 = pronouns.get("k32").unwrap();
        assert_eq!(k32.pronouns, "xe/xir");
        assert_eq!(k32.pref, Some(1));
    }

    #[test]
    fn test_figure_24() {
        let json = include_bytes!("./rfc9553/figure_24.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let titles = card.titles.unwrap();
        assert_eq!(titles.len(), 2);
        let le9 = titles.get("le9").unwrap();
        assert_eq!(le9.kind, Some(TitleKind::Title));
        assert_eq!(le9.name, "Research Scientist");
        let k2 = titles.get("k2").unwrap();
        assert_eq!(k2.kind, Some(TitleKind::Role));
        assert_eq!(k2.name, "Project Leader");
        assert_eq!(k2.organization_id, Some("o2".to_string()));
        let organizations = card.organizations.unwrap();
        assert_eq!(organizations.len(), 1);
        let o2 = organizations.get("o2").unwrap();
        assert_eq!(o2.name, Some("ABC, Inc.".to_string()));
    }

    #[test]
    fn test_figure_25() {
        let json = include_bytes!("./rfc9553/figure_25.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let emails = card.emails.unwrap();
        assert_eq!(emails.len(), 2);
        let e1 = emails.get("e1").unwrap();
        assert_eq!(e1.address, "jqpublic@xyz.example.com");
        let contexts = e1.contexts.as_ref().unwrap();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts.get("work"), Some(&true));
        let e2 = emails.get("e2").unwrap();
        assert_eq!(e2.address, "jane_doe@example.com");
        assert_eq!(e2.pref, Some(1));
    }

    #[test]
    fn test_figure_26() {
        let json = include_bytes!("./rfc9553/figure_26.json");
        // {
        //     "onlineServices": {
        //         "x1": {
        //             "uri": "xmpp:alice@example.com"
        //         },
        //         "x2": {
        //             "service": "Mastodon",
        //             "user": "@alice@example2.com",
        //             "uri": "https://example2.com/@alice"
        //         }
        //     }
        // }
        let card: Card = serde_json::from_slice(json).unwrap();
        let online_services = card.online_services.unwrap();
        assert_eq!(online_services.len(), 2);
        let x1 = online_services.get("x1").unwrap();
        assert_eq!(x1.uri, Some("xmpp:alice@example.com".to_string()));
        let x2 = online_services.get("x2").unwrap();
        assert_eq!(x2.service, Some("Mastodon".to_string()));
        assert_eq!(x2.user, Some("@alice@example2.com".to_string()));
        assert_eq!(x2.uri, Some("https://example2.com/@alice".to_string()));
    }
    #[test]
    fn test_figure_27() {
        let json = include_bytes!("./rfc9553/figure_27.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let phones = card.phones.unwrap();
        assert_eq!(phones.len(), 2);
        let tel0 = phones.get("tel0").unwrap();
        assert_eq!(tel0.number, "tel:+1-555-555-5555;ext=5555");
        let contexts = tel0.contexts.as_ref().unwrap();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts.get("private"), Some(&true));
        let features = tel0.features.as_ref().unwrap();
        assert_eq!(features.len(), 1);
        assert_eq!(features.get("voice"), Some(&true));
        let tel3 = phones.get("tel3").unwrap();
        assert_eq!(tel3.number, "tel:+1-201-555-0123");
        let contexts = tel3.contexts.as_ref().unwrap();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts.get("work"), Some(&true));
    }

    #[test]
    fn test_figure_28() {
        let json = include_bytes!("./rfc9553/figure_28.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let preferred_languages = card.preferred_languages.unwrap();
        assert_eq!(preferred_languages.len(), 3);
        let l1 = preferred_languages.get("l1").unwrap();
        assert_eq!(l1.language, "en");
        let contexts = l1.contexts.as_ref().unwrap();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts.get("work"), Some(&true));
        assert_eq!(l1.pref, Some(1));
        let l2 = preferred_languages.get("l2").unwrap();
        assert_eq!(l2.language, "fr");
        let contexts = l2.contexts.as_ref().unwrap();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts.get("work"), Some(&true));
        assert_eq!(l2.pref, Some(2));
        let l3 = preferred_languages.get("l3").unwrap();
        assert_eq!(l3.language, "fr");
        let contexts = l3.contexts.as_ref().unwrap();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts.get("private"), Some(&true));
    }

    #[test]
    fn test_figure_29() {
        let json = include_bytes!("./rfc9553/figure_29.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let calendars = card.calendars.unwrap();
        assert_eq!(calendars.len(), 2);
        let cal_a = calendars.get("calA").unwrap();
        assert_eq!(cal_a.kind, Some(CalendarKind::Calendar));
        assert_eq!(cal_a.uri, "webcal://calendar.example.com/calA.ics");
        let project_a = calendars.get("project-a").unwrap();
        assert_eq!(project_a.kind, Some(CalendarKind::FreeBusy));
        assert_eq!(project_a.uri, "https://calendar.example.com/busy/project-a");
    }

    #[test]
    fn test_figure_30() {
        let json = include_bytes!("./rfc9553/figure_30.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let scheduling_addresses = card.scheduling_addresses.unwrap();
        assert_eq!(scheduling_addresses.len(), 1);
        let sched1 = scheduling_addresses.get("sched1").unwrap();
        assert_eq!(sched1.uri, "mailto:janedoe@example.com");
    }

    #[test]
    fn test_figure_31() {
        let json = include_bytes!("./rfc9553/figure_31.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let addresses = card.addresses.unwrap();
        assert_eq!(addresses.len(), 1);
        let k23 = addresses.get("k23").unwrap();
        let components = k23.components.as_ref().unwrap();
        assert_eq!(components.len(), 8);
        assert_eq!(components[0].kind, AddressComponentKind::Number);
        assert_eq!(components[0].value, "54321");
        assert_eq!(components[1].kind, AddressComponentKind::Separator);
        assert_eq!(components[1].value, " ");
        assert_eq!(components[2].kind, AddressComponentKind::Name);
        assert_eq!(components[2].value, "Oak St");
        assert_eq!(components[3].kind, AddressComponentKind::Locality);
        assert_eq!(components[3].value, "Reston");
        assert_eq!(components[4].kind, AddressComponentKind::Region);
        assert_eq!(components[4].value, "VA");
        assert_eq!(components[5].kind, AddressComponentKind::Separator);
        assert_eq!(components[5].value, " ");
        assert_eq!(components[6].kind, AddressComponentKind::Postcode);
        assert_eq!(components[6].value, "20190");
        assert_eq!(components[7].kind, AddressComponentKind::Country);
        assert_eq!(components[7].value, "USA");
        assert_eq!(k23.country_code, Some("US".to_string()));
        assert_eq!(k23.default_separator, Some(", ".to_string()));
        assert_eq!(k23.is_ordered, Some(true));
    }

    #[test]
    fn test_figure_32() {
        let json = include_bytes!("./rfc9553/figure_32.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let addresses = card.addresses.unwrap();
        assert_eq!(addresses.len(), 1);
        let k25 = addresses.get("k25").unwrap();
        let components = k25.components.as_ref().unwrap();
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
        assert_eq!(k25.default_separator, Some(", ".to_string()));
        assert_eq!(k25.is_ordered, Some(true));
    }

    #[test]
    fn test_figure_33() {
        let json = include_bytes!("./rfc9553/figure_33.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let addresses = card.addresses.unwrap();
        assert_eq!(addresses.len(), 1);
        let k26 = addresses.get("k26").unwrap();
        let components = k26.components.as_ref().unwrap();
        assert_eq!(components.len(), 9);
        assert_eq!(components[0].kind, AddressComponentKind::Block);
        assert_eq!(components[0].value, "2-7");
        assert_eq!(components[1].kind, AddressComponentKind::Separator);
        assert_eq!(components[1].value, "-");
        assert_eq!(components[2].kind, AddressComponentKind::Number);
        assert_eq!(components[2].value, "2");
        assert_eq!(components[3].kind, AddressComponentKind::Separator);
        assert_eq!(components[3].value, " ");
        assert_eq!(components[4].kind, AddressComponentKind::District);
        assert_eq!(components[4].value, "Marunouchi");
        assert_eq!(components[5].kind, AddressComponentKind::Locality);
        assert_eq!(components[5].value, "Chiyoda-ku");
        assert_eq!(components[6].kind, AddressComponentKind::Region);
        assert_eq!(components[6].value, "Tokyo");
        assert_eq!(components[7].kind, AddressComponentKind::Separator);
        assert_eq!(components[7].value, " ");
        assert_eq!(components[8].kind, AddressComponentKind::Postcode);
        assert_eq!(components[8].value, "100-8994");
        assert_eq!(k26.default_separator, Some(", ".to_string()));
    }

    #[test]
    fn test_figure_34() {
        let json = include_bytes!("./rfc9553/figure_34.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let crypto_keys = card.crypto_keys.unwrap();
        assert_eq!(crypto_keys.len(), 1);
        let mykey1 = crypto_keys.get("mykey1").unwrap();
        assert_eq!(mykey1.uri, "https://www.example.com/keys/jdoe.cer");
    }

    #[test]
    fn test_figure_35() {
        let json = include_bytes!("./rfc9553/figure_35.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let crypto_keys = card.crypto_keys.unwrap();
        assert_eq!(crypto_keys.len(), 1);
        let mykey2 = crypto_keys.get("mykey2").unwrap();
        assert_eq!(
            mykey2.uri,
            "data:application/pgp-keys;base64,LS0tLS1CRUdJTiBSU0EgUFVCTElDIEtFWS0tLS0tCk1JSUJDZ0tDQVFFQSt4R1ovd2N6OXVnRnBQMDdOc3BvNlUxN2wwWWhGaUZweHhVNHBUazNMaWZ6OVIzenNJc3UKRVJ3dGE3K2ZXSWZ4T28yMDhldHQvamhza2lWb2RTRXQzUUJHaDRYQmlweVdvcEt3WjkzSEhhRFZaQUFMaS8yQQoreFRCdFdkRW83WEdVdWpLRHZDMi9hWkt1a2ZqcE9pVUk4QWhMQWZqbWxjRC9VWjFRUGgwbUhzZ2xSTkNtcEN3Cm13U1hBOVZObWh6K1BpQitEbWw0V1duS1cvVkhvMnVqVFh4cTcrZWZNVTRIMmZueTNTZTNLWU9zRlBGR1oxVE4KUVNZbEZ1U2hXckhQdGlMbVVkUG9QNkNWMm1NTDF0aytsN0RJSXFYclFoTFVLREFDZU01cm9NeDBrTGhVV0I4UAorMHVqMUNObE5ONEpSWmxDN3hGZnFpTWJGUlU5WjRONll3SURBUUFCCi0tLS0tRU5EIFJTQSBQVUJMSUMgS0VZLS0tLS0K"
        );
    }

    #[test]
    fn test_figure_36() {
        let json = include_bytes!("./rfc9553/figure_36.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let directories = card.directories.unwrap();
        assert_eq!(directories.len(), 2);
        let dir1 = directories.get("dir1").unwrap();
        assert_eq!(dir1.kind, Some(DirectoryKind::Entry));
        assert_eq!(
            dir1.uri,
            "https://dir.example.com/addrbook/jdoe/Jean%20Dupont.vcf"
        );
        let dir2 = directories.get("dir2").unwrap();
        assert_eq!(dir2.kind, Some(DirectoryKind::Directory));
        assert_eq!(
            dir2.uri,
            "ldap://ldap.example/o=Example%20Tech,ou=Engineering"
        );
    }

    #[test]
    fn test_figure_37() {
        let json = include_bytes!("./rfc9553/figure_37.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let links = card.links.unwrap();
        assert_eq!(links.len(), 1);
        let link3 = links.get("link3").unwrap();
        assert_eq!(link3.kind, Some(LinkKind::Contact));
        assert_eq!(link3.uri, "mailto:contact@example.com");
        assert_eq!(link3.pref, Some(1));
    }

    #[test]
    fn test_figure_38() {
        let json = include_bytes!("./rfc9553/figure_38.json");
        let card: Card = serde_json::from_slice(json).unwrap();
        let media = card.media.unwrap();
        assert_eq!(media.len(), 3);
        let res45 = media.get("res45").unwrap();
        assert_eq!(res45.kind, MediaKind::Sound);
        assert_eq!(
            res45.uri,
            "CID:JOHNQ.part8.19960229T080000.xyzMail@example.com"
        );
        let res47 = media.get("res47").unwrap();
        assert_eq!(res47.kind, MediaKind::Logo);
        assert_eq!(res47.uri, "https://www.example.com/pub/logos/abccorp.jpg");
        let res1 = media.get("res1").unwrap();
        assert_eq!(res1.kind, MediaKind::Photo);
        assert_eq!(
            res1.uri,
            "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAASABIAAD/4..."
        );
    }

    #[test]
    fn test_figure_39() {
        let json = include_bytes!("./rfc9553/figure_39.json");
        let name: Card = serde_json::from_slice(json).unwrap();
        let components = name.name.unwrap().components.unwrap();
        assert_eq!(components.len(), 4);
        assert_eq!(components[0].kind, NameComponentKind::Title);
        assert_eq!(components[0].value, "Mr.");
        assert_eq!(components[1].kind, NameComponentKind::Given);
        assert_eq!(components[1].value, "Ivan");
        assert_eq!(components[2].kind, NameComponentKind::Given2);
        assert_eq!(components[2].value, "Petrovich");
        assert_eq!(components[3].kind, NameComponentKind::Surname);
        assert_eq!(components[3].value, "Vasiliev");
        let localizations = name.localizations.unwrap();
        let uk_cyrl = localizations.get("uk-Cyrl").unwrap();
        let uk_cyrl = match uk_cyrl {
            LocalizationObject::Name { name } => name,
            _ => panic!("Expected PatchObject"),
        };
        let components_uk_cyrl = uk_cyrl.components.as_ref().unwrap();
        assert_eq!(components_uk_cyrl.len(), 4);
        assert_eq!(components_uk_cyrl[0].kind, NameComponentKind::Title);
        assert_eq!(components_uk_cyrl[0].value, "г-н");
        assert_eq!(components_uk_cyrl[1].kind, NameComponentKind::Given);
        assert_eq!(components_uk_cyrl[1].value, "Иван");
        assert_eq!(components_uk_cyrl[2].kind, NameComponentKind::Given2);
        assert_eq!(components_uk_cyrl[2].value, "Петрович");
        assert_eq!(components_uk_cyrl[3].kind, NameComponentKind::Surname);
        assert_eq!(components_uk_cyrl[3].value, "Васильев");
    }

    #[test]
    fn test_figure_40() {
        let json = include_bytes!("./rfc9553/figure_40.json");
        let card: Card = serde_json::from_slice(json).unwrap();
        let name = card.name.unwrap();
        assert_eq!(name.full, Some("Gabriel García Márquez".to_string()));
        let titles = card.titles.unwrap();
        assert_eq!(titles.len(), 1);
        let t1 = titles.get("t1").unwrap();
        assert_eq!(t1.kind, Some(TitleKind::Title));
        assert_eq!(t1.name, "novelist");
        let localizations = card.localizations.unwrap();
        let es = localizations.get("es").unwrap();
        let title = match es {
            LocalizationObject::PatchObject(patch) => patch.get("titles/t1/name").unwrap(),
            _ => panic!("Expected PatchObject"),
        };
        assert_eq!(title, "escritor");
    }

    #[test]
    fn test_figure_41() {
        let json = include_bytes!("./rfc9553/figure_41.json");
        let card: Card = serde_json::from_slice(json).unwrap();
        let anniversaries = card.anniversaries.unwrap();
        assert_eq!(anniversaries.len(), 2);
        let k8 = anniversaries.get("k8").unwrap();
        assert_eq!(k8.kind, "birth");
        let date_k8 = match &k8.date {
            DateObject::PartialDate(date) => date,
            _ => panic!("Expected PartialDate"),
        };
        assert_eq!(date_k8.year, Some(1953));
        assert_eq!(date_k8.month, Some(4));
        assert_eq!(date_k8.day, Some(15));
        let k9 = anniversaries.get("k9").unwrap();
        assert_eq!(k9.kind, "death");
        let date_k9 = match &k9.date {
            DateObject::Timestamp(date) => date,
            _ => panic!("Expected Timestamp"),
        };
        assert_eq!(date_k9.utc, "2019-10-15T23:10:00Z");
        let place = k9.place.as_ref().unwrap();
        assert_eq!(
            place.full,
            Some("4445 Tree Street\nNew England, ND 58647\nUSA".to_string())
        );
    }

    #[test]
    fn test_figure_42() {
        let json = include_bytes!("./rfc9553/figure_42.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let keywords = card.keywords.unwrap();
        assert_eq!(keywords.len(), 2);
        assert_eq!(keywords.get("internet"), Some(&true));
        assert_eq!(keywords.get("IETF"), Some(&true));
    }

    #[test]
    fn test_figure_43() {
        let json = include_bytes!("./rfc9553/figure_43.json");

        let card: Card = serde_json::from_slice(json).unwrap();
        let notes = card.notes.unwrap();
        assert_eq!(notes.len(), 1);
        let n1 = notes.get("n1").unwrap();
        assert_eq!(n1.note, "Open office hours are 1600 to 1715 EST, Mon-Fri");
        assert_eq!(n1.created, Some("2022-11-23T15:01:32Z".to_string()));
        let author = n1.author.as_ref().unwrap();
        assert_eq!(author.name, Some("John".to_string()));
    }

    #[test]
    fn test_figure_44() {
        let json = include_bytes!("./rfc9553/figure_44.json");
        let card: Card = serde_json::from_slice(json).unwrap();
        let personal_info = card.personal_info.unwrap();
        assert_eq!(personal_info.len(), 3);
        let pi2 = personal_info.get("pi2").unwrap();
        assert_eq!(pi2.kind, "expertise");
        assert_eq!(pi2.value, "chemistry");
        assert_eq!(pi2.level, Some(PersonalInfoLevel::High));
        let pi1 = personal_info.get("pi1").unwrap();
        assert_eq!(pi1.kind, "hobby");
        assert_eq!(pi1.value, "reading");
        assert_eq!(pi1.level, Some(PersonalInfoLevel::High));
        let pi6 = personal_info.get("pi6").unwrap();
        assert_eq!(pi6.kind, "interest");
        assert_eq!(pi6.value, "r&b music");
        assert_eq!(pi6.level, Some(PersonalInfoLevel::Medium));
    }
}
