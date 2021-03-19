use crate::dictionary::LayoutItemKind;
use crate::Dictionary;
use std::collections::HashSet;

/// A utility data structure that helps to parse FIX repeating groups.
#[derive(Debug, Clone)]
pub struct GroupDelimiter {
    group_items: HashSet<(String, u32, u32)>,
    entered_groups: Vec<u32>,
    current_msg_type: String,
}

impl GroupDelimiter {
    /// Crates a new [`GroupDelimiter`] following the specification of `dictionary`.
    pub fn new(dictionary: Dictionary) -> Self {
        let mut group_items = HashSet::new();
        for message in dictionary.iter_messages() {
            for item in message.layout() {
                match item.kind() {
                    LayoutItemKind::Component(component) if component.is_group() => {
                        for item in component.items() {
                            match item.kind() {
                                LayoutItemKind::Field(f) => {
                                    group_items.insert((
                                        message.msg_type().to_string(),
                                        component.id(),
                                        f.tag(),
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        Self {
            current_msg_type: "".to_string(),
            group_items,
            entered_groups: Vec::new(),
        }
    }

    /// Assumes for all subsequent operations that `self` is parsing `msg_type`.
    pub fn set_msg_type(&mut self, msg_type: &str) {
        self.current_msg_type = msg_type.to_string();
    }

    /// Notifies `self` that currently it's parsing a `group`.
    pub fn enter_group(&mut self, group: u32) {
        self.entered_groups.push(group);
    }

    /// Calculates wheather or not the next `tag` is part of the current group.
    pub fn is_outside_group(&self, tag: u32) -> bool {
        if let Some(group_tag) = self.entered_groups.last() {
            self.group_items
                .contains(&(self.current_msg_type.clone(), *group_tag, tag))
        } else {
            true
        }
    }

    pub fn debug(&self) {
        for (msg_type, group_tag, tag) in self.group_items.iter() {
            println!("{}: {} {}", msg_type, group_tag, tag);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::AppVersion;

    #[test]
    fn nomdentries() {
        let dict = Dictionary::from_version(AppVersion::Fix44);
        let mut group_delimiter = GroupDelimiter::new(dict);
        group_delimiter.debug();
        group_delimiter.set_msg_type("X");
        group_delimiter.enter_group(268);
        assert_eq!(group_delimiter.is_outside_group(278), false);
        assert_eq!(group_delimiter.is_outside_group(285), false);
        assert_eq!(group_delimiter.is_outside_group(8), true);
    }
}
