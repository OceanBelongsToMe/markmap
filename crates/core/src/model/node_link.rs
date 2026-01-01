use super::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LinkType {
    Inline,
    Reference,
    ReferenceUnknown,
    Collapsed,
    CollapsedUnknown,
    Shortcut,
    ShortcutUnknown,
    Autolink,
    Email,
    WikiLink { has_pothole: bool },
}

impl LinkType {
    pub fn from_str(value: &str) -> Option<Self> {
        if let Some(rest) = value.strip_prefix("WikiLink:") {
            let has_pothole = matches!(rest, "true" | "1");
            return Some(LinkType::WikiLink { has_pothole });
        }
        if value == "WikiLink" {
            return Some(LinkType::WikiLink { has_pothole: false });
        }
        match value {
            "Inline" => Some(LinkType::Inline),
            "Reference" => Some(LinkType::Reference),
            "ReferenceUnknown" => Some(LinkType::ReferenceUnknown),
            "Collapsed" => Some(LinkType::Collapsed),
            "CollapsedUnknown" => Some(LinkType::CollapsedUnknown),
            "Shortcut" => Some(LinkType::Shortcut),
            "ShortcutUnknown" => Some(LinkType::ShortcutUnknown),
            "Autolink" => Some(LinkType::Autolink),
            "Email" => Some(LinkType::Email),
            _ => None,
        }
    }
}

impl std::fmt::Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkType::Inline => f.write_str("Inline"),
            LinkType::Reference => f.write_str("Reference"),
            LinkType::ReferenceUnknown => f.write_str("ReferenceUnknown"),
            LinkType::Collapsed => f.write_str("Collapsed"),
            LinkType::CollapsedUnknown => f.write_str("CollapsedUnknown"),
            LinkType::Shortcut => f.write_str("Shortcut"),
            LinkType::ShortcutUnknown => f.write_str("ShortcutUnknown"),
            LinkType::Autolink => f.write_str("Autolink"),
            LinkType::Email => f.write_str("Email"),
            LinkType::WikiLink { has_pothole } => {
                write!(f, "WikiLink:{has_pothole}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeLink {
    pub node_id: NodeId,
    pub href: String,
    pub title: Option<String>,
    pub link_type: LinkType,
    pub ref_id: Option<String>,
}
