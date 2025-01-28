//! The Resource data type defines a resource associated with the entity represented by the Card, identified by a URI RFC3986.
//! Later in this document, several property definitions refer to the Resource type as the basis for their property-specific value types.
//! The Resource type defines the properties that are common to all of them.
//! Property definitions making use of Resource MAY define additional properties for their value types.

use crate::{
    Calendar, CalendarKind, Context, CryptoKey, Directory, DirectoryKind, Link, LinkKind, Media,
    MediaKind,
};
#[cfg(feature = "typed")]
use crate::{CalendarType, CryptoKeyType, DirectoryType, LinkType, MediaType};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Resource data type defines a resource associated with the entity represented by the Card
/// Resource is exposed for utility purposes.
/// It is used to define common properties for all resources.
/// It is not intended to be used directly.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// The JSContact type of the object.
    #[cfg(feature = "typed")]
    #[serde(rename = "@type")]
    resource_type: Option<ResourceType>,
    /// The kind of the resource.
    pub kind: Option<String>,
    /// The resource value.
    pub uri: String,
    /// The media type RFC2046 of the resource identified by the uri property value.
    pub media_type: Option<String>,
    /// The contexts in which to use this resource.
    pub contexts: Option<HashMap<Context, bool>>,
    /// The preference of the resource in relation to other resources.
    pub pref: Option<u64>,
    /// A custom label for the value.
    pub label: Option<String>,
}

impl Resource {
    /// Create a new Resource
    pub fn new(uri: String) -> Self {
        Self {
            #[cfg(feature = "typed")]
            resource_type: Some(ResourceType::Resource),
            kind: None,
            uri,
            media_type: None,
            contexts: None,
            pref: None,
            label: None,
        }
    }
}

/// Resource @type
#[cfg(feature = "typed")]
#[derive(Serialize, Deserialize, Debug)]
enum ResourceType {
    /// Resource @type
    Resource,
}

impl From<Resource> for Calendar {
    fn from(resource: Resource) -> Self {
        let kind: Option<CalendarKind> = resource.kind.as_deref().map(|s| s.to_string().into());
        Self {
            #[cfg(feature = "typed")]
            calendar_type: Some(CalendarType::Calendar),
            kind,
            uri: resource.uri,
            media_type: resource.media_type,
            contexts: resource.contexts,
            pref: resource.pref,
            label: resource.label,
        }
    }
}

impl From<Resource> for CryptoKey {
    fn from(resource: Resource) -> Self {
        Self {
            #[cfg(feature = "typed")]
            crypto_key_type: Some(CryptoKeyType::CryptoKey),
            kind: resource.kind,
            uri: resource.uri,
            media_type: resource.media_type,
            contexts: resource.contexts,
            pref: resource.pref,
            label: resource.label,
        }
    }
}

impl From<Resource> for Directory {
    fn from(resource: Resource) -> Self {
        let kind: Option<DirectoryKind> = resource.kind.as_deref().map(|s| s.to_string().into());
        Self {
            #[cfg(feature = "typed")]
            directory_type: Some(DirectoryType::Directory),
            kind,
            uri: resource.uri,
            media_type: resource.media_type,
            contexts: resource.contexts,
            pref: resource.pref,
            label: resource.label,
            list_as: None,
        }
    }
}

impl From<Resource> for Media {
    fn from(resource: Resource) -> Self {
        let kind: MediaKind = match resource.kind {
            Some(kind) => kind.into(),
            None => MediaKind::default(),
        };
        Self {
            #[cfg(feature = "typed")]
            media_hidden_type: Some(MediaType::Media),
            kind,
            uri: resource.uri,
            media_type: resource.media_type,
            contexts: resource.contexts,
            pref: resource.pref,
            label: resource.label,
        }
    }
}

impl From<Resource> for Link {
    fn from(resource: Resource) -> Self {
        let kind: Option<LinkKind> = resource.kind.as_deref().map(|s| s.to_string().into());
        Self {
            #[cfg(feature = "typed")]
            link_type: Some(LinkType::Link),
            kind,
            uri: resource.uri,
            media_type: resource.media_type,
            contexts: resource.contexts,
            pref: resource.pref,
            label: resource.label,
        }
    }
}
