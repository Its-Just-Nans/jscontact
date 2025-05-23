//! # JSContact
//! This crates implements types for the JSContact format as defined in RFC 9553.
//!
//! To start using this crate, run the following command in your project directory:
//! ```bash
//! cargo add jscontact
//! ```
//!
//! Simple deserialization example:
//! ```rust
//! use jscontact::Card;
//! use serde_json;
//!
//! let json_value = serde_json::json!({
//!     "@type": "Card",
//!     "version": "1.0",
//!     "uid": "1234"
//! });
//! let card_deserialized: Card = Card::try_from(json_value).unwrap(); // deserialize from serde::Value
//! let card_string: String = String::try_from(card_deserialized.clone()).unwrap(); // serialize to String
//! let card: Card = card_string.parse().unwrap(); // deserialize with parse()
//! assert_eq!(card_deserialized, card);
//! ```
//!
//! Simple creation example:
//! ```rust
//! use jscontact::{Card, CardKind, CardVersion, Name, NameComponent, NameComponentKind};
//! use serde_json;
//!
//! let mut card = Card::new(CardVersion::OneDotZero, "my:uri");
//! card.kind = Some(CardKind::Individual);
//! let json = serde_json::to_string(&card).unwrap(); // serialize with serde
//! ```
//!
//! Get localized Card:
//! ```rust
//! use jscontact::{Card, CardVersion, Name};
//! use std::collections::HashMap;
//! use serde_json::Value;
//!
//! // create a card
//! let mut card = Card::new(CardVersion::OneDotZero, "my:uri");
//! let mut name = Name::default();
//! name.full = Some("John".to_string());
//! card.name = Some(name);
//!
//! // add localization
//! let mut translations: HashMap<String, Value> = HashMap::new();
//! let mut name_en = Name::default();
//! name_en.full = Some("Johny".to_string());
//! translations.insert(
//!     "name".to_string(),
//!     serde_json::to_value(name_en).expect("Failed to serialize name"),
//! );
//! card.add_localization("en", translations);
//!
//! // use localized card
//! let langs = card.get_available_languages();
//! assert_eq!(langs, vec!["en"]);
//! let localized = card.get_localized(&langs[0]).unwrap();
//! assert_eq!(localized.name.unwrap().full.unwrap(), "Johny");
//! ```

#![deny(
    missing_docs,
    clippy::all,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cargo
)]
#![warn(clippy::multiple_crate_versions)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod card;
pub use card::Card;

mod resource;
pub use resource::Resource;

/// Represents the card version.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CardVersion {
    /// version 1.0
    #[serde(rename = "1.0")]
    OneDotZero,
}

/// [`crate::Resource`] The calendaring resources of the entity represented by the Card, such as to look up free-busy information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    /// The @type property value MUST be "Calendar", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    calendar_type: Option<CalendarType>,
    /// The kind of the calendar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CalendarKind>,
    /// The media type RFC2046 of the resource identified by the uri property value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The resource value.
    pub uri: String,
    /// The contexts in which to use this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// The preference of the resource in relation to other resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u64>,
    /// A custom label for the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Calendar @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum CalendarType {
    /// Calendar @type
    Calendar,
}

impl Calendar {
    /// Creates a new Calendar object with the specified URI.
    pub fn new(uri: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            calendar_type: Some(CalendarType::Calendar),
            uri: uri.to_string(),
            ..Resource::default().into()
        }
    }
}

/// Calendar kind
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CalendarKind {
    /// The resource is a calendar that contains entries such as calendar events or tasks.
    Calendar,
    /// The resource allows for free-busy lookups, for example, to schedule group events.
    FreeBusy,
}

impl From<String> for CalendarKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "calendar" => CalendarKind::Calendar,
            "freeBusy" => CalendarKind::FreeBusy,
            _ => panic!("Invalid CalendarKind"),
        }
    }
}

