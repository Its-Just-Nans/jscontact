//! Card Object

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Address, AddressComponent, Anniversary, Calendar, CardKind, CardVersion, CryptoKey, Directory,
    EmailAddress, LanguagePref, Link, Media, Name, NameComponent, Nickname, Note, OnlineService,
    Organization, PersonalInfo, Phone, Relation, SchedulingAddress, SpeakToAs, Title,
};

/// Represents the primary Card object as defined in RFC 9553, storing metadata and contact properties.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// The JSContact type of the Card object. Must be "Card".
    /// Not localized.
    #[serde(rename = "@type")]
    card_type: String,
    /// The JSContact version of this Card.
    /// Not localized.
    pub version: CardVersion,
    /// The date and time when the Card was created.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// A unique identifier for the Card.
    /// Not localized.
    pub uid: String,
    /// The kind of entity the Card represents (e.g., individual, group).
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CardKind>,
    /// The language used in the Card (e.g., en, fr).
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Members of a group Card, if applicable.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<HashMap<String, bool>>,
    /// Identifier for the product that created the Card.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prod_id: Option<String>,
    /// Related Cards with their relationship types.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_to: Option<HashMap<String, Relation>>,
    /// The last modification time of the Card.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// The name of the entity represented by the Card.
    /// Localized by [`localize_name`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /// Nicknames of the entity.
    /// Localized by [`localize_nicknames`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nicknames: Option<HashMap<String, Nickname>>,
    /// Organizations associated with the entity.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<HashMap<String, Organization>>,
    /// How to address or refer to the entity.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speak_to_as: Option<SpeakToAs>,
    /// Job titles or roles of the entity.
    /// Localized by [`localize_titles`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<HashMap<String, Title>>,
    /// Email addresses for contacting the entity.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<HashMap<String, EmailAddress>>,
    /// Online services or social media associated with the entity.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_services: Option<HashMap<String, OnlineService>>,
    /// Phone numbers for contacting the entity.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<HashMap<String, Phone>>,
    /// Preferred languages for communication.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_languages: Option<HashMap<String, LanguagePref>>,
    /// The calendaring resources of the entity represented by the Card, such as to look up free-busy information.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendars: Option<HashMap<String, Calendar>>,
    /// The scheduling addresses by which the entity may receive calendar scheduling invitations.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_addresses: Option<HashMap<String, SchedulingAddress>>,
    /// Localizations provide language-specific alternatives for existing property values and SHOULD NOT add new properties.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    localizations: Option<HashMap<String, HashMap<String, Value>>>,
    /// The memorable dates and events for the entity represented by the Card.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anniversaries: Option<HashMap<String, Anniversary>>,
    /// The scheduling addresses by which the entity may receive calendar scheduling invitations.
    /// Localized by [`localize_addresses`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<HashMap<String, Address>>,
    /// The cryptographic resources such as public keys and certificates associated with the entity represented by the Card.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_keys: Option<HashMap<String, CryptoKey>>,
    /// The directories containing information about the entity represented by the Card.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<HashMap<String, Directory>>,
    /// The links to resources that do not fit any of the other use-case-specific resource properties.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, Link>>,
    /// The media resources such as photographs, avatars, or sounds that are associated with the entity represented by the Card.
    /// Not localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<HashMap<String, Media>>,
    /// The set of free-text keywords, also known as tags.
    /// Localized by [`localize_keywords`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<HashMap<String, bool>>,
    /// The free-text notes that are associated with the Card.
    /// Localized by [`localize_notes`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<HashMap<String, Note>>,
    /// The personal information of the entity represented by the Card.
    /// Localized by [`localize_personal_info`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_info: Option<HashMap<String, PersonalInfo>>,
}

impl Card {
    /// Creates a new Card object with the specified version and unique identifier.
    pub fn new(version: CardVersion, uid: &str) -> Self {
        Self {
            card_type: "Card".to_string(),
            version,
            uid: uid.to_string(),
            created: None,
            kind: None,
            language: None,
            members: None,
            prod_id: None,
            related_to: None,
            updated: None,
            name: None,
            nicknames: None,
            organizations: None,
            speak_to_as: None,
            titles: None,
            emails: None,
            online_services: None,
            phones: None,
            preferred_languages: None,
            calendars: None,
            scheduling_addresses: None,
            localizations: None,
            anniversaries: None,
            addresses: None,
            crypto_keys: None,
            directories: None,
            links: None,
            media: None,
            keywords: None,
            notes: None,
            personal_info: None,
        }
    }

    /// Wrapper around serde_json
    /// # Errors
    /// Will return an error if the input is not a valid Card object.
    pub fn from_slice(json_slice: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(json_slice)
    }

    /// Wrapper around serde_json
    /// # Errors
    /// Will return an error if the input is not a valid Card object.
    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    /// Wrapper around serde_json
    /// # Errors
    /// Will return an error if the input is not a valid Card object.
    pub fn from_value(value: Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }

    /// Wrapper around serde_json
    /// # Errors
    /// Will return an error if the input is not a valid Card object.
    pub fn try_from_str(json_string: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json_string)
    }

    /// Wrapper around serde_json
    /// # Errors
    /// Will return an error if the input is not a valid Card object.
    pub fn serialize_str(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Creates a new Card object with the latest version and the specified unique identifier.
    pub fn new_with_latest_version(uid: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            card_type: "Card".to_string(),
            uid: uid.to_string(),
            ..Card::new(CardVersion::OneDotZero, uid)
        }
    }

    /// Get the Raw Localizations
    pub fn get_raw_localizations(&self) -> Option<&HashMap<String, HashMap<String, Value>>> {
        self.localizations.as_ref()
    }

    /// Adds a localization to the Card object.
    pub fn add_localization(&mut self, language: &str, value: HashMap<String, Value>) {
        match &mut self.localizations {
            Some(localizations_map) => {
                localizations_map.insert(language.to_string(), value);
            }
            None => {
                let mut localizations_map = HashMap::new();
                localizations_map.insert(language.to_string(), value);
                self.localizations = Some(localizations_map);
            }
        };
    }

    /// Get available languages in the Card object.
    pub fn get_available_languages(&self) -> Vec<String> {
        match &self.localizations {
            Some(localizations_map) => localizations_map.keys().cloned().collect(),
            None => Vec::new(),
        }
    }

    /// Get the localized Card object for the specified language.
    /// # Errors
    /// Will return an error if translation are invalid.
    pub fn get_localized(&self, language: &str) -> Result<Card, String> {
        let lang = language.to_string();
        let localizations = match &self.localizations {
            Some(localizations_map) => localizations_map,
            None => return Ok(self.clone()),
        };
        let localized_lang = match localizations.get(&lang) {
            Some(lang) => lang,
            None => return Ok(self.clone()),
        };
        // iter on localized_lang and set the values
        let mut card = self.clone();
        for (key, value) in localized_lang.iter() {
            if key.starts_with("name") {
                localize_name(&mut card, key, value)?;
            } else if key.starts_with("titles") {
                localize_titles(&mut card, key, value)?;
            } else if key.starts_with("addresses") {
                localize_addresses(&mut card, key, value)?;
            } else if key.starts_with("nicknames") {
                localize_nicknames(&mut card, key, value)?;
            } else if key.starts_with("personalInfo") {
                localize_personal_info(&mut card, key, value)?;
            } else if key.starts_with("notes") {
                localize_notes(&mut card, key, value)?;
            } else if key.starts_with("keywords") {
                localize_keywords(&mut card, key, value)?;
            }
        }
        Ok(card)
    }
}

