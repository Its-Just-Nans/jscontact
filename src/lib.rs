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
//! let json = serde_json::json!({
//!     "@type": "Card",
//!     "version": "1.0",
//!     "uid": "1234"
//! });
//! let card: Card = serde_json::from_value(json).unwrap();
//! ```
//!
//! Simple creation example:
//! ```rust
//! use jscontact::{Card, CardKind, Name, NameComponent, NameComponentKind};
//! use serde_json;
//!
//! let mut card = Card::new_with_latest_version("1234");
//! card.kind = Some(CardKind::Individual);
//! let json = serde_json::to_string(&card).unwrap();
//! ```

#![deny(
    // missing_docs,
    clippy::all,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cargo
)]
#![warn(clippy::multiple_crate_versions)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the primary Card object as defined in RFC 9553, storing metadata and contact properties.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// The JSContact type of the Card object. Must be "Card".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    card_type: String,
    /// The JSContact version of this Card.
    pub version: CardVersion,

    pub created: Option<String>,

    /// A unique identifier for the Card.
    pub uid: String,
    /// The kind of entity the Card represents (e.g., individual, group).
    pub kind: Option<CardKind>,
    /// The language used in the Card (e.g., en, fr).
    pub language: Option<String>,
    /// Members of a group Card, if applicable.
    pub members: Option<HashMap<String, bool>>,
    /// Identifier for the product that created the Card.
    pub prod_id: Option<String>,
    /// Related Cards with their relationship types.
    pub related_to: Option<HashMap<String, Relation>>,
    /// The last modification time of the Card.
    pub updated: Option<String>,
    /// The name of the entity represented by the Card.
    pub name: Option<Name>,
    /// Nicknames of the entity.
    pub nicknames: Option<HashMap<String, Nickname>>,
    /// Organizations associated with the entity.
    pub organizations: Option<HashMap<String, Organization>>,
    /// How to address or refer to the entity.
    pub speak_to_as: Option<SpeakToAs>,
    /// Job titles or roles of the entity.
    pub titles: Option<HashMap<String, Title>>,
    /// Email addresses for contacting the entity.
    pub emails: Option<HashMap<String, EmailAddress>>,
    /// Online services or social media associated with the entity.
    pub online_services: Option<HashMap<String, OnlineService>>,
    /// Phone numbers for contacting the entity.
    pub phones: Option<HashMap<String, Phone>>,
    /// Preferred languages for communication.
    pub preferred_languages: Option<HashMap<String, LanguagePref>>,

    pub calendars: Option<HashMap<String, Calendar>>,

    pub scheduling_addresses: Option<HashMap<String, SchedulingAddress>>,

    /// Localizations provide language-specific alternatives for existing property values and SHOULD NOT add new properties.
    pub localizations: Option<HashMap<String, LocalizationObject>>,
    /// The memorable dates and events for the entity represented by the Card.
    pub anniversaries: Option<HashMap<String, Anniversary>>,

    pub addresses: Option<HashMap<String, Address>>,

    pub crypto_keys: Option<HashMap<String, CryptoKey>>,

    pub directories: Option<HashMap<String, Directory>>,

    pub links: Option<HashMap<String, Link>>,

    pub media: Option<HashMap<String, Media>>,

    pub keywords: Option<HashMap<String, bool>>,

    pub notes: Option<HashMap<String, Note>>,

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

    /// Creates a new Card object with the latest version and the specified unique identifier.
    pub fn new_with_latest_version(uid: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            card_type: "Card".to_string(),
            uid: uid.to_string(),
            ..Card::new(CardVersion::OneDotZero, uid)
        }
    }
}

/// Represents the card version.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CardVersion {
    /// version 1.0
    #[serde(rename = "1.0")]
    OneDotZero,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    /// The @type property value MUST be "Calendar", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    calendar_type: Option<CalendarType>,
    pub kind: Option<CalendarKind>,
    pub uri: String,
    pub contexts: Option<HashMap<Context, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
}

/// Calendar @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum CalendarType {
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
            kind: None,
            contexts: None,
            pref: None,
            label: None,
        }
    }
}