/// The scheduling addresses by which the entity may receive calendar scheduling invitations.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchedulingAddress {
    /// The JSContact type of the object. The value MUST be "SchedulingAddress", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    scheduling_address_type: Option<SchedulingAddressType>,
    /// The address to use for calendar scheduling with the contact.
    pub uri: String,
    /// The contexts in which to use the scheduling address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// The preference of the scheduling address in relation to other scheduling addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u64>,
    /// A custom label for the scheduling address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// SchedulingAddress @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum SchedulingAddressType {
    /// SchedulingAddress @type
    SchedulingAddress,
}

impl SchedulingAddress {
    /// Creates a new SchedulingAddress object with the specified URI.
    pub fn new(uri: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            scheduling_address_type: Some(SchedulingAddressType::SchedulingAddress),
            uri: uri.to_string(),
            contexts: None,
            pref: None,
            label: None,
        }
    }
}

/// The kind of the entity the Card represents.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CardKind {
    /// a software application
    Application,
    /// a device such as an appliance, a computer, or a network element
    Device,
    /// a group of people or entities
    Group,
    /// a single person
    Individual,
    /// a named location
    Location,
    /// an organization
    Org,
}

impl From<String> for CardKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "application" => CardKind::Application,
            "device" => CardKind::Device,
            "group" => CardKind::Group,
            "individual" => CardKind::Individual,
            "location" => CardKind::Location,
            "org" => CardKind::Org,
            _ => panic!("Invalid CardKind"),
        }
    }
}

/// [`crate::Resource`] The cryptographic resources such as public keys and certificates associated with the entity represented by the Card.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CryptoKey {
    /// The @type property value MUST be "CryptoKey", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    crypto_key_type: Option<CryptoKeyType>,
    /// The resource value.
    pub uri: String,
    /// The media type RFC2046 of the resource identified by the uri property value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The kind of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The contexts in which to use this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// The preference of the resource in relation to other resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u64>,
    /// A custom label for the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// CryptoKey @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum CryptoKeyType {
    /// CryptoKey @type
    CryptoKey,
}

impl CryptoKey {
    /// Creates a new CryptoKey object with the specified URI.
    pub fn new(uri: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            crypto_key_type: Some(CryptoKeyType::CryptoKey),
            uri: uri.to_string(),
            ..Resource::default().into()
        }
    }
}

/// [`crate::Resource`] The directories containing information about the entity represented by the Card.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    /// The @type property value MUST be "Directory", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    directory_type: Option<DirectoryType>,
    /// The kind of the directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<DirectoryKind>,
    /// The resource value.
    pub uri: String,
    /// The media type RFC2046 of the resource identified by the uri property value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The contexts in which to use this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// The preference of the resource in relation to other resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u64>,
    /// A custom label for the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The position of the directory resource in the list of all Directory objects having the same kind property value in the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_as: Option<u64>,
}

/// Directory @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum DirectoryType {
    /// Directory @type
    Directory,
}

impl Directory {
    /// Creates a new Directory object with the specified URI.
    pub fn new(uri: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            directory_type: Some(DirectoryType::Directory),
            uri: uri.to_string(),
            ..Resource::default().into()
        }
    }
}

/// Directory kind
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DirectoryKind {
    ///  the resource is a directory service that the entity represented by the Card is a part of. This typically is an organizational directory that also contains associated entities, e.g., co-workers and management in a company directory.
    Directory,
    ///  the resource is a directory entry of the entity represented by the Card. In contrast to the "directory" type, this is the specific URI for the entity within a directory.
    Entry,
}

impl From<String> for DirectoryKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "directory" => DirectoryKind::Directory,
            "entry" => DirectoryKind::Entry,
            _ => panic!("Invalid DirectoryKind"),
        }
    }
}

/// [`crate::Resource`] The media resources such as photographs, avatars, or sounds that are associated with the entity represented by the Card.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    /// The @type property value MUST be "Media", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    media_hidden_type: Option<MediaType>,
    /// The kind of the media.
    pub kind: MediaKind,
    /// The resource value.
    pub uri: String,
    /// The media type RFC2046 of the resource identified by the uri property value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The contexts in which to use this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// The preference of the resource in relation to other resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u64>,
    /// A custom label for the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Media @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum MediaType {
    /// Media @type
    Media,
}

