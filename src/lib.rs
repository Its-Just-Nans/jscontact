use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the primary Card object as defined in RFC 9553, storing metadata and contact properties.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// The JSContact type of the Card object. Must be "Card".
    #[serde(rename = "@type")]
    card_type: String,
    /// The JSContact version of this Card.
    pub version: String,

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

/// The kind of the entity the Card represents.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CardKind {
    /// a single person
    Individual,
    /// a group of people or entities
    Group,
    /// an organization
    Org,
    /// a named location
    Location,
    /// a device such as an appliance, a computer, or a network element
    Device,
    /// a software application
    Application,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CryptoKey {
    #[serde(rename = "@type")]
    crypto_key_type: Option<String>,
    pub uri: String,
    pub kind: Option<String>,
    pub contexts: Option<HashMap<String, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    #[serde(rename = "@type")]
    directory_type: Option<String>,
    pub kind: Option<DirectoryKind>,
    pub uri: String,
    pub contexts: Option<HashMap<String, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
    pub list_as: Option<u64>,
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
    PatchObject(HashMap<String, String>),
    PatchObject2(HashMap<String, Address>),
    Name {
        name: Name,
    },
    Nickname {
        nicknames: Nickname,
    },
    Organization {
        organizations: Organization,
    },
    SpeakToAs {
        speak_to_as: SpeakToAs,
    },
    Title {
        titles: Title,
    },
    EmailAddress {
        emails: EmailAddress,
    },
    OnlineService {
        online_services: OnlineService,
    },
    Phone {
        phones: Phone,
    },
    LanguagePref {
        preferred_languages: LanguagePref,
    },
    Media {
        media: Media,
    },
    Anniversary {
        anniversaries: Anniversary,
    },
    Note {
        notes: Note,
    },
    PersonalInfo {
        personal_info: PersonalInfo,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    #[serde(rename = "@type")]
    media_hidden_type: Option<String>,
    pub kind: MediaKind,
    pub uri: String,
    pub media_type: Option<String>,
    pub contexts: Option<HashMap<String, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MediaKind {
    Photo,
    Sound,
    Logo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    #[serde(rename = "@type")]
    link_type: Option<String>,
    pub kind: Option<LinkKind>,
    pub uri: String,
    pub contexts: Option<HashMap<String, bool>>,
    pub pref: Option<u64>,
    pub label: Option<String>,
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
    #[serde(rename = "@type")]
    relation_type: Option<String>,
    /// The relationship types to related Cards.
    pub relation: Option<HashMap<String, bool>>,
}

/// Defines the Name object, which contains information about the entity's name components.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Name {
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
    pub phonetic_system: Option<String>,
}

/// Represents individual components of a name, such as given name or surname.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NameComponent {
    /// The JSContact type of the object. Must be "NameComponent".
    #[serde(rename = "@type")]
    name_component_type: Option<String>,
    /// The value of the name component (e.g., "John").
    pub value: String,
    /// The kind of the name component (e.g., given, surname).
    pub kind: NameComponentKind,
    /// The phonetic representation of the name component.
    pub phonetic: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NameComponentKind {
    Title,
    Given,
    Given2,
    Surname,
    Surname2,
    Credential,
    Generation,
    Separator,
}

/// Defines the Nickname object, which includes nicknames for the entity.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Nickname {
    /// The JSContact type of the object. Must be "Nickname".
    #[serde(rename = "@type")]
    nickname_type: Option<String>,
    /// The nickname value.
    pub name: String,
    /// Contexts in which to use the nickname.
    pub contexts: Option<HashMap<String, bool>>,
    /// Preference of the nickname relative to others.
    pub pref: Option<u32>,
}

/// Represents an Organization object containing company or organization information.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    /// The JSContact type of the object. Must be "Organization".
    #[serde(rename = "@type")]
    org_type: Option<String>,
    /// The name of the organization.
    pub name: Option<String>,
    /// Organizational units within the organization.
    pub units: Option<Vec<OrgUnit>>,
    /// Custom sorting order for the organization.
    pub sort_as: Option<String>,
    /// Contexts in which the organization is relevant.
    pub contexts: Option<HashMap<String, bool>>,
}

/// Represents a unit within an organization, such as a department.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrgUnit {
    /// The JSContact type of the object. Must be "OrgUnit".
    #[serde(rename = "@type")]
    unit_type: Option<String>,
    /// The name of the organizational unit.
    pub name: String,
    /// Custom sorting order for the organizational unit.
    pub sort_as: Option<String>,
}

/// Represents how to address or refer to the entity, including grammatical gender and pronouns.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpeakToAs {
    /// The JSContact type of the object. Must be "SpeakToAs".
    #[serde(rename = "@type")]
    speak_to_as_type: Option<String>,
    /// Grammatical gender to use in salutations.
    pub grammatical_gender: Option<String>,
    /// Pronouns associated with the entity.
    pub pronouns: Option<HashMap<String, Pronouns>>,
}

/// Defines pronouns used for the entity, such as they/them.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pronouns {
    /// The JSContact type of the object. Must be "Pronouns".
    #[serde(rename = "@type")]
    pronoun_type: Option<String>,
    /// The pronouns value (e.g., "they/them").
    pub pronouns: String,
    /// Contexts in which to use the pronouns.
    pub contexts: Option<HashMap<String, bool>>,
    /// Preference of the pronouns relative to others.
    pub pref: Option<u32>,
}

