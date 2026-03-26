//! Canonical `Person` JSON-LD node for the site author.

use kernel::seo::{
    AUTHOR_ALUMNI, AUTHOR_AWARDS, AUTHOR_BIO, AUTHOR_CREDENTIALS, AUTHOR_EMAIL, AUTHOR_FAMILY_NAME,
    AUTHOR_GIVEN_NAME, AUTHOR_IMAGE, AUTHOR_JOB_TITLE, AUTHOR_KNOWS_ABOUT, AUTHOR_NAME,
    AUTHOR_OCCUPATIONS, AUTHOR_SAME_AS, SITE_URL,
};
use serde_json::{Value, json};

/// The single canonical `Person` node representing the site author.
///
/// Injected into every JSON-LD object so page components never repeat
/// author information. Includes `image` for Google Knowledge Panel,
/// structured career history, education, credentials, and awards.
pub fn author_node() -> Value {
    let alumni_of: Vec<Value> = AUTHOR_ALUMNI
        .iter()
        .map(|(name, url, typ)| {
            json!({
                "@type": *typ,
                "name": *name,
                "url": *url,
            })
        })
        .collect();

    let has_occupation: Vec<Value> = AUTHOR_OCCUPATIONS
        .iter()
        .map(|(role, org, start, end, desc)| {
            json!({
                "@type": "OrganizationRole",
                "roleName": *role,
                "startDate": *start,
                "endDate": *end,
                "description": *desc,
                "memberOf": {
                    "@type": "Organization",
                    "name": *org,
                },
            })
        })
        .collect();

    let has_credential: Vec<Value> = AUTHOR_CREDENTIALS
        .iter()
        .map(|(name, institution, year)| {
            json!({
                "@type": "EducationalOccupationalCredential",
                "credentialCategory": "degree",
                "name": *name,
                "recognizedBy": {
                    "@type": "EducationalOrganization",
                    "name": *institution,
                },
                "dateCreated": *year,
            })
        })
        .collect();

    json!({
        "@type": "Person",
        "@id": format!("{SITE_URL}/#person"),
        "name": AUTHOR_NAME,
        "givenName": AUTHOR_GIVEN_NAME,
        "familyName": AUTHOR_FAMILY_NAME,
        "description": AUTHOR_BIO,
        "jobTitle": AUTHOR_JOB_TITLE,
        "url": SITE_URL,
        "image": AUTHOR_IMAGE,
        "email": AUTHOR_EMAIL,
        "sameAs": AUTHOR_SAME_AS,
        "knowsAbout": AUTHOR_KNOWS_ABOUT,
        "nationality": {
            "@type": "Country",
            "name": "France",
        },
        "knowsLanguage": ["French", "English"],
        "address": {
            "@type": "PostalAddress",
            "addressLocality": "Niort",
            "addressRegion": "Nouvelle-Aquitaine",
            "addressCountry": "FR",
        },
        "alumniOf": alumni_of,
        "hasOccupation": has_occupation,
        "hasCredential": has_credential,
        "award": AUTHOR_AWARDS,
    })
}

/// Compact `Person` reference with only identifying fields.
///
/// Use this in article or project `author` slots where embedding the
/// full node would be excessive — consumers can follow `@id` to
/// resolve the complete person.
pub fn author_node_compact() -> Value {
    json!({
        "@type": "Person",
        "@id": format!("{SITE_URL}/#person"),
        "name": AUTHOR_NAME,
        "url": SITE_URL,
        "image": AUTHOR_IMAGE,
    })
}