impl Media {
    /// Creates a new Media object with the specified URI and kind.
    /// Kind is mandatory on [`crate::Media`] struct
    pub fn new(uri: &str, kind: MediaKind) -> Self {
        Self {
            #[cfg(feature = "typed")]
            media_hidden_type: Some(MediaType::Media),
            kind,
            uri: uri.to_string(),
            ..Resource::default().into()
        }
    }
}

/// Media kind
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MediaKind {
    #[default]
    /// the resource is a photograph or avatar.
    Photo,
    /// the resource is audio media, e.g., to specify the proper pronunciation of the name property contents.
    Sound,
    /// the resource is a graphic image or logo associated with the entity represented by the Card.
    Logo,
}

impl From<String> for MediaKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "photo" => MediaKind::Photo,
            "sound" => MediaKind::Sound,
            "logo" => MediaKind::Logo,
            _ => panic!("Invalid MediaKind"),
        }
    }
}

/// [`crate::Resource`] The links to resources that do not fit any of the other use-case-specific resource properties.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// The @type property value MUST be "Link", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    link_type: Option<LinkType>,
    /// The kind of the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<LinkKind>,
    /// The resource value.
    pub uri: String,
    /// The media type RFC2046 of the resource identified by the uri property value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The contexts in which to use this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// The preference of the resource in relation to other resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u64>,
    /// A custom label for the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Link @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum LinkType {
    /// Link @type
    Link,
}

impl Link {
    /// Creates a new Link object with the specified URI.
    pub fn new(uri: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            link_type: Some(LinkType::Link),
            uri: uri.to_string(),
            ..Resource::default().into()
        }
    }
}

/// Link kind
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LinkKind {
    /// a contact link
    Contact,
}

impl From<String> for LinkKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "contact" => LinkKind::Contact,
            _ => panic!("Invalid LinkKind"),
        }
    }
}

/// Represents the Relation object for associating related Cards.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    /// The JSContact type of the object. Must be "Relation".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    relation_type: Option<RelationType>,
    /// The relationship types to related Cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<HashMap<RelationshipType, bool>>,
}

/// the IANA-registered TYPE [IANA-vCard] parameter values of the vCard RELATED property (Section 6.6.6 of RFC6350):
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipType {
    /// acquaintance
    Acquaintance,
    /// agent
    Agent,
    /// child
    Child,
    /// co-resident
    #[serde(rename = "co-resident")]
    CoResident,
    /// co-worker
    #[serde(rename = "co-worker")]
    CoWorker,
    /// colleague
    Colleague,
    /// contact
    Contact,
    /// crush
    Crush,
    /// date
    Date,
    /// emergency
    Emergency,
    /// friend
    Friend,
    /// kin
    Kin,
    /// me
    Me,
    /// met
    Met,
    /// muse
    Muse,
    /// neighbor
    Neighbor,
    /// parent
    Parent,
    /// sibling
    Sibling,
    /// spouse
    Spouse,
    /// sweetheart
    Sweetheart,
}

/// Relation @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum RelationType {
    /// Relation @type
    Relation,
}

/// Defines the Name object, which contains information about the entity's name components.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    /// The JSContact type of the object. The value MUST be "Name", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    name_type: Option<NameType>,
    /// Components making up the name (e.g., given name, surname).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<NameComponent>>,
    /// Whether the name components are ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ordered: Option<bool>,
    /// Default separator between name components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_separator: Option<String>,
    /// The full name as a single string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<String>,
    /// Custom sorting order for the name components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<HashMap<String, String>>,
    /// The script used in the phonetic property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic_script: Option<String>,
    /// The phonetic system used in the phonetic property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic_system: Option<PhoneticSystem>,
}

/// The phonetic system used in the related value of the phonetic property.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PhoneticSystem {
    /// International Phonetic Alphabet
    Ipa,
    /// Cantonese romanization system "Jyutping"
    Jyut,
    /// Standard Mandarin romanization system "Hanyu Pinyin".
    Piny,
}

