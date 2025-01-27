// we force to use the without `type` API by disabling the feature "typed"
// because rfc examples are in without `type` format
#![cfg(not(feature = "typed"))]

// These test are present to validate that the examples in the RFC can be
// correctly created, encoded and decoded by the library.
mod test {

    use std::collections::HashMap;

    use jscontact::{
        Card, CardVersion, Name, NameComponent, NameComponentKind, PersonalInfo, PersonalInfoKind,
        PersonalInfoLevel, PhoneticSystem,
    };

    #[test]
    fn test_figure_01() {
        let verifier = include_bytes!("./rfc9553/figure_01.json");
        let verifier: Card = serde_json::from_slice(verifier).unwrap();
        let verifier = serde_json::to_value(verifier).unwrap();

        let mut card = Card::new(
            CardVersion::OneDotZero,
            "22B2C7DF-9120-4969-8460-05956FE6B065",
        );
        let mut name = Name::default();
        let mut name_component_1 = NameComponent::new(NameComponentKind::Given, "John");
        name_component_1.phonetic = Some("/ˈdʒɑːn/".to_string());
        let mut name_component_2 = NameComponent::new(NameComponentKind::Surname, "Smith");
        name_component_2.phonetic = Some("/smɪθ/".to_string());
        name.components = Some(vec![name_component_1, name_component_2]);
        name.phonetic_system = Some(PhoneticSystem::Ipa);
        card.name = Some(name);
        let card_value = serde_json::to_value(card).unwrap();
        assert_eq!(verifier, card_value);
    }

    #[test]
    fn test_figure_44() {
        let verifier = include_bytes!("./rfc9553/figure_44.json");
        let verifier: Card = serde_json::from_slice(verifier).unwrap();
        let verifier = serde_json::to_value(verifier).unwrap();

        let mut card = Card::new(
            CardVersion::OneDotZero,
            "22B2C7DF-9120-4969-8460-05956FE6B065",
        );
        let mut personal_infos = HashMap::new();
        let mut personal_info = PersonalInfo::new(PersonalInfoKind::Expertise, "chemistry");
        personal_info.level = Some(PersonalInfoLevel::High);
        personal_infos.insert("pi2".to_string(), personal_info.into());
        let mut personal_info = PersonalInfo::new(PersonalInfoKind::Hobby, "reading");
        personal_info.level = Some(PersonalInfoLevel::High);
        personal_infos.insert("pi1".to_string(), personal_info.into());
        let mut personal_info = PersonalInfo::new(PersonalInfoKind::Interest, "r&b music");
        personal_info.level = Some(PersonalInfoLevel::Medium);
        personal_infos.insert("pi6".to_string(), personal_info.into());
        card.personal_info = Some(personal_infos);
        let card_value = serde_json::to_value(card).unwrap();
        assert_eq!(verifier, card_value);
    }
}
