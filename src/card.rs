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
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    card_type: String,
    /// The JSContact version of this Card.
    pub version: CardVersion,
    /// The date and time when the Card was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// A unique identifier for the Card.
    pub uid: String,
    /// The kind of entity the Card represents (e.g., individual, group).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CardKind>,
    /// The language used in the Card (e.g., en, fr).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Members of a group Card, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<HashMap<String, bool>>,
    /// Identifier for the product that created the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prod_id: Option<String>,
    /// Related Cards with their relationship    types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_to: Option<HashMap<String, Relation>>,
    /// The last modification time of the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// The name of the entity represented by the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /// Nicknames of the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nicknames: Option<HashMap<String, Nickname>>,
    /// Organizations associated with the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<HashMap<String, Organization>>,
    /// How to address or refer to the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speak_to_as: Option<SpeakToAs>,
    /// Job titles or roles of the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<HashMap<String, Title>>,
    /// Email addresses for contacting the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<HashMap<String, EmailAddress>>,
    /// Online services or social media associated with the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_services: Option<HashMap<String, OnlineService>>,
    /// Phone numbers for contacting the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<HashMap<String, Phone>>,
    /// Preferred languages for communication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_languages: Option<HashMap<String, LanguagePref>>,
    /// The calendaring resources of the entity represented by the Card, such as to look up free-busy information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendars: Option<HashMap<String, Calendar>>,
    /// The scheduling addresses by which the entity may receive calendar scheduling invitations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_addresses: Option<HashMap<String, SchedulingAddress>>,
    /// Localizations provide language-specific alternatives for existing property values and SHOULD NOT add new properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    localizations: Option<HashMap<String, HashMap<String, Value>>>,
    /// The memorable dates and events for the entity represented by the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anniversaries: Option<HashMap<String, Anniversary>>,
    /// The scheduling addresses by which the entity may receive calendar scheduling invitations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<HashMap<String, Address>>,
    /// The cryptographic resources such as public keys and certificates associated with the entity represented by the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_keys: Option<HashMap<String, CryptoKey>>,
    /// The directories containing information about the entity represented by the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<HashMap<String, Directory>>,
    /// The links to resources that do not fit any of the other use-case-specific resource properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, Link>>,
    /// The media resources such as photographs, avatars, or sounds that are associated with the entity represented by the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<HashMap<String, Media>>,
    /// The set of free-text keywords, also known as tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<HashMap<String, bool>>,
    /// The free-text notes that are associated with the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<HashMap<String, Note>>,
    /// The personal information of the entity represented by the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_info: Option<HashMap<String, PersonalInfo>>,
}

impl Card {
    /// Creates a new Card object with the specified version and unique identifier.
    pub fn new(version: CardVersion, uid: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
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
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
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
                if key == "name" {
                    card.name = serde_json::from_value(value.clone()).ok();
                } else {
                    let curr_name = match &mut card.name {
                        Some(name) => name,
                        None => &mut Name::default(),
                    };
                    let key = key.replace("name/", "");
                    if key.starts_with("components") {
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
                }
            } else if key.starts_with("titles") {
                let titles = match &mut card.titles {
                    Some(titles) => titles,
                    None => &mut HashMap::new(),
                };
                let key = key.replace("titles", "");
                let key = if key.is_empty() {
                    let Ok(titles_map) =
                        serde_json::from_value::<HashMap<String, Title>>(value.clone())
                    else {
                        return Err("Invalid value".into());
                    };
                    *titles = titles_map;
                    card.titles = Some(titles.clone());
                    continue;
                } else {
                    remove_first(&key)
                };
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
                    continue;
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
            } else if key.starts_with("addresses") {
                let addresses = match &mut card.addresses {
                    Some(addresses) => addresses,
                    None => &mut HashMap::new(),
                };
                let key = key.replace("addresses", "");
                let key = if key.is_empty() {
                    let Ok(addr) =
                        serde_json::from_value::<HashMap<String, Address>>(value.clone())
                    else {
                        return Err("Invalid value".into());
                    };
                    *addresses = addr;
                    card.addresses = Some(addresses.clone());
                    continue;
                } else {
                    remove_first(&key)
                };
                let keys = key.split("/").collect::<Vec<&str>>();
                let Some(idx_key) = keys.first() else {
                    return Err("Invalid addresses key".into());
                };
                let idx_key = idx_key.to_string();
                let key = key.replace(&idx_key, "");
                let Some(address) = addresses.get_mut(&idx_key) else {
                    return Err(format!("addresses key '{}' not found", idx_key));
                };
                if key.starts_with("components") {
                    if key == "components" {
                        address.components = serde_json::from_value(value.clone()).ok();
                    } else {
                        let components = match &mut address.components {
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
                        let component: &mut AddressComponent = &mut components[idx];
                        if key == "value" {
                            let Ok(str) = serde_json::from_value::<String>(value.clone()) else {
                                return Err("Invalid value".into());
                            };
                            component.value = str;
                        } else if key == "phonetic" {
                            component.phonetic = serde_json::from_value(value.clone()).ok();
                        }
                        address.components = Some(components.clone());
                    }
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
                    println!("key is empty");
                    let Ok(addr) = serde_json::from_value::<Address>(value.clone()) else {
                        return Err("Invalid value".into());
                    };
                    addresses.insert(idx_key, addr);
                }
            } else if key.starts_with("nicknames") {
                if key == "nicknames" {
                    card.nicknames = serde_json::from_value(value.clone()).ok();
                } else {
                    let nicknames = match &mut card.nicknames {
                        Some(nicknames) => nicknames,
                        None => &mut HashMap::new(),
                    };
                    let key = key.replace("nicknames", "");
                    let key = if key.is_empty() {
                        let Ok(nicks) =
                            serde_json::from_value::<HashMap<String, Nickname>>(value.clone())
                        else {
                            return Err("Invalid value".into());
                        };
                        *nicknames = nicks;
                        card.nicknames = Some(nicknames.clone());
                        continue;
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
                        continue;
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
                }
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