impl Default for Name {
    fn default() -> Self {
        Self {
            #[cfg(feature = "typed")]
            name_type: Some(NameType::Name),
            components: None,
            is_ordered: None,
            default_separator: None,
            full: None,
            sort_as: None,
            phonetic_script: None,
            phonetic_system: None,
        }
    }
}

/// Name @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum NameType {
    /// Name @type
    Name,
}

/// Represents individual components of a name, such as given name or surname.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NameComponent {
    /// The JSContact type of the object. Must be "NameComponent".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    name_component_type: Option<NameComponentType>,
    /// The value of the name component (e.g., "John").
    pub value: String,
    /// The kind of the name component (e.g., given, surname).
    pub kind: NameComponentKind,
    /// The phonetic representation of the name component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic: Option<String>,
}

impl NameComponent {
    /// Creates a new NameComponent object with the specified kind and value.
    pub fn new(kind: NameComponentKind, value: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            name_component_type: Some(NameComponentType::NameComponent),
            value: value.to_string(),
            kind,
            phonetic: None,
        }
    }
}

/// NameComponent @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum NameComponentType {
    /// NameComponent @type
    NameComponent,
}

/// The kind of the name component.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NameComponentKind {
    ///  a credential, also known as "accreditation qualifier" or "honorific suffix", e.g., "B.A.", "Esq.".
    Credential,
    /// a generation marker or qualifier, e.g., "Jr." or "III".
    Generation,
    /// a given name, also known as "first name" or "personal name".
    Given,
    /// a name that appears between the given and surname such as a middle name or patronymic name.
    Given2,
    /// a formatting separator between two ordered name non-separator components. The value property of the component includes the verbatim separator, for example, a hyphen character or even an empty string.
    /// This value has higher precedence than the defaultSeparator property of the Name.
    Separator,
    ///  a surname, also known as "last name" or "family name".
    Surname,
    /// a secondary surname (used in some cultures), also known as "maternal surname".
    Surname2,
    /// an honorific title or prefix, e.g., "Mr.", "Ms.", or "Dr.".
    Title,
}

impl From<String> for NameComponentKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "credential" => NameComponentKind::Credential,
            "generation" => NameComponentKind::Generation,
            "given" => NameComponentKind::Given,
            "given2" => NameComponentKind::Given2,
            "separator" => NameComponentKind::Separator,
            "surname" => NameComponentKind::Surname,
            "surname2" => NameComponentKind::Surname2,
            "title" => NameComponentKind::Title,
            _ => panic!("Invalid NameComponentKind"),
        }
    }
}

/// Defines the Nickname object, which includes nicknames for the entity.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Nickname {
    /// The JSContact type of the object. Must be "Nickname".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    nickname_type: Option<NicknameType>,
    /// The nickname value.
    pub name: String,
    /// Contexts in which to use the nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the nickname relative to others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u32>,
}

/// Nickname @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum NicknameType {
    /// Nickname @type
    Nickname,
}

/// Represents an Organization object containing company or organization information.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    /// The JSContact type of the object. Must be "Organization".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    org_type: Option<OrganizationType>,
    /// The name of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Organizational units within the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<OrgUnit>>,
    /// Custom sorting order for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<String>,
    /// Contexts in which the organization is relevant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
}

/// Organization @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum OrganizationType {
    /// Organization @type
    Organization,
}

/// Represents a unit within an organization, such as a department.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrgUnit {
    /// The JSContact type of the object. Must be "OrgUnit".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    unit_type: Option<OrgUnitType>,
    /// The name of the organizational unit.
    pub name: String,
    /// Custom sorting order for the organizational unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<String>,
}

/// OrgUnit @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum OrgUnitType {
    /// OrgUnit @type
    OrgUnit,
}

impl OrgUnit {
    /// Creates a new OrgUnit object with the specified name.
    pub fn new(name: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            unit_type: Some(OrgUnitType::OrgUnit),
            name: name.to_string(),
            sort_as: None,
        }
    }
}

