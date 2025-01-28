//! The primary Card object as defined in RFC 9553

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Address, AddressComponent, AddressComponentKind, Anniversary, Calendar, CardKind, CardVersion,
    CryptoKey, Directory, EmailAddress, LanguagePref, Link, Media, Name, NameComponent, Nickname,
    Note, OnlineService, Organization, PersonalInfo, Phone, Relation, SchedulingAddress, SpeakToAs,
    Title,
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

/// Localize the [`crate::Name`]
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

/// Localize the [`crate::Titles`]
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

/// Localize the [`crate::Addresses`]
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
                AddressComponentKind::Apartment,
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
            let Ok(kind) = serde_json::from_value::<AddressComponentKind>(value.clone()) else {
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

/// Localize the [`crate::Nicknames`]
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

/// Localize the [`crate::PersonalInfos`]
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

/// Localize the [`crate::Notes`]
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

/// Localize the [`crate::Keywords`]
fn localize_keywords(card: &mut Card, key: &str, value: &Value) -> Result<(), String> {
    if key == "keywords" {
        card.keywords = serde_json::from_value(value.clone()).ok();
        return Ok(());
    }
    Ok(())
}
