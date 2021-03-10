use crate::AppVersion;
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "resources/quickfix/"]
struct QuickFixDicts;

const SPEC_FIX_40: &str = include_str!("../resources/quickfix/FIX-4.0.xml");
const SPEC_FIX_41: &str = include_str!("../resources/quickfix/FIX-4.1.xml");
const SPEC_FIX_42: &str = include_str!("../resources/quickfix/FIX-4.2.xml");
const SPEC_FIX_43: &str = include_str!("../resources/quickfix/FIX-4.3.xml");
const SPEC_FIX_44: &str = include_str!("../resources/quickfix/FIX-4.4.xml");
const SPEC_FIX_50: &str = include_str!("../resources/quickfix/FIX-5.0.xml");
const SPEC_FIX_50SP1: &str = include_str!("../resources/quickfix/FIX-5.0-SP1.xml");
const SPEC_FIX_50SP2: &str = include_str!("../resources/quickfix/FIX-5.0-SP2.xml");
const SPEC_FIXT_11: &str = include_str!("../resources/quickfix/FIXT-1.1.xml");

/// Returns a [`String`] with the QuickFIX definition file for `self`
/// as its content. The QuickFix definition files are extracted and
/// decompressed
/// from the binary without filesystem access.
pub fn quickfix_spec(version: AppVersion) -> Cow<'static, str> {
    (match version {
        AppVersion::Fix40 => SPEC_FIX_40,
        AppVersion::Fix41 => SPEC_FIX_41,
        AppVersion::Fix42 => SPEC_FIX_42,
        AppVersion::Fix43 => SPEC_FIX_43,
        AppVersion::Fix44 => SPEC_FIX_44,
        AppVersion::Fix50 => SPEC_FIX_50,
        AppVersion::Fix50SP1 => SPEC_FIX_50SP1,
        AppVersion::Fix50SP2 => SPEC_FIX_50SP2,
        AppVersion::Fixt11 => SPEC_FIXT_11,
    })
    .into()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_versions_have_quickfix_spec() {
        assert!(AppVersion::all()
            .map(|version| quickfix_spec(version))
            .all(|spec| spec.len() > 0));
    }

    #[test]
    fn all_versions_have_different_quickfix_spec() {
        let mut set: HashSet<String> = HashSet::default();
        AppVersion::all()
            .map(|version| set.insert(quickfix_spec(version).to_string()))
            .count();
        assert_eq!(set.len(), AppVersion::all().count());
    }

    #[test]
    fn all_versions_have_xml_valid_quickfix_spec() {
        assert!(AppVersion::all()
            .map(|version| quickfix_spec(version))
            .all(|spec| roxmltree::Document::parse(&spec).is_ok()));
    }
}