/// Represents how to address or refer to the entity, including grammatical gender and pronouns.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpeakToAs {
    /// The JSContact type of the object. Must be "SpeakToAs".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    speak_to_as_type: Option<SpeakToAsType>,
    /// Grammatical gender to use in salutations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grammatical_gender: Option<GrammaticalGender>,
    /// Pronouns associated with the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<HashMap<String, Pronouns>>,
}

/// The grammatical gender to use in salutations and other grammatical constructs.
/// For example, the German language distinguishes by grammatical gender in salutations such as "Sehr geehrte" (feminine) and "Sehr geehrter" (masculine).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GrammaticalGender {
    /// animate
    Animate,
    /// common
    Common,
    /// feminine
    Feminine,
    /// inanimate
    Inanimate,
    /// masculine
    Masculine,
    /// neuter
    Neuter,
}

/// SpeakToAs @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum SpeakToAsType {
    /// SpeakToAs @type
    SpeakToAs,
}

/// Defines pronouns used for the entity, such as they/them.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pronouns {
    /// The JSContact type of the object. Must be "Pronouns".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pronoun_type: Option<PronounsType>,
    /// The pronouns value (e.g., "they/them").
    pub pronouns: String,
    /// Contexts in which to use the pronouns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the pronouns relative to others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u32>,
}

/// Pronouns @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum PronounsType {
    /// Pronouns @type
    Pronouns,
}

impl Pronouns {
    /// Creates a new Pronouns object with the specified pronouns.
    pub fn new(pronouns: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            pronoun_type: Some(PronounsType::Pronouns),
            pronouns: pronouns.to_string(),
            contexts: None,
            pref: None,
        }
    }
}

/// Represents titles or roles of the entity, such as job titles or functional positions.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    /// The JSContact type of the object. Must be "Title".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    title_type: Option<TitleType>,
    /// The title or role name.
    pub name: String,
    /// The kind of title (e.g., title, role).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TitleKind>,
    /// Identifier of the organization associated with this title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

/// Title @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum TitleType {
    /// Title @type
    Title,
}

impl Title {
    /// Creates a new Title object with the specified name.
    pub fn new(name: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            title_type: Some(TitleType::Title),
            name: name.to_string(),
            kind: None,
            organization_id: None,
        }
    }
}

/// Title kind
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TitleKind {
    /// a role
    Role,
    /// a title
    Title,
}

impl From<String> for TitleKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "role" => TitleKind::Role,
            "title" => TitleKind::Title,
            _ => panic!("Invalid TitleKind"),
        }
    }
}

/// Defines email addresses associated with the entity.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmailAddress {
    /// The JSContact type of the object. Must be "EmailAddress".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    email_type: Option<EmailAddressType>,
    /// The email address.
    pub address: String,
    /// Contexts in which to use the email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the email address relative to others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u32>,
    /// Custom label for the email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// EmailAddress @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum EmailAddressType {
    /// EmailAddress @type
    EmailAddress,
}

impl EmailAddress {
    /// Creates a new EmailAddress object with the specified email address.
    pub fn new(address: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            email_type: Some(EmailAddressType::EmailAddress),
            address: address.to_string(),
            contexts: None,
            pref: None,
            label: None,
        }
    }
}

/// Represents online services such as social media or messaging accounts.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnlineService {
    /// The JSContact type of the object. Must be "OnlineService".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    service_type: Option<OnlineServiceType>,
    /// The name of the online service or protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The URI identifying the entity on the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The username or handle on the online service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Contexts in which to use the online service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the service relative to others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u32>,
    /// Custom label for the online service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// OnlineService @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum OnlineServiceType {
    /// OnlineService @type
    OnlineService,
}