/// remove the first character of a string
fn remove_first(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.as_str()
}

/// Localize the Name
fn localize_name(card: &mut Card, key: &str, value: &Value) -> Result<(), String> {
    if key == "name" {
        card.name = serde_json::from_value(value.clone()).ok();
        return Ok(());
    }
    let curr_name = match &mut card.name {
        Some(name) => name,
        None => &mut Name::default(),
    };
    let key = key.replace("name/", "");
    if key.starts_with("components") {
        if key == "components" {
            curr_name.components = serde_json::from_value(value.clone()).ok();
            return Ok(());
        }
        let components = match &mut curr_name.components {
            Some(components) => components,
            None => &mut vec![],
        };
        let key = key.replace("components/", "");
        let keys = key.split("/").collect::<Vec<&str>>();
        let Some(idx) = keys.first() else {
            return Err("Index out of bounds".into());
        };
        let key = key.replace(&format!("{}/", idx), "");
        let Ok(idx) = idx.parse::<usize>() else {
            return Err("Index out of bounds".into());
        };
        if components.len() <= idx {
            return Err("Index out of bounds".into());
        }
        let component: &mut NameComponent = &mut components[idx];
        if key == "value" {
            let Ok(str) = serde_json::from_value::<String>(value.clone()) else {
                return Err("Invalid value".into());
            };
            component.value = str;
        } else if key == "phonetic" {
            component.phonetic = serde_json::from_value(value.clone()).ok();
        }
        curr_name.components = Some(components.clone());
    } else if key == "full" {
        curr_name.full = serde_json::from_value(value.clone()).ok();
    } else if key == "phoneticSystem" {
        curr_name.phonetic_system = serde_json::from_value(value.clone()).ok();
    } else if key == "phoneticScript" {
        curr_name.phonetic_script = serde_json::from_value(value.clone()).ok();
    }
    card.name = Some(curr_name.clone());
    Ok(())
}

/// Localize the Titles
fn localize_titles(card: &mut Card, key: &str, value: &Value) -> Result<(), String> {
    if key == "titles" {
        card.titles = serde_json::from_value(value.clone()).ok();
        return Ok(());
    }
    let titles = match &mut card.titles {
        Some(titles) => titles,
        None => &mut HashMap::new(),
    };
    let key = key.replace("titles/", "");
    let keys = key.split("/").collect::<Vec<&str>>();
    let Some(idx_key) = keys.first() else {
        return Err("Index out of bounds".into());
    };
    let idx_key = idx_key.to_string();
    let key = key.replace(&idx_key, "");
    let key = if key.is_empty() {
        let Ok(title) = serde_json::from_value::<Title>(value.clone()) else {
            return Err("Invalid value".into());
        };
        titles.insert(idx_key, title);
        card.titles = Some(titles.clone());
        return Ok(());
    } else {
        remove_first(&key)
    };
    let Some(title) = titles.get_mut(&idx_key) else {
        return Err(format!("titles key '{}' not found", idx_key));
    };
    if key == "name" {
        let Ok(str) = serde_json::from_value::<String>(value.clone()) else {
            return Err("Invalid value".into());
        };
        title.name = str;
    } else if key == "kind" {
        title.kind = serde_json::from_value(value.clone()).ok();
    } else if key == "organizationId" {
        let Ok(str) = serde_json::from_value::<String>(value.clone()) else {
            return Err("Invalid value".into());
        };
        title.organization_id = Some(str);
    }
    card.titles = Some(titles.clone());
    Ok(())
}

/// Localize the Addresses
fn localize_addresses(card: &mut Card, key: &str, value: &Value) -> Result<(), String> {
    let full_key = key;
    if key == "addresses" {
        card.addresses = serde_json::from_value(value.clone()).ok();
        return Ok(());
    }
    let key = key.replace("addresses/", "");
    let addresses = match &mut card.addresses {
        Some(addresses) => addresses,
        None => &mut HashMap::new(),
    };
    let keys = key.split("/").collect::<Vec<&str>>();
    let Some(idx_key) = keys.first() else {
        return Err("Invalid addresses key".into());
    };
    let idx_key = idx_key.to_string();
    let key = key.replace(&idx_key, "");
    let key = remove_first(&key);
    let Some(address) = addresses.get_mut(&idx_key) else {
        return Err(format!("addresses key '{}' not found", idx_key));
    };
    if key.starts_with("components") {
        if key == "components" {
            address.components = serde_json::from_value(value.clone()).ok();
            card.addresses = Some(addresses.clone());
            return Ok(());
        }
        let components = match &mut address.components {
            Some(components) => components,
            None => &mut vec![],
        };
        let key = key.replace("components/", "");
        let keys = key.split("/").collect::<Vec<&str>>();
        let Some(idx) = keys.first() else {
            return Err("Index out of bounds".into());
        };
        let key = key.replace(idx, "");
        let key = remove_first(&key);
        let Ok(idx) = idx.parse::<usize>() else {
            return Err("Index out of bounds".into());
        };
        while components.len() <= idx {
            components.push(AddressComponent::new(
                crate::AddressComponentKind::Apartment,
                "DEFAULT",
            ));
        }
        if key.is_empty() {
            let Ok(component) = serde_json::from_value::<AddressComponent>(value.clone()) else {
                return Err("Invalid value".into());
            };
            components[idx] = component;
            address.components = Some(components.clone());
            card.addresses = Some(addresses.clone());
            return Ok(());
        }
        let component: &mut AddressComponent = &mut components[idx];
        if key == "value" {
            let Ok(str) = serde_json::from_value::<String>(value.clone()) else {
                return Err(format!(
                    "Invalid value: {} for value (at {})",
                    value, full_key
                ));
            };
            component.value = str;
        } else if key == "kind" {
            let Ok(kind) = serde_json::from_value::<crate::AddressComponentKind>(value.clone())
            else {
                return Err(format!(
                    "Invalid value: {} for kind (at {})",
                    value, full_key
                ));
            };
            component.kind = kind;
        } else if key == "phonetic" {
            component.phonetic = serde_json::from_value(value.clone()).ok();
        }
        address.components = Some(components.clone());
    } else if key == "full" {
        address.full = serde_json::from_value(value.clone()).ok();
    } else if key == "countryCode" {
        address.country_code = serde_json::from_value(value.clone()).ok();
    } else if key == "coordinates" {
        address.coordinates = serde_json::from_value(value.clone()).ok();
    } else if key == "timeZone" {
        address.time_zone = serde_json::from_value(value.clone()).ok();
    } else if key == "contexts" {
        address.contexts = serde_json::from_value(value.clone()).ok();
    } else if key.is_empty() {
        let Ok(addr) = serde_json::from_value::<Address>(value.clone()) else {
            return Err("Invalid value".into());
        };
        addresses.insert(idx_key, addr);
    }
    card.addresses = Some(addresses.clone());
    Ok(())
}