/// Represents titles or roles of the entity, such as job titles or functional positions.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    /// The JSContact type of the object. Must be "Title".
    #[serde(rename = "@type")]
    title_type: Option<String>,
    /// The title or role name.
    pub name: String,
    /// The kind of title (e.g., title, role).
    pub kind: Option<TitleKind>,
    /// Identifier of the organization associated with this title.
    pub organization_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TitleKind {
    Title,
    Role,
}

/// Defines email addresses associated with the entity.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailAddress {
    /// The JSContact type of the object. Must be "EmailAddress".
    #[serde(rename = "@type")]
    email_type: Option<String>,
    /// The email address.
    pub address: String,
    /// Contexts in which to use the email address.
    pub contexts: Option<HashMap<String, bool>>,
    /// Preference of the email address relative to others.
    pub pref: Option<u32>,
    /// Custom label for the email address.
    pub label: Option<String>,
}

/// Represents online services such as social media or messaging accounts.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OnlineService {
    /// The JSContact type of the object. Must be "OnlineService".
    #[serde(rename = "@type")]
    service_type: Option<String>,
    /// The name of the online service or protocol.
    pub service: Option<String>,
    /// The URI identifying the entity on the service.
    pub uri: Option<String>,
    /// The username or handle on the online service.
    pub user: Option<String>,
    /// Contexts in which to use the online service.
    pub contexts: Option<HashMap<String, bool>>,
    /// Preference of the service relative to others.
    pub pref: Option<u32>,
    /// Custom label for the online service.
    pub label: Option<String>,
}

/// Defines phone numbers for the entity, including features like voice or text.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
    /// The JSContact type of the object. Must be "Phone".
    #[serde(rename = "@type")]
    phone_type: Option<String>,
    /// The phone number, either as a URI or free text.
    pub number: String,
    /// Contact features the phone number supports (e.g., voice, text).
    pub features: Option<HashMap<String, bool>>,
    /// Contexts in which to use the phone number.
    pub contexts: Option<HashMap<String, bool>>,
    /// Preference of the phone number relative to others.
    pub pref: Option<u32>,
    /// Custom label for the phone number.
    pub label: Option<String>,
}

/// Represents preferred languages for communication.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LanguagePref {
    /// The JSContact type of the object. Must be "LanguagePref".
    #[serde(rename = "@type")]
    lang_pref_type: Option<String>,
    /// The preferred language as a language tag (e.g., en, fr).
    pub language: String,
    /// Contexts in which to use the preferred language.
    pub contexts: Option<HashMap<String, bool>>,
    /// Preference of the language relative to others.
    pub pref: Option<u32>,
}

/// Represents memorable dates and events for the entity.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Anniversary {
    /// The JSContact type of the object. Must be "Anniversary".
    #[serde(rename = "@type")]
    anniversary_type: Option<String>,
    /// The date of the anniversary.
    pub date: DateObject,
    /// The type of anniversary (e.g., birthday, work anniversary).
    pub kind: String,
    /// Contexts in which to use the anniversary.
    pub contexts: Option<HashMap<String, bool>>,
    /// Preference of the anniversary relative to others.
    pub place: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DateObject {
    Timestamp(Timestamp),
    PartialDate(PartialDate),
}

/// Timestamp
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    /// The JSContact type of the object. The value MUST be "Timestamp", if set.
    timestamp_type: Option<String>,

    /// The point in time in UTC time
    pub utc: String,
}

/// A PartialDate object represents a complete or partial calendar date in the Gregorian calendar.  It represents a complete date, a year, a month in a year, or a day in a month.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartialDate {
    pub year: Option<u64>,
    pub month: Option<u32>,
    pub day: Option<u32>,

    /// The calendar system in which this date occurs, in lowercase.  This MUST be either a calendar system name registered as a Common Locale Data Repository (CLDR) [RFC7529] or a vendor-specific value.
    /// The year, month, and day still MUST be represented in the Gregorian calendar.
    /// Note that the year property might be required to convert the date between the Gregorian calendar and the respective calendar system.
    pub calendar_scale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    address_type: Option<String>,
    pub street: Option<String>,
    pub components: Option<Vec<AddressComponent>>,
    pub is_ordered: Option<bool>,
    pub country_code: Option<String>,
    pub coordinates: Option<String>,
    pub time_zone: Option<String>,
    pub contexts: Option<HashMap<String, bool>>,
    pub full: Option<String>,
    pub default_separator: Option<String>,
    pub pref: Option<u64>,
    pub phonetic_script: Option<String>,
    pub phonetic_system: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddressComponent {
    component_type: Option<String>,
    pub value: String,
    pub kind: AddressComponentKind,
    pub phonetic: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AddressComponentKind {
    Room,
    Apartment,
    Floor,
    Building,
    Number,
    Name,
    Block,
    Subdistrict,
    District,
    Locality,
    Region,
    Postcode,
    Country,
    Direction,
    Landmark,
    #[serde(rename = "postOfficeBox")]
    PostOfficeBox,
    Separator,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    note_type: Option<String>,
    pub note: String,
    pub created: Option<String>,
    pub author: Option<Author>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    author_type: Option<String>,
    pub name: Option<String>,
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInfo {
    ///The JSContact type of the object.  The value MUST be "PersonalInfo", if set.
    #[serde(rename = "@type")]
    personal_info_type: Option<String>,
    pub kind: String,
    pub value: String,
    pub level: Option<PersonalInfoLevel>,
    pub list_as: Option<u64>,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PersonalInfoLevel {
    High,
    Medium,
    Low,
}