/// Defines phone numbers for the entity, including features like voice or text.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
    /// The JSContact type of the object. Must be "Phone".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    phone_type: Option<PhoneType>,
    /// The phone number, either as a URI or free text.
    pub number: String,
    /// Contact features the phone number supports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<HashMap<PhoneFeature, bool>>,
    /// Contexts in which to use the phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the phone number relative to others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u32>,
    /// Custom label for the phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// The set of contact features that the phone number may be used for.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PhoneFeature {
    /// this number supports sending faxes.
    Fax,
    /// this number is a main phone number such as the number of the front desk at a company, as opposed to a direct-dial number of an individual employee.
    #[serde(rename = "main-number")]
    MainNumber,
    /// this number is for a mobile phone.
    Mobile,
    ///  this number is for a pager or beeper.
    Pager,
    /// this number supports text messages (SMS).
    Text,
    /// this number is for a device for people with hearing or speech difficulties.
    Textphone,
    /// this number supports video conferencing.
    Video,
    ///  this number supports calling by voice.
    Voice,
}

/// The contexts in which to use the contact information.
/// For example, someone might have distinct phone numbers for work and private contexts and may set the desired context on the respective phone number in the phones property.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Context {
    /// the contact information that may be used in a private context.
    Private,
    /// the contact information that may be used in a professional context.
    Work,
}

/// Phone @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum PhoneType {
    /// Phone @type
    Phone,
}

impl Phone {
    /// Creates a new Phone object with the specified phone number.
    pub fn new(number: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            phone_type: Some(PhoneType::Phone),
            number: number.to_string(),
            features: None,
            contexts: None,
            pref: None,
            label: None,
        }
    }
}

/// Represents preferred languages for communication.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LanguagePref {
    /// The JSContact type of the object. Must be "LanguagePref".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    lang_pref_type: Option<LanguagePrefType>,
    /// The preferred language as a language tag (e.g., en, fr).
    pub language: String,
    /// Contexts in which to use the preferred language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the language relative to others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u32>,
}

/// LanguagePref @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum LanguagePrefType {
    /// LanguagePref @type
    LanguagePref,
}

impl LanguagePref {
    /// Creates a new LanguagePref object with the specified language.
    pub fn new(language: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            lang_pref_type: Some(LanguagePrefType::LanguagePref),
            language: language.to_string(),
            contexts: None,
            pref: None,
        }
    }
}

/// Represents memorable dates and events for the entity.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Anniversary {
    /// The JSContact type of the object. Must be "Anniversary".
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    anniversary_type: Option<AnniversaryType>,
    /// The date of the anniversary.
    pub date: DateObject,
    /// The kind of anniversary
    pub kind: AnniversaryKind,
    /// Contexts in which to use the anniversary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<String, bool>>,
    /// Preference of the anniversary relative to others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<Address>,
}

/// The kind of anniversary
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AnniversaryKind {
    /// a birthday anniversary
    Birth,
    /// a deathday anniversary
    Death,
    /// a wedding day anniversary
    Wedding,
}

impl From<String> for AnniversaryKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "birth" => AnniversaryKind::Birth,
            "death" => AnniversaryKind::Death,
            "wedding" => AnniversaryKind::Wedding,
            _ => panic!("Invalid AnniversaryKind"),
        }
    }
}

/// Anniversary @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum AnniversaryType {
    /// Anniversary @type
    Anniversary,
}

impl Anniversary {
    /// Creates a new Anniversary object with the specified date and kind.
    pub fn new(kind: AnniversaryKind, date: DateObject) -> Self {
        Self {
            #[cfg(feature = "typed")]
            anniversary_type: Some(AnniversaryType::Anniversary),
            date,
            kind,
            contexts: None,
            place: None,
        }
    }
}

/// Represents a date object, which can be a timestamp or a partial date.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum DateObject {
    // Check first if the date is a timestamp because timestamp has a field
    /// Timestamp
    Timestamp(Timestamp),
    /// PartialDate
    PartialDate(PartialDate),
}

/// Timestamp
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    /// The JSContact type of the object. The value MUST be "Timestamp", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    timestamp_type: Option<TimestampType>,

    /// The point in time in UTC time
    pub utc: String,
}

/// Timestamp @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum TimestampType {
    /// Timestamp @type
    Timestamp,
}

impl Timestamp {
    /// Creates a new Timestamp object with the specified UTC time.
    pub fn new(utc: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            timestamp_type: Some(TimestampType::Timestamp),
            utc: utc.to_string(),
        }
    }
}