/// Localize the Nicknames
fn localize_nicknames(card: &mut Card, key: &str, value: &Value) -> Result<(), String> {
    if key == "nicknames" {
        card.nicknames = serde_json::from_value(value.clone()).ok();
        return Ok(());
    }
    let nicknames = match &mut card.nicknames {
        Some(nicknames) => nicknames,
        None => &mut HashMap::new(),
    };
    let key = key.replace("nicknames", "");
    let key = if key.is_empty() {
        let Ok(nicks) = serde_json::from_value::<HashMap<String, Nickname>>(value.clone()) else {
            return Err("Invalid value".into());
        };
        *nicknames = nicks;
        card.nicknames = Some(nicknames.clone());
        return Ok(());
    } else {
        remove_first(&key)
    };
    let keys = key.split("/").collect::<Vec<&str>>();
    let Some(idx_key) = keys.first() else {
        return Err("Invalid nicknames key".into());
    };
    let idx_key = idx_key.to_string();
    let key = key.replace(&idx_key, "");
    let key = if key.is_empty() {
        let Ok(nick) = serde_json::from_value::<Nickname>(value.clone()) else {
            return Err("Invalid value".into());
        };
        nicknames.insert(idx_key, nick);
        card.nicknames = Some(nicknames.clone());
        return Ok(());
    } else {
        remove_first(&key)
    };
    let Some(nick) = nicknames.get_mut(&idx_key) else {
        return Err(format!("nicknames key '{}' not found", idx_key));
    };
    if key == "name" {
        let Ok(str) = serde_json::from_value::<String>(value.clone()) else {
            return Err("Invalid value".into());
        };
        nick.name = str;
    }
    card.nicknames = Some(nicknames.clone());
    Ok(())
}

/// Localize the PersonalInfos
fn localize_personal_info(card: &mut Card, key: &str, value: &Value) -> Result<(), String> {
    if key == "personalInfo" {
        card.personal_info = serde_json::from_value(value.clone()).ok();
        return Ok(());
    }
    let personal_infos = match &mut card.personal_info {
        Some(personal_infos) => personal_infos,
        None => &mut HashMap::new(),
    };
    let key = key.replace("personalInfo", "");
    if key.is_empty() {
        let Ok(personal_infos_map) =
            serde_json::from_value::<HashMap<String, PersonalInfo>>(value.clone())
        else {
            return Err("Invalid value".into());
        };
        *personal_infos = personal_infos_map;
        card.personal_info = Some(personal_infos.clone());
        return Ok(());
    }
    let key = remove_first(&key);
    let keys = key.split("/").collect::<Vec<&str>>();
    let Some(idx_key) = keys.first() else {
        return Err("Invalid personalInfo key".into());
    };
    let idx_key = idx_key.to_string();
    let key = key.replace(&idx_key, "");
    if key.is_empty() {
        let Ok(personal_info) = serde_json::from_value::<PersonalInfo>(value.clone()) else {
            return Err("Invalid value".into());
        };
        personal_infos.insert(idx_key, personal_info);
        card.personal_info = Some(personal_infos.clone());
        return Ok(());
    }
    let key = remove_first(&key);
    let Some(personal_info) = personal_infos.get_mut(&idx_key) else {
        return Err(format!("personalInfo key '{}' not found", idx_key));
    };
    if key == "value" {
        let Ok(str) = serde_json::from_value(value.clone()) else {
            return Err("Invalid value".into());
        };
        personal_info.value = str;
    } else if key == "kind" {
        let Ok(kind) = serde_json::from_value(value.clone()) else {
            return Err("Invalid value".into());
        };
        personal_info.kind = kind;
    }
    card.personal_info = Some(personal_infos.clone());
    Ok(())
}

/// Localize the Notes
fn localize_notes(card: &mut Card, key: &str, value: &Value) -> Result<(), String> {
    if key == "notes" {
        card.notes = serde_json::from_value(value.clone()).ok();
        return Ok(());
    }
    let notes = match &mut card.notes {
        Some(notes) => notes,
        None => &mut HashMap::new(),
    };
    let key = key.replace("notes", "");
    if key.is_empty() {
        let Ok(notes_map) = serde_json::from_value::<HashMap<String, Note>>(value.clone()) else {
            return Err("Invalid value".into());
        };
        *notes = notes_map;
        card.notes = Some(notes.clone());
        return Ok(());
    }
    let key = remove_first(&key);
    let keys = key.split("/").collect::<Vec<&str>>();
    let Some(idx_key) = keys.first() else {
        return Err("Invalid notes key".into());
    };
    let idx_key = idx_key.to_string();
    let key = key.replace(&idx_key, "");
    if key.is_empty() {
        let Ok(note) = serde_json::from_value::<Note>(value.clone()) else {
            return Err("Invalid value".into());
        };
        notes.insert(idx_key, note);
        card.notes = Some(notes.clone());
        return Ok(());
    }
    let key = remove_first(&key);
    let Some(note) = notes.get_mut(&idx_key) else {
        return Err(format!("notes key '{}' not found", idx_key));
    };
    if key == "note" {
        let Ok(str) = serde_json::from_value(value.clone()) else {
            return Err("Invalid value".into());
        };
        note.note = str;
    } else if key == "created" {
        note.created = serde_json::from_value(value.clone()).ok();
    } else if key == "author" {
        let Ok(author) = serde_json::from_value(value.clone()) else {
            return Err("Invalid value".into());
        };
        note.author = author;
    }
    card.notes = Some(notes.clone());
    Ok(())
}

/// Localize the Keywords
fn localize_keywords(card: &mut Card, key: &str, value: &Value) -> Result<(), String> {
    if key == "keywords" {
        card.keywords = serde_json::from_value(value.clone()).ok();
        return Ok(());
    }
    Ok(())
}

// fn localize_phones(card: &mut Card, key: &str, value: &Value) -> Result<(), String> {}

// #![deny(
//     missing_docs,
//     clippy::all,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc,
//     clippy::cargo
// )]
// #![warn(clippy::multiple_crate_versions)]

// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// pub mod card;
// pub use card::Card;

// mod resource;
// pub use resource::Resource;

// /// Represents the card version.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// pub enum CardVersion {
//     /// version 1.0
//     #[serde(rename = "1.0")]
//     OneDotZero,
// }

// /// @Resource The calendaring resources of the entity represented by the Card, such as to look up free-busy information.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Calendar {
//     /// The @type property value MUST be "Calendar", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     calendar_type: Option<CalendarType>,
//     /// The kind of the calendar.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub kind: Option<CalendarKind>,
//     /// The media type [RFC2046] of the resource identified by the uri property value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub media_type: Option<String>,
//     /// The resource value.
//     pub uri: String,
//     /// The contexts in which to use this resource.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// The preference of the resource in relation to other resources.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u64>,
//     /// A custom label for the value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
// }

// /// Calendar @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum CalendarType {
//     /// Calendar @type
//     Calendar,
// }

// impl Calendar {
//     /// Creates a new Calendar object with the specified URI.
//     pub fn new(uri: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             calendar_type: Some(CalendarType::Calendar),
//             uri: uri.to_string(),
//             ..Resource::default().into()
//         }
//     }
// }

// /// Calendar kind
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum CalendarKind {
//     /// The resource is a calendar that contains entries such as calendar events or tasks.
//     Calendar,
//     /// The resource allows for free-busy lookups, for example, to schedule group events.
//     FreeBusy,
// }

// impl From<String> for CalendarKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "calendar" => CalendarKind::Calendar,
//             "freeBusy" => CalendarKind::FreeBusy,
//             _ => panic!("Invalid CalendarKind"),
//         }
//     }
// }

// /// The scheduling addresses by which the entity may receive calendar scheduling invitations.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct SchedulingAddress {
//     /// The JSContact type of the object. The value MUST be "SchedulingAddress", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     scheduling_address_type: Option<SchedulingAddressType>,
//     /// The address to use for calendar scheduling with the contact.
//     pub uri: String,
//     /// The contexts in which to use the scheduling address.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// The preference of the scheduling address in relation to other scheduling addresses.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u64>,
//     /// A custom label for the scheduling address.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
// }

// /// SchedulingAddress @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum SchedulingAddressType {
//     /// SchedulingAddress @type
//     SchedulingAddress,
// }

// impl SchedulingAddress {
//     /// Creates a new SchedulingAddress object with the specified URI.
//     pub fn new(uri: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             scheduling_address_type: Some(SchedulingAddressType::SchedulingAddress),
//             uri: uri.to_string(),
//             contexts: None,
//             pref: None,
//             label: None,
//         }
//     }
// }

// /// The kind of the entity the Card represents.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum CardKind {
//     /// a software application
//     Application,
//     /// a device such as an appliance, a computer, or a network element
//     Device,
//     /// a group of people or entities
//     Group,
//     /// a single person
//     Individual,
//     /// a named location
//     Location,
//     /// an organization
//     Org,
// }

// impl From<String> for CardKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "application" => CardKind::Application,
//             "device" => CardKind::Device,
//             "group" => CardKind::Group,
//             "individual" => CardKind::Individual,
//             "location" => CardKind::Location,
//             "org" => CardKind::Org,
//             _ => panic!("Invalid CardKind"),
//         }
//     }
// }

// /// @Resource The cryptographic resources such as public keys and certificates associated with the entity represented by the Card.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct CryptoKey {
//     /// The @type property value MUST be "CryptoKey", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     crypto_key_type: Option<CryptoKeyType>,
//     /// The resource value.
//     pub uri: String,
//     /// The media type [RFC2046] of the resource identified by the uri property value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub media_type: Option<String>,
//     /// The kind of the resource.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub kind: Option<String>,
//     /// The contexts in which to use this resource.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// The preference of the resource in relation to other resources.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u64>,
//     /// A custom label for the value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
// }

// /// CryptoKey @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum CryptoKeyType {
//     /// CryptoKey @type
//     CryptoKey,
// }

// impl CryptoKey {
//     /// Creates a new CryptoKey object with the specified URI.
//     pub fn new(uri: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             crypto_key_type: Some(CryptoKeyType::CryptoKey),
//             uri: uri.to_string(),
//             ..Resource::default().into()
//         }
//     }
// }

// /// @Resource The directories containing information about the entity represented by the Card.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Directory {
//     /// The @type property value MUST be "Directory", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     directory_type: Option<DirectoryType>,
//     /// The kind of the directory.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub kind: Option<DirectoryKind>,
//     /// The resource value.
//     pub uri: String,
//     /// The media type [RFC2046] of the resource identified by the uri property value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub media_type: Option<String>,
//     /// The contexts in which to use this resource.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// The preference of the resource in relation to other resources.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u64>,
//     /// A custom label for the value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
//     /// The position of the directory resource in the list of all Directory objects having the same kind property value in the Card.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub list_as: Option<u64>,
// }

// /// Directory @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum DirectoryType {
//     /// Directory @type
//     Directory,
// }

// impl Directory {
//     /// Creates a new Directory object with the specified URI.
//     pub fn new(uri: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             directory_type: Some(DirectoryType::Directory),
//             uri: uri.to_string(),
//             ..Resource::default().into()
//         }
//     }
// }

// /// Directory kind
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum DirectoryKind {
//     ///  the resource is a directory service that the entity represented by the Card is a part of. This typically is an organizational directory that also contains associated entities, e.g., co-workers and management in a company directory.
//     Directory,
//     ///  the resource is a directory entry of the entity represented by the Card. In contrast to the "directory" type, this is the specific URI for the entity within a directory.
//     Entry,
// }

// impl From<String> for DirectoryKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "directory" => DirectoryKind::Directory,
//             "entry" => DirectoryKind::Entry,
//             _ => panic!("Invalid DirectoryKind"),
//         }
//     }
// }

// /// @Resource The media resources such as photographs, avatars, or sounds that are associated with the entity represented by the Card.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Media {
//     /// The @type property value MUST be "Media", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     media_hidden_type: Option<MediaType>,
//     /// The kind of the media.
//     pub kind: MediaKind,
//     /// The resource value.
//     pub uri: String,
//     /// The media type [RFC2046] of the resource identified by the uri property value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub media_type: Option<String>,
//     /// The contexts in which to use this resource.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// The preference of the resource in relation to other resources.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u64>,
//     /// A custom label for the value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
// }

// /// Media @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum MediaType {
//     /// Media @type
//     Media,
// }

// impl Media {
//     /// Creates a new Media object with the specified URI and kind.
//     /// Kind is mandatory on [`crate:Media`] struct
//     pub fn new(uri: &str, kind: MediaKind) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             media_hidden_type: Some(MediaType::Media),
//             kind,
//             uri: uri.to_string(),
//             ..Resource::default().into()
//         }
//     }
// }

// /// Media kind
// #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum MediaKind {
//     #[default]
//     /// the resource is a photograph or avatar.
//     Photo,
//     /// the resource is audio media, e.g., to specify the proper pronunciation of the name property contents.
//     Sound,
//     /// the resource is a graphic image or logo associated with the entity represented by the Card.
//     Logo,
// }

// impl From<String> for MediaKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "photo" => MediaKind::Photo,
//             "sound" => MediaKind::Sound,
//             "logo" => MediaKind::Logo,
//             _ => panic!("Invalid MediaKind"),
//         }
//     }
// }

// /// @Resource The links to resources that do not fit any of the other use-case-specific resource properties.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Link {
//     /// The @type property value MUST be "Link", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     link_type: Option<LinkType>,
//     /// The kind of the link.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub kind: Option<LinkKind>,
//     /// The resource value.
//     pub uri: String,
//     /// The media type [RFC2046] of the resource identified by the uri property value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub media_type: Option<String>,
//     /// The contexts in which to use this resource.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// The preference of the resource in relation to other resources.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u64>,
//     /// A custom label for the value.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
// }

// /// Link @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum LinkType {
//     /// Link @type
//     Link,
// }

// impl Link {
//     /// Creates a new Link object with the specified URI.
//     pub fn new(uri: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             link_type: Some(LinkType::Link),
//             uri: uri.to_string(),
//             ..Resource::default().into()
//         }
//     }
// }

// /// Link kind
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum LinkKind {
//     /// a contact link
//     Contact,
// }

// impl From<String> for LinkKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "contact" => LinkKind::Contact,
//             _ => panic!("Invalid LinkKind"),
//         }
//     }
// }

// /// Represents the Relation object for associating related Cards.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Relation {
//     /// The JSContact type of the object. Must be "Relation".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     relation_type: Option<RelationType>,
//     /// The relationship types to related Cards.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub relation: Option<HashMap<RelationshipType, bool>>,
// }

// /// the IANA-registered TYPE [IANA-vCard] parameter values of the vCard RELATED property (Section 6.6.6 of [RFC6350]):
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
// #[serde(rename_all = "lowercase")]
// pub enum RelationshipType {
//     /// acquaintance
//     Acquaintance,
//     /// agent
//     Agent,
//     /// child
//     Child,
//     /// co-resident
//     #[serde(rename = "co-resident")]
//     CoResident,
//     /// co-worker
//     #[serde(rename = "co-worker")]
//     CoWorker,
//     /// colleague
//     Colleague,
//     /// contact
//     Contact,
//     /// crush
//     Crush,
//     /// date
//     Date,
//     /// emergency
//     Emergency,
//     /// friend
//     Friend,
//     /// kin
//     Kin,
//     /// me
//     Me,
//     /// met
//     Met,
//     /// muse
//     Muse,
//     /// neighbor
//     Neighbor,
//     /// parent
//     Parent,
//     /// sibling
//     Sibling,
//     /// spouse
//     Spouse,
//     /// sweetheart
//     Sweetheart,
// }

// /// Relation @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum RelationType {
//     /// Relation @type
//     Relation,
// }

// /// Defines the Name object, which contains information about the entity's name components.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Name {
//     /// The JSContact type of the object. The value MUST be "Name", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     name_type: Option<NameType>,
//     /// Components making up the name (e.g., given name, surname).
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub components: Option<Vec<NameComponent>>,
//     /// Whether the name components are ordered.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub is_ordered: Option<bool>,
//     /// Default separator between name components.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub default_separator: Option<String>,
//     /// The full name as a single string.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub full: Option<String>,
//     /// Custom sorting order for the name components.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub sort_as: Option<HashMap<String, String>>,
//     /// The script used in the phonetic property.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub phonetic_script: Option<String>,
//     /// The phonetic system used in the phonetic property.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub phonetic_system: Option<PhoneticSystem>,
// }

// /// The phonetic system used in the related value of the phonetic property.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum PhoneticSystem {
//     /// International Phonetic Alphabet
//     Ipa,
//     /// Cantonese romanization system "Jyutping"
//     Jyut,
//     /// Standard Mandarin romanization system "Hanyu Pinyin".
//     Piny,
// }

// impl Default for Name {
//     fn default() -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             name_type: Some(NameType::Name),
//             components: None,
//             is_ordered: None,
//             default_separator: None,
//             full: None,
//             sort_as: None,
//             phonetic_script: None,
//             phonetic_system: None,
//         }
//     }
// }

// /// Name @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum NameType {
//     /// Name @type
//     Name,
// }

// /// Represents individual components of a name, such as given name or surname.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct NameComponent {
//     /// The JSContact type of the object. Must be "NameComponent".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     name_component_type: Option<NameComponentType>,
//     /// The value of the name component (e.g., "John").
//     pub value: String,
//     /// The kind of the name component (e.g., given, surname).
//     pub kind: NameComponentKind,
//     /// The phonetic representation of the name component.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub phonetic: Option<String>,
// }

// impl NameComponent {
//     /// Creates a new NameComponent object with the specified kind and value.
//     pub fn new(kind: NameComponentKind, value: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             name_component_type: Some(NameComponentType::NameComponent),
//             value: value.to_string(),
//             kind,
//             phonetic: None,
//         }
//     }
// }

// /// NameComponent @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum NameComponentType {
//     /// NameComponent @type
//     NameComponent,
// }

// /// The kind of the name component.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum NameComponentKind {
//     ///  a credential, also known as "accreditation qualifier" or "honorific suffix", e.g., "B.A.", "Esq.".
//     Credential,
//     /// a generation marker or qualifier, e.g., "Jr." or "III".
//     Generation,
//     /// a given name, also known as "first name" or "personal name".
//     Given,
//     /// a name that appears between the given and surname such as a middle name or patronymic name.
//     Given2,
//     /// a formatting separator between two ordered name non-separator components. The value property of the component includes the verbatim separator, for example, a hyphen character or even an empty string.
//     /// This value has higher precedence than the defaultSeparator property of the Name.
//     Separator,
//     ///  a surname, also known as "last name" or "family name".
//     Surname,
//     /// a secondary surname (used in some cultures), also known as "maternal surname".
//     Surname2,
//     /// an honorific title or prefix, e.g., "Mr.", "Ms.", or "Dr.".
//     Title,
// }

// impl From<String> for NameComponentKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "credential" => NameComponentKind::Credential,
//             "generation" => NameComponentKind::Generation,
//             "given" => NameComponentKind::Given,
//             "given2" => NameComponentKind::Given2,
//             "separator" => NameComponentKind::Separator,
//             "surname" => NameComponentKind::Surname,
//             "surname2" => NameComponentKind::Surname2,
//             "title" => NameComponentKind::Title,
//             _ => panic!("Invalid NameComponentKind"),
//         }
//     }
// }

// /// Defines the Nickname object, which includes nicknames for the entity.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Nickname {
//     /// The JSContact type of the object. Must be "Nickname".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     nickname_type: Option<NicknameType>,
//     /// The nickname value.
//     pub name: String,
//     /// Contexts in which to use the nickname.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// Preference of the nickname relative to others.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u32>,
// }

// /// Nickname @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum NicknameType {
//     /// Nickname @type
//     Nickname,
// }

// /// Represents an Organization object containing company or organization information.
// #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Organization {
//     /// The JSContact type of the object. Must be "Organization".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     org_type: Option<OrganizationType>,
//     /// The name of the organization.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
//     /// Organizational units within the organization.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub units: Option<Vec<OrgUnit>>,
//     /// Custom sorting order for the organization.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub sort_as: Option<String>,
//     /// Contexts in which the organization is relevant.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
// }

// /// Organization @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum OrganizationType {
//     /// Organization @type
//     Organization,
// }

// /// Represents a unit within an organization, such as a department.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct OrgUnit {
//     /// The JSContact type of the object. Must be "OrgUnit".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     unit_type: Option<OrgUnitType>,
//     /// The name of the organizational unit.
//     pub name: String,
//     /// Custom sorting order for the organizational unit.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub sort_as: Option<String>,
// }

// /// OrgUnit @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum OrgUnitType {
//     /// OrgUnit @type
//     OrgUnit,
// }

// impl OrgUnit {
//     /// Creates a new OrgUnit object with the specified name.
//     pub fn new(name: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             unit_type: Some(OrgUnitType::OrgUnit),
//             name: name.to_string(),
//             sort_as: None,
//         }
//     }
// }

// /// Represents how to address or refer to the entity, including grammatical gender and pronouns.
// #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct SpeakToAs {
//     /// The JSContact type of the object. Must be "SpeakToAs".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     speak_to_as_type: Option<SpeakToAsType>,
//     /// Grammatical gender to use in salutations.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub grammatical_gender: Option<GrammaticalGender>,
//     /// Pronouns associated with the entity.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pronouns: Option<HashMap<String, Pronouns>>,
// }

// /// The grammatical gender to use in salutations and other grammatical constructs.
// /// For example, the German language distinguishes by grammatical gender in salutations such as "Sehr geehrte" (feminine) and "Sehr geehrter" (masculine).
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum GrammaticalGender {
//     /// animate
//     Animate,
//     /// common
//     Common,
//     /// feminine
//     Feminine,
//     /// inanimate
//     Inanimate,
//     /// masculine
//     Masculine,
//     /// neuter
//     Neuter,
// }

// /// SpeakToAs @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum SpeakToAsType {
//     /// SpeakToAs @type
//     SpeakToAs,
// }

// /// Defines pronouns used for the entity, such as they/them.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Pronouns {
//     /// The JSContact type of the object. Must be "Pronouns".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     pronoun_type: Option<PronounsType>,
//     /// The pronouns value (e.g., "they/them").
//     pub pronouns: String,
//     /// Contexts in which to use the pronouns.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// Preference of the pronouns relative to others.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u32>,
// }

// /// Pronouns @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum PronounsType {
//     /// Pronouns @type
//     Pronouns,
// }

// impl Pronouns {
//     /// Creates a new Pronouns object with the specified pronouns.
//     pub fn new(pronouns: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             pronoun_type: Some(PronounsType::Pronouns),
//             pronouns: pronouns.to_string(),
//             contexts: None,
//             pref: None,
//         }
//     }
// }

// /// Represents titles or roles of the entity, such as job titles or functional positions.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Title {
//     /// The JSContact type of the object. Must be "Title".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     title_type: Option<TitleType>,
//     /// The title or role name.
//     pub name: String,
//     /// The kind of title (e.g., title, role).
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub kind: Option<TitleKind>,
//     /// Identifier of the organization associated with this title.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub organization_id: Option<String>,
// }

// /// Title @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum TitleType {
//     /// Title @type
//     Title,
// }

// impl Title {
//     /// Creates a new Title object with the specified name.
//     pub fn new(name: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             title_type: Some(TitleType::Title),
//             name: name.to_string(),
//             kind: None,
//             organization_id: None,
//         }
//     }
// }

// /// Title kind
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum TitleKind {
//     /// a role
//     Role,
//     /// a title
//     Title,
// }

// impl From<String> for TitleKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "role" => TitleKind::Role,
//             "title" => TitleKind::Title,
//             _ => panic!("Invalid TitleKind"),
//         }
//     }
// }

// /// Defines email addresses associated with the entity.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct EmailAddress {
//     /// The JSContact type of the object. Must be "EmailAddress".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     email_type: Option<EmailAddressType>,
//     /// The email address.
//     pub address: String,
//     /// Contexts in which to use the email address.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// Preference of the email address relative to others.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u32>,
//     /// Custom label for the email address.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
// }

// /// EmailAddress @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum EmailAddressType {
//     /// EmailAddress @type
//     EmailAddress,
// }

// impl EmailAddress {
//     /// Creates a new EmailAddress object with the specified email address.
//     pub fn new(address: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             email_type: Some(EmailAddressType::EmailAddress),
//             address: address.to_string(),
//             contexts: None,
//             pref: None,
//             label: None,
//         }
//     }
// }

// /// Represents online services such as social media or messaging accounts.
// #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct OnlineService {
//     /// The JSContact type of the object. Must be "OnlineService".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     service_type: Option<OnlineServiceType>,
//     /// The name of the online service or protocol.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub service: Option<String>,
//     /// The URI identifying the entity on the service.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub uri: Option<String>,
//     /// The username or handle on the online service.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub user: Option<String>,
//     /// Contexts in which to use the online service.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// Preference of the service relative to others.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u32>,
//     /// Custom label for the online service.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
// }

// /// OnlineService @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum OnlineServiceType {
//     /// OnlineService @type
//     OnlineService,
// }

// /// Defines phone numbers for the entity, including features like voice or text.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Phone {
//     /// The JSContact type of the object. Must be "Phone".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     phone_type: Option<PhoneType>,
//     /// The phone number, either as a URI or free text.
//     pub number: String,
//     /// Contact features the phone number supports
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub features: Option<HashMap<PhoneFeature, bool>>,
//     /// Contexts in which to use the phone number.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// Preference of the phone number relative to others.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u32>,
//     /// Custom label for the phone number.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
// }

// /// The set of contact features that the phone number may be used for.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum PhoneFeature {
//     /// this number supports sending faxes.
//     Fax,
//     /// this number is a main phone number such as the number of the front desk at a company, as opposed to a direct-dial number of an individual employee.
//     #[serde(rename = "main-number")]
//     MainNumber,
//     /// this number is for a mobile phone.
//     Mobile,
//     ///  this number is for a pager or beeper.
//     Pager,
//     /// this number supports text messages (SMS).
//     Text,
//     /// this number is for a device for people with hearing or speech difficulties.
//     Textphone,
//     /// this number supports video conferencing.
//     Video,
//     ///  this number supports calling by voice.
//     Voice,
// }

// /// The contexts in which to use the contact information.
// /// For example, someone might have distinct phone numbers for work and private contexts and may set the desired context on the respective phone number in the phones property.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
// #[serde(rename_all = "lowercase")]
// pub enum Context {
//     /// the contact information that may be used in a private context.
//     Private,
//     /// the contact information that may be used in a professional context.
//     Work,
// }

// /// Phone @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum PhoneType {
//     /// Phone @type
//     Phone,
// }

// impl Phone {
//     /// Creates a new Phone object with the specified phone number.
//     pub fn new(number: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             phone_type: Some(PhoneType::Phone),
//             number: number.to_string(),
//             features: None,
//             contexts: None,
//             pref: None,
//             label: None,
//         }
//     }
// }

// /// Represents preferred languages for communication.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct LanguagePref {
//     /// The JSContact type of the object. Must be "LanguagePref".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     lang_pref_type: Option<LanguagePrefType>,
//     /// The preferred language as a language tag (e.g., en, fr).
//     pub language: String,
//     /// Contexts in which to use the preferred language.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<Context, bool>>,
//     /// Preference of the language relative to others.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u32>,
// }

// /// LanguagePref @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum LanguagePrefType {
//     /// LanguagePref @type
//     LanguagePref,
// }

// impl LanguagePref {
//     /// Creates a new LanguagePref object with the specified language.
//     pub fn new(language: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             lang_pref_type: Some(LanguagePrefType::LanguagePref),
//             language: language.to_string(),
//             contexts: None,
//             pref: None,
//         }
//     }
// }

// /// Represents memorable dates and events for the entity.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Anniversary {
//     /// The JSContact type of the object. Must be "Anniversary".
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     anniversary_type: Option<AnniversaryType>,
//     /// The date of the anniversary.
//     pub date: DateObject,
//     /// The kind of anniversary
//     pub kind: AnniversaryKind,
//     /// Contexts in which to use the anniversary.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<String, bool>>,
//     /// Preference of the anniversary relative to others.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub place: Option<Address>,
// }

// /// The kind of anniversary
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum AnniversaryKind {
//     /// a birthday anniversary
//     Birth,
//     /// a deathday anniversary
//     Death,
//     /// a wedding day anniversary
//     Wedding,
// }

// impl From<String> for AnniversaryKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "birth" => AnniversaryKind::Birth,
//             "death" => AnniversaryKind::Death,
//             "wedding" => AnniversaryKind::Wedding,
//             _ => panic!("Invalid AnniversaryKind"),
//         }
//     }
// }

// /// Anniversary @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum AnniversaryType {
//     /// Anniversary @type
//     Anniversary,
// }

// impl Anniversary {
//     /// Creates a new Anniversary object with the specified date and kind.
//     pub fn new(kind: AnniversaryKind, date: DateObject) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             anniversary_type: Some(AnniversaryType::Anniversary),
//             date,
//             kind,
//             contexts: None,
//             place: None,
//         }
//     }
// }

// /// Represents a date object, which can be a timestamp or a partial date.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(untagged)]
// pub enum DateObject {
//     // Check first if the date is a timestamp because timestamp has a field
//     /// Timestamp
//     Timestamp(Timestamp),
//     /// PartialDate
//     PartialDate(PartialDate),
// }

// /// Timestamp
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Timestamp {
//     /// The JSContact type of the object. The value MUST be "Timestamp", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     timestamp_type: Option<TimestampType>,

//     /// The point in time in UTC time
//     pub utc: String,
// }

// /// Timestamp @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum TimestampType {
//     /// Timestamp @type
//     Timestamp,
// }

// impl Timestamp {
//     /// Creates a new Timestamp object with the specified UTC time.
//     pub fn new(utc: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             timestamp_type: Some(TimestampType::Timestamp),
//             utc: utc.to_string(),
//         }
//     }
// }

// /// A PartialDate object represents a complete or partial calendar date in the Gregorian calendar.  It represents a complete date, a year, a month in a year, or a day in a month.
// #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct PartialDate {
//     /// The JSContact type of the object. The value MUST be "PartialDate", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     partial_date_type: Option<PartialDateType>,
//     /// The calendar year.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub year: Option<u64>,
//     /// The calendar month, represented as the integers 1 <= month <= 12. If this property is set, then either the year or the day property MUST be set.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub month: Option<u32>,
//     /// The calendar month day, represented as the integers 1 <= day <= 31, depending on the validity within the month and year. If this property is set, then the month property MUST be set.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub day: Option<u32>,

//     /// The calendar system in which this date occurs, in lowercase.  This MUST be either a calendar system name registered as a Common Locale Data Repository (CLDR) [RFC7529] or a vendor-specific value.
//     /// The year, month, and day still MUST be represented in the Gregorian calendar.
//     /// Note that the year property might be required to convert the date between the Gregorian calendar and the respective calendar system.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub calendar_scale: Option<String>,
// }

// /// PartialDate @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum PartialDateType {
//     /// PartialDate @type
//     PartialDate,
// }

// /// The addresses of the entity represented by the Card, such as postal addresses or geographic locations.
// #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Address {
//     /// The JSContact type of the object. The value MUST be "Address", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     address_type: Option<AddressType>,
//     /// The components that make up the address.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub components: Option<Vec<AddressComponent>>,
//     /// The indicator if the address components in the components property are ordered.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub is_ordered: Option<bool>,
//     /// The Alpha-2 country code [ISO.3166-1].
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub country_code: Option<String>,
//     /// A "geo:" URI [RFC5870] for the address.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub coordinates: Option<String>,
//     /// The time zone in which the address is located. This MUST be a time zone name registered in the IANA Time Zone Database [IANA-TZ].
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub time_zone: Option<String>,
//     /// The contexts in which to use this address.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contexts: Option<HashMap<AddressContext, bool>>,
//     /// The full address, including street, region, or country. The purpose of this property is to define an address, even if the individual address components are not known.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub full: Option<String>,
//     /// The default separator to insert between address component values when concatenating all address component values to a single String.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub default_separator: Option<String>,
//     /// The preference of the address in relation to other addresses.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pref: Option<u64>,
//     /// The script used in the value of the AddressComponent phonetic property.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub phonetic_script: Option<String>,
//     /// The phonetic system used in the AddressComponent phonetic property.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub phonetic_system: Option<PhoneticSystem>,
// }

// /// The contexts in which to use this address.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
// #[serde(rename_all = "lowercase")]
// pub enum AddressContext {
//     /// an address to be used for billing.
//     Billing,
//     /// an address to be used for delivering physical items.
//     Delivery,
//     /// may be used in a private context.
//     Private,
//     /// may be used in a professional context.
//     Work,
// }

// /// Address @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum AddressType {
//     /// Address @type
//     Address,
// }

// /// The components that make up the address.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct AddressComponent {
//     /// The JSContact type of the object. The value MUST be "AddressComponent", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     component_type: Option<AddressComponentType>,
//     /// The value of the address component.
//     pub value: String,
//     /// The kind of the address component.
//     pub kind: AddressComponentKind,
//     /// The pronunciation of the name component. If this property is set, then at least one of the Address object phoneticSystem or phoneticScript properties MUST be set.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub phonetic: Option<String>,
// }

// /// AddressComponent @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum AddressComponentType {
//     /// AddressComponent @type
//     AddressComponent,
// }

// impl AddressComponent {
//     /// Creates a new AddressComponent object with the specified kind and value.
//     pub fn new(kind: AddressComponentKind, value: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             component_type: Some(AddressComponentType::AddressComponent),
//             value: value.to_string(),
//             kind,
//             phonetic: None,
//         }
//     }
// }

// /// The kind of the address component.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum AddressComponentKind {
//     /// the extension designation such as the apartment number, unit, or box number.
//     Apartment,
//     /// the block name or number.
//     Block,
//     /// the building, tower, or condominium the address is located in.
//     Building,
//     /// the country name.
//     Country,
//     /// the cardinal direction or quadrant, e.g., "north".
//     Direction,
//     ///  the district name.
//     District,
//     /// the floor or level the address is located on.
//     Floor,
//     /// the publicly known prominent feature that can substitute the street name and number, e.g., "White House" or "Taj Mahal".
//     Landmark,
//     /// the municipality, city, town, village, post town, or other locality.
//     Locality,
//     /// the street name.
//     Name,
//     ///  the street number, e.g., "123". This value is not restricted to numeric values and can include any value such as number ranges ("112-10"), grid style ("39.2 RD"), alphanumerics ("N6W23001"), or fractionals ("123 1/2").
//     Number,
//     /// the postal code, post code, ZIP code, or other short code associated with the address by the relevant country's postal system.
//     Postcode,
//     ///  the post office box number or identifier.
//     #[serde(rename = "postOfficeBox")]
//     PostOfficeBox,
//     /// the administrative area such as province, state, prefecture, county, or canton.
//     Region,
//     /// the room, suite number, or identifier.
//     Room,
//     /// a formatting separator between two ordered address non-separator components. The value property of the component includes the verbatim separator, for example, a hyphen character or even an empty string. This value has higher precedence than the defaultSeparator property of the Address.
//     Separator,
//     ///  the subdistrict, ward, or other subunit of a district.
//     Subdistrict,
// }

// impl From<String> for AddressComponentKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "apartment" => AddressComponentKind::Apartment,
//             "block" => AddressComponentKind::Block,
//             "building" => AddressComponentKind::Building,
//             "country" => AddressComponentKind::Country,
//             "direction" => AddressComponentKind::Direction,
//             "district" => AddressComponentKind::District,
//             "floor" => AddressComponentKind::Floor,
//             "landmark" => AddressComponentKind::Landmark,
//             "locality" => AddressComponentKind::Locality,
//             "name" => AddressComponentKind::Name,
//             "number" => AddressComponentKind::Number,
//             "postcode" => AddressComponentKind::Postcode,
//             "postOfficeBox" => AddressComponentKind::PostOfficeBox,
//             "region" => AddressComponentKind::Region,
//             "room" => AddressComponentKind::Room,
//             "separator" => AddressComponentKind::Separator,
//             "subdistrict" => AddressComponentKind::Subdistrict,
//             _ => panic!("Invalid AddressComponentKind"),
//         }
//     }
// }

// /// The free-text notes that are associated with the Card.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Note {
//     /// The JSContact type of the object. The value MUST be "Note", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     note_type: Option<NoteType>,
//     /// The free-text value of this note.
//     pub note: String,
//     /// The date and time when this note was created.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub created: Option<String>,
//     /// The author of this note.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub author: Option<Author>,
// }

// /// Note @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum NoteType {
//     /// Note @type
//     Note,
// }

// /// The author of a note.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Author {
//     /// The JSContact type of the object. The value MUST be "Author", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     author_type: Option<AuthorType>,
//     /// The name of this author.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
//     /// The URI value that identifies the author.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub uri: Option<String>,
// }

// /// Author @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum AuthorType {
//     /// Author @type
//     Author,
// }

// /// The personal information of the entity represented by the Card.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct PersonalInfo {
//     ///The JSContact type of the object.  The value MUST be "PersonalInfo", if set.
//     #[cfg(feature = "typed")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "@type")]
//     personal_info_type: Option<PersonalInfoType>,
//     /// The kind of personal information.
//     pub kind: PersonalInfoKind,
//     /// The actual information.
//     pub value: String,
//     /// The level of expertise or engagement in hobby or interest.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub level: Option<PersonalInfoLevel>,
//     /// The position of the personal information in the list of all PersonalInfo objects that have the same kind property value in the Card.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub list_as: Option<u64>,
//     /// A custom label.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub label: Option<String>,
// }

// /// The kind of personal information.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum PersonalInfoKind {
//     /// a field of expertise or a credential
//     Expertise,
//     /// a hobby
//     Hobby,
//     /// an interest
//     Interest,
// }

// impl From<String> for PersonalInfoKind {
//     fn from(kind: String) -> Self {
//         match kind.as_str() {
//             "expertise" => PersonalInfoKind::Expertise,
//             "hobby" => PersonalInfoKind::Hobby,
//             "interest" => PersonalInfoKind::Interest,
//             _ => panic!("Invalid PersonalInfoKind"),
//         }
//     }
// }

// /// PersonalInfo @type
// #[cfg(feature = "typed")]
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// enum PersonalInfoType {
//     /// PersonalInfo @type
//     PersonalInfo,
// }

// impl PersonalInfo {
//     /// Creates a new PersonalInfo object with the specified kind and value.
//     pub fn new(kind: PersonalInfoKind, value: &str) -> Self {
//         Self {
//             #[cfg(feature = "typed")]
//             personal_info_type: Some(PersonalInfoType::PersonalInfo),
//             kind,
//             value: value.to_string(),
//             level: None,
//             list_as: None,
//             label: None,
//         }
//     }
// }

// /// The level of expertise or engagement in hobby or interest.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum PersonalInfoLevel {
//     /// High level of expertise or engagement.
//     High,
//     /// Medium level of expertise or engagement.
//     Medium,
//     /// Low level of expertise or engagement.
//     Low,
// }