/// Calendar kind
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CalendarKind {
    /// The resource is a calendar that contains entries such as calendar events or tasks.
    Calendar,
    /// The resource allows for free-busy lookups, for example, to schedule group events.
    FreeBusy,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchedulingAddress {
    /// The JSContact type of the object. The value MUST be "SchedulingAddress", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    scheduling_address_type: Option<SchedulingAddressType>,
    pub uri: String,
    pub contexts: Option<HashMap<Context, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
}

/// SchedulingAddress @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum SchedulingAddressType {
    /// SchedulingAddress @type
    SchedulingAddress,
}

impl SchedulingAddress {
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CryptoKey {
    /// The @type property value MUST be "CryptoKey", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    crypto_key_type: Option<CryptoKeyType>,
    pub uri: String,
    pub kind: Option<String>,
    pub contexts: Option<HashMap<Context, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
}

/// CryptoKey @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum CryptoKeyType {
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
            kind: None,
            contexts: None,
            pref: None,
            label: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    /// The @type property value MUST be "Directory", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    directory_type: Option<DirectoryType>,
    pub kind: Option<DirectoryKind>,
    pub uri: String,
    pub contexts: Option<HashMap<Context, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
    pub list_as: Option<u64>,
}

/// Directory @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum DirectoryType {
    /// Directory @type
    Directory,
}

impl Directory {
    pub fn new(uri: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            directory_type: Some(DirectoryType::Directory),
            kind: None,
            uri: uri.to_string(),
            contexts: None,
            pref: None,
            label: None,
            list_as: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DirectoryKind {
    Directory,
    Entry,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LocalizationObject {
    /// Priority
    /// The key is the path of the property to localize, and the value is the localized value.
    PatchObject(HashMap<String, PatchingObject>),
    Replacement {
        name: Box<Option<Name>>,
        nicknames: Box<Option<Nickname>>,
        organizations: Box<Option<Organization>>,
        speak_to_as: Box<Option<SpeakToAs>>,
        titles: Box<Option<Title>>,
        emails: Box<Option<EmailAddress>>,
        online_services: Box<Option<OnlineService>>,
        phones: Box<Option<Phone>>,
        preferred_languages: Box<Option<LanguagePref>>,
        calendars: Box<Option<Calendar>>,
        scheduling_addresses: Box<Option<SchedulingAddress>>,
        anniversaries: Box<Option<Anniversary>>,
        addresses: Box<Option<Address>>,
        crypto_keys: Box<Option<CryptoKey>>,
        directories: Box<Option<Directory>>,
        links: Box<Option<Link>>,
        media: Box<Option<Media>>,
        notes: Box<Option<Note>>,
        personal_info: Box<Option<PersonalInfo>>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PatchingObject {
    String(String),
    Nickname(Nickname),
    // Organization(Organization),
    // Title(Title),
    // EmailAddress(EmailAddress),
    // OnlineService(OnlineService),
    // Phone(Phone),
    // LanguagePref(LanguagePref),
    // Calendar(Calendar),
    // SchedulingAddress(SchedulingAddress),
    // Anniversary(Anniversary),
    Address(Address),
    // CryptoKey(CryptoKey),
    // Directory(Directory),
    // Link(Link),
    // Media(Media),
    // Note(Note),
    // PersonalInfo(PersonalInfo),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    /// The @type property value MUST be "Media", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    media_hidden_type: Option<MediaType>,
    pub kind: MediaKind,
    pub uri: String,
    pub media_type: Option<String>,
    pub contexts: Option<HashMap<Context, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
}

/// Media @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum MediaType {
    /// Media @type
    Media,
}

impl Media {
    pub fn new(uri: &str, kind: MediaKind) -> Self {
        Self {
            #[cfg(feature = "typed")]
            media_hidden_type: Some(MediaType::Media),
            kind,
            uri: uri.to_string(),
            media_type: None,
            contexts: None,
            pref: None,
            label: None,
        }
    }
}

/// Media kind
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MediaKind {
    /// the resource is a photograph or avatar.
    Photo,
    /// the resource is audio media, e.g., to specify the proper pronunciation of the name property contents.
    Sound,
    /// the resource is a graphic image or logo associated with the entity represented by the Card.
    Logo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// The @type property value MUST be "Link", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    link_type: Option<LinkType>,
    pub kind: Option<LinkKind>,
    pub uri: String,
    pub contexts: Option<HashMap<Context, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
}

/// Link @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum LinkType {
    /// Link @type
    Link,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub contexts: Option<HashMap<Context, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
}

impl From<Resource> for Link {
    fn from(resource: Resource) -> Self {
        Self {
            #[cfg(feature = "typed")]
            link_type: Some(LinkType::Link),
            kind: None,
            uri: "".to_string(),
            contexts: resource.contexts,
            pref: resource.pref,
            label: resource.label,
        }
    }
}

impl Link {
    /// Creates a new Link object with the specified URI.
    pub fn new(uri: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            link_type: Some(LinkType::Link),
            kind: None,
            uri: uri.to_string(),
            ..Resource::default().into()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LinkKind {
    Contact,
}

/// Represents the Relation object for associating related Cards.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    /// The JSContact type of the object. Must be "Relation".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    relation_type: Option<RelationType>,
    /// The relationship types to related Cards.
    pub relation: Option<HashMap<RelationshipType, bool>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipType {
    Acquaintance,
    Agent,
    Child,
    #[serde(rename = "co-resident")]
    CoResident,
    #[serde(rename = "co-worker")]
    CoWorker,
    Colleague,
    Contact,
    Crush,
    Date,
    Emergency,
    Friend,
    Kin,
    Me,
    Met,
    Muse,
    Neighbor,
    Parent,
    Sibling,
    Spouse,
    Sweetheart,
}

#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum RelationType {
    Relation,
}

/// Defines the Name object, which contains information about the entity's name components.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    /// The JSContact type of the object. The value MUST be "Name", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    name_type: Option<NameType>,
    /// Components making up the name (e.g., given name, surname).
    pub components: Option<Vec<NameComponent>>,
    /// Whether the name components are ordered.
    pub is_ordered: Option<bool>,
    /// Default separator between name components.
    pub default_separator: Option<String>,
    /// The full name as a single string.
    pub full: Option<String>,
    /// Custom sorting order for the name components.
    pub sort_as: Option<HashMap<String, String>>,
    /// The script used in the phonetic property.
    pub phonetic_script: Option<String>,
    /// The phonetic system used in the phonetic property.
    pub phonetic_system: Option<PhoneticSystem>,
}

/// The phonetic system used in the related value of the phonetic property.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub enum NameType {
    #[default]
    Name,
}

/// Represents individual components of a name, such as given name or surname.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NameComponent {
    /// The JSContact type of the object. Must be "NameComponent".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    name_component_type: Option<NameComponentType>,
    /// The value of the name component (e.g., "John").
    pub value: String,
    /// The kind of the name component (e.g., given, surname).
    pub kind: NameComponentKind,
    /// The phonetic representation of the name component.
    pub phonetic: Option<String>,
}

impl NameComponent {
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

#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum NameComponentType {
    NameComponent,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NameComponentKind {
    Credential,
    Generation,
    Given,
    Given2,
    Separator,
    Surname,
    Surname2,
    Title,
}

/// Defines the Nickname object, which includes nicknames for the entity.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Nickname {
    /// The JSContact type of the object. Must be "Nickname".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    nickname_type: Option<NicknameType>,
    /// The nickname value.
    pub name: String,
    /// Contexts in which to use the nickname.
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the nickname relative to others.
    pub pref: Option<u32>,
}

#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NicknameType {
    Nickname,
}