/// A PartialDate object represents a complete or partial calendar date in the Gregorian calendar.  It represents a complete date, a year, a month in a year, or a day in a month.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartialDate {
    /// The JSContact type of the object. The value MUST be "PartialDate", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    partial_date_type: Option<PartialDateType>,
    /// The calendar year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u64>,
    /// The calendar month, represented as the integers 1 <= month <= 12. If this property is set, then either the year or the day property MUST be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u32>,
    /// The calendar month day, represented as the integers 1 <= day <= 31, depending on the validity within the month and year. If this property is set, then the month property MUST be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<u32>,

    /// The calendar system in which this date occurs, in lowercase.  This MUST be either a calendar system name registered as a Common Locale Data Repository (CLDR) RFC7529 or a vendor-specific value.
    /// The year, month, and day still MUST be represented in the Gregorian calendar.
    /// Note that the year property might be required to convert the date between the Gregorian calendar and the respective calendar system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_scale: Option<String>,
}

/// PartialDate @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum PartialDateType {
    /// PartialDate @type
    PartialDate,
}

/// The addresses of the entity represented by the Card, such as postal addresses or geographic locations.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The JSContact type of the object. The value MUST be "Address", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    address_type: Option<AddressType>,
    /// The components that make up the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<AddressComponent>>,
    /// The indicator if the address components in the components property are ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ordered: Option<bool>,
    /// The Alpha-2 country code [ISO.3166-1].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// A "geo:" URI RFC5870 for the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<String>,
    /// The time zone in which the address is located. This MUST be a time zone name registered in the IANA Time Zone Database [IANA-TZ].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// The contexts in which to use this address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<HashMap<AddressContext, bool>>,
    /// The full address, including street, region, or country. The purpose of this property is to define an address, even if the individual address components are not known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<String>,
    /// The default separator to insert between address component values when concatenating all address component values to a single String.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_separator: Option<String>,
    /// The preference of the address in relation to other addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pref: Option<u64>,
    /// The script used in the value of the AddressComponent phonetic property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic_script: Option<String>,
    /// The phonetic system used in the AddressComponent phonetic property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic_system: Option<PhoneticSystem>,
}

/// The contexts in which to use this address.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AddressContext {
    /// an address to be used for billing.
    Billing,
    /// an address to be used for delivering physical items.
    Delivery,
    /// may be used in a private context.
    Private,
    /// may be used in a professional context.
    Work,
}

/// Address @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum AddressType {
    /// Address @type
    Address,
}

/// The components that make up the address.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddressComponent {
    /// The JSContact type of the object. The value MUST be "AddressComponent", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    component_type: Option<AddressComponentType>,
    /// The value of the address component.
    pub value: String,
    /// The kind of the address component.
    pub kind: AddressComponentKind,
    /// The pronunciation of the name component. If this property is set, then at least one of the Address object phoneticSystem or phoneticScript properties MUST be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic: Option<String>,
}

/// AddressComponent @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum AddressComponentType {
    /// AddressComponent @type
    AddressComponent,
}

impl AddressComponent {
    /// Creates a new AddressComponent object with the specified kind and value.
    pub fn new(kind: AddressComponentKind, value: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            component_type: Some(AddressComponentType::AddressComponent),
            value: value.to_string(),
            kind,
            phonetic: None,
        }
    }
}