/// Represents an Organization object containing company or organization information.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    /// The JSContact type of the object. Must be "Organization".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    org_type: Option<OrganizationType>,
    /// The name of the organization.
    pub name: Option<String>,
    /// Organizational units within the organization.
    pub units: Option<Vec<OrgUnit>>,
    /// Custom sorting order for the organization.
    pub sort_as: Option<String>,
    /// Contexts in which the organization is relevant.
    pub contexts: Option<HashMap<Context, bool>>,
}

#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum OrganizationType {
    Organization,
}

/// Represents a unit within an organization, such as a department.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrgUnit {
    /// The JSContact type of the object. Must be "OrgUnit".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    unit_type: Option<OrgUnitType>,
    /// The name of the organizational unit.
    pub name: String,
    /// Custom sorting order for the organizational unit.
    pub sort_as: Option<String>,
}

#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum OrgUnitType {
    OrgUnit,
}

impl OrgUnit {
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
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpeakToAs {
    /// The JSContact type of the object. Must be "SpeakToAs".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    speak_to_as_type: Option<SpeakToAsType>,
    /// Grammatical gender to use in salutations.
    pub grammatical_gender: Option<GrammaticalGender>,
    /// Pronouns associated with the entity.
    pub pronouns: Option<HashMap<String, Pronouns>>,
}

/// The grammatical gender to use in salutations and other grammatical constructs.
/// For example, the German language distinguishes by grammatical gender in salutations such as "Sehr geehrte" (feminine) and "Sehr geehrter" (masculine).
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GrammaticalGender {
    Animate,
    Common,
    Feminine,
    Inanimate,
    Masculine,
    Neuter,
}

#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum SpeakToAsType {
    SpeakToAs,
}

/// Defines pronouns used for the entity, such as they/them.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pronouns {
    /// The JSContact type of the object. Must be "Pronouns".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    pronoun_type: Option<PronounsType>,
    /// The pronouns value (e.g., "they/them").
    pub pronouns: String,
    /// Contexts in which to use the pronouns.
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the pronouns relative to others.
    pub pref: Option<u32>,
}

#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum PronounsType {
    Pronouns,
}

impl Pronouns {
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    /// The JSContact type of the object. Must be "Title".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    title_type: Option<TitleType>,
    /// The title or role name.
    pub name: String,
    /// The kind of title (e.g., title, role).
    pub kind: Option<TitleKind>,
    /// Identifier of the organization associated with this title.
    pub organization_id: Option<String>,
}

/// Title @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum TitleType {
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TitleKind {
    Role,
    Title,
}

/// Defines email addresses associated with the entity.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailAddress {
    /// The JSContact type of the object. Must be "EmailAddress".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    email_type: Option<EmailAddressType>,
    /// The email address.
    pub address: String,
    /// Contexts in which to use the email address.
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the email address relative to others.
    pub pref: Option<u32>,
    /// Custom label for the email address.
    pub label: Option<String>,
}

/// EmailAddress @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum EmailAddressType {
    /// EmailAddress @type
    EmailAddress,
}

impl EmailAddress {
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
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OnlineService {
    /// The JSContact type of the object. Must be "OnlineService".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    service_type: Option<OnlineServiceType>,
    /// The name of the online service or protocol.
    pub service: Option<String>,
    /// The URI identifying the entity on the service.
    pub uri: Option<String>,
    /// The username or handle on the online service.
    pub user: Option<String>,
    /// Contexts in which to use the online service.
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the service relative to others.
    pub pref: Option<u32>,
    /// Custom label for the online service.
    pub label: Option<String>,
}

/// OnlineService @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum OnlineServiceType {
    /// OnlineService @type
    OnlineService,
}

/// Defines phone numbers for the entity, including features like voice or text.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
    /// The JSContact type of the object. Must be "Phone".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    phone_type: Option<PhoneType>,
    /// The phone number, either as a URI or free text.
    pub number: String,
    /// Contact features the phone number supports (e.g., voice, text).
    pub features: Option<HashMap<PhoneFeature, bool>>,
    /// Contexts in which to use the phone number.
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the phone number relative to others.
    pub pref: Option<u32>,
    /// Custom label for the phone number.
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum PhoneFeature {
    Fax,
    #[serde(rename = "main-number")]
    MainNumber,
    Mobile,
    Pager,
    Text,
    Textphone,
    Video,
    Voice,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Context {
    Private,
    Work,
}

/// Phone @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum PhoneType {
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LanguagePref {
    /// The JSContact type of the object. Must be "LanguagePref".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    lang_pref_type: Option<LanguagePrefType>,
    /// The preferred language as a language tag (e.g., en, fr).
    pub language: String,
    /// Contexts in which to use the preferred language.
    pub contexts: Option<HashMap<Context, bool>>,
    /// Preference of the language relative to others.
    pub pref: Option<u32>,
}

/// LanguagePref @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum LanguagePrefType {
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Anniversary {
    /// The JSContact type of the object. Must be "Anniversary".
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    anniversary_type: Option<AnniversaryType>,
    /// The date of the anniversary.
    pub date: DateObject,
    /// The type of anniversary (e.g., birthday, work anniversary).
    pub kind: AnniversaryKind,
    /// Contexts in which to use the anniversary.
    pub contexts: Option<HashMap<String, bool>>,
    /// Preference of the anniversary relative to others.
    pub place: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AnniversaryKind {
    Birth,
    Death,
    Wedding,
}

/// Anniversary @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum AnniversaryType {
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DateObject {
    // Check first if the date is a timestamp because timestamp has a field
    Timestamp(Timestamp),
    PartialDate(PartialDate),
}

/// Timestamp
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    /// The JSContact type of the object. The value MUST be "Timestamp", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    timestamp_type: Option<TimestampType>,

    /// The point in time in UTC time
    pub utc: String,
}

/// Timestamp @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum TimestampType {
    /// Timestamp @type
    Timestamp,
}