/// The kind of the address component.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AddressComponentKind {
    /// the extension designation such as the apartment number, unit, or box number.
    Apartment,
    /// the block name or number.
    Block,
    /// the building, tower, or condominium the address is located in.
    Building,
    /// the country name.
    Country,
    /// the cardinal direction or quadrant, e.g., "north".
    Direction,
    ///  the district name.
    District,
    /// the floor or level the address is located on.
    Floor,
    /// the publicly known prominent feature that can substitute the street name and number, e.g., "White House" or "Taj Mahal".
    Landmark,
    /// the municipality, city, town, village, post town, or other locality.
    Locality,
    /// the street name.
    Name,
    ///  the street number, e.g., "123". This value is not restricted to numeric values and can include any value such as number ranges ("112-10"), grid style ("39.2 RD"), alphanumerics ("N6W23001"), or fractionals ("123 1/2").
    Number,
    /// the postal code, post code, ZIP code, or other short code associated with the address by the relevant country's postal system.
    Postcode,
    ///  the post office box number or identifier.
    #[serde(rename = "postOfficeBox")]
    PostOfficeBox,
    /// the administrative area such as province, state, prefecture, county, or canton.
    Region,
    /// the room, suite number, or identifier.
    Room,
    /// a formatting separator between two ordered address non-separator components. The value property of the component includes the verbatim separator, for example, a hyphen character or even an empty string. This value has higher precedence than the defaultSeparator property of the Address.
    Separator,
    ///  the subdistrict, ward, or other subunit of a district.
    Subdistrict,
}

impl From<String> for AddressComponentKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "apartment" => AddressComponentKind::Apartment,
            "block" => AddressComponentKind::Block,
            "building" => AddressComponentKind::Building,
            "country" => AddressComponentKind::Country,
            "direction" => AddressComponentKind::Direction,
            "district" => AddressComponentKind::District,
            "floor" => AddressComponentKind::Floor,
            "landmark" => AddressComponentKind::Landmark,
            "locality" => AddressComponentKind::Locality,
            "name" => AddressComponentKind::Name,
            "number" => AddressComponentKind::Number,
            "postcode" => AddressComponentKind::Postcode,
            "postOfficeBox" => AddressComponentKind::PostOfficeBox,
            "region" => AddressComponentKind::Region,
            "room" => AddressComponentKind::Room,
            "separator" => AddressComponentKind::Separator,
            "subdistrict" => AddressComponentKind::Subdistrict,
            _ => panic!("Invalid AddressComponentKind"),
        }
    }
}

/// The free-text notes that are associated with the Card.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    /// The JSContact type of the object. The value MUST be "Note", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    note_type: Option<NoteType>,
    /// The free-text value of this note.
    pub note: String,
    /// The date and time when this note was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The author of this note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
}

/// Note @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum NoteType {
    /// Note @type
    Note,
}

/// The author of a note.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    /// The JSContact type of the object. The value MUST be "Author", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    author_type: Option<AuthorType>,
    /// The name of this author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URI value that identifies the author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// Author @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum AuthorType {
    /// Author @type
    Author,
}

/// The personal information of the entity represented by the Card.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInfo {
    ///The JSContact type of the object.  The value MUST be "PersonalInfo", if set.
    #[cfg(feature = "typed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    personal_info_type: Option<PersonalInfoType>,
    /// The kind of personal information.
    pub kind: PersonalInfoKind,
    /// The actual information.
    pub value: String,
    /// The level of expertise or engagement in hobby or interest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<PersonalInfoLevel>,
    /// The position of the personal information in the list of all PersonalInfo objects that have the same kind property value in the Card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_as: Option<u64>,
    /// A custom label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// The kind of personal information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PersonalInfoKind {
    /// a field of expertise or a credential
    Expertise,
    /// a hobby
    Hobby,
    /// an interest
    Interest,
}

impl From<String> for PersonalInfoKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "expertise" => PersonalInfoKind::Expertise,
            "hobby" => PersonalInfoKind::Hobby,
            "interest" => PersonalInfoKind::Interest,
            _ => panic!("Invalid PersonalInfoKind"),
        }
    }
}

/// PersonalInfo @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum PersonalInfoType {
    /// PersonalInfo @type
    PersonalInfo,
}

impl PersonalInfo {
    /// Creates a new PersonalInfo object with the specified kind and value.
    pub fn new(kind: PersonalInfoKind, value: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            personal_info_type: Some(PersonalInfoType::PersonalInfo),
            kind,
            value: value.to_string(),
            level: None,
            list_as: None,
            label: None,
        }
    }
}

/// The level of expertise or engagement in hobby or interest.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PersonalInfoLevel {
    /// High level of expertise or engagement.
    High,
    /// Medium level of expertise or engagement.
    Medium,
    /// Low level of expertise or engagement.
    Low,
}