impl Timestamp {
    pub fn new(utc: &str) -> Self {
        Self {
            #[cfg(feature = "typed")]
            timestamp_type: Some(TimestampType::Timestamp),
            utc: utc.to_string(),
        }
    }
}

/// A PartialDate object represents a complete or partial calendar date in the Gregorian calendar.  It represents a complete date, a year, a month in a year, or a day in a month.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PartialDate {
    /// The JSContact type of the object. The value MUST be "PartialDate", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    partial_date_type: Option<PartialDateType>,

    pub year: Option<u64>,
    pub month: Option<u32>,
    pub day: Option<u32>,

    /// The calendar system in which this date occurs, in lowercase.  This MUST be either a calendar system name registered as a Common Locale Data Repository (CLDR) [RFC7529] or a vendor-specific value.
    /// The year, month, and day still MUST be represented in the Gregorian calendar.
    /// Note that the year property might be required to convert the date between the Gregorian calendar and the respective calendar system.
    pub calendar_scale: Option<String>,
}

/// PartialDate @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum PartialDateType {
    /// PartialDate @type
    PartialDate,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The JSContact type of the object. The value MUST be "Address", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    address_type: Option<AddressType>,
    pub street: Option<String>,
    pub components: Option<Vec<AddressComponent>>,
    pub is_ordered: Option<bool>,
    pub country_code: Option<String>,
    pub coordinates: Option<String>,
    pub time_zone: Option<String>,
    pub contexts: Option<HashMap<AddressContext, bool>>,
    pub full: Option<String>,
    pub default_separator: Option<String>,
    pub pref: Option<u64>,
    pub phonetic_script: Option<String>,
    pub phonetic_system: Option<PhoneticSystem>,
}

/// The contexts in which to use this address.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AddressType {
    /// Address @type
    Address,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddressComponent {
    /// The JSContact type of the object. The value MUST be "AddressComponent", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    component_type: Option<AddressComponentType>,
    pub value: String,
    pub kind: AddressComponentKind,
    pub phonetic: Option<String>,
}

/// AddressComponent @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AddressComponentType {
    /// AddressComponent @type
    AddressComponent,
}

impl AddressComponent {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AddressComponentKind {
    Apartment,
    Block,
    Building,
    Country,
    Direction,
    District,
    Floor,
    Landmark,
    Locality,
    Name,
    Number,
    Postcode,
    #[serde(rename = "postOfficeBox")]
    PostOfficeBox,
    Region,
    Room,
    Separator,
    Subdistrict,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    /// The JSContact type of the object. The value MUST be "Note", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    note_type: Option<NoteType>,
    pub note: String,
    pub created: Option<String>,
    pub author: Option<Author>,
}

/// Note @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum NoteType {
    /// Note @type
    Note,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    /// The JSContact type of the object. The value MUST be "Author", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    author_type: Option<AuthorType>,
    pub name: Option<String>,
    pub uri: Option<String>,
}

/// Author @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum AuthorType {
    /// Author @type
    Author,
}

/// The personal information of the entity represented by the Card.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInfo {
    ///The JSContact type of the object.  The value MUST be "PersonalInfo", if set.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    personal_info_type: Option<PersonalInfoType>,
    pub kind: PersonalInfoKind,
    pub value: String,
    pub level: Option<PersonalInfoLevel>,
    pub list_as: Option<u64>,
    pub label: Option<String>,
}

/// The kind of personal information.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PersonalInfoKind {
    /// a field of expertise or a credential
    Expertise,
    /// a hobby
    Hobby,
    /// an interest
    Interest,
}

/// PersonalInfo @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
pub enum PersonalInfoType {
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PersonalInfoLevel {
    /// High level of expertise or engagement.
    High,
    /// Medium level of expertise or engagement.
    Medium,
    /// Low level of expertise or engagement.
    Low,
}
