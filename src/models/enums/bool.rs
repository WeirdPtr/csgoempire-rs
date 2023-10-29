#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CSGOEmpireBool {
    Yes,
    No,
}

impl CSGOEmpireBool {
    pub fn to_string(&self) -> String {
        match self {
            CSGOEmpireBool::Yes => "yes".to_string(),
            CSGOEmpireBool::No => "no".to_string(),
        }
    }
}

impl std::fmt::Display for CSGOEmpireBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSGOEmpireBool::Yes => write!(f, "yes"),
            CSGOEmpireBool::No => write!(f, "no"),
        }
    }
}

impl From<bool> for CSGOEmpireBool {
    fn from(value: bool) -> Self {
        if value {
            CSGOEmpireBool::Yes
        } else {
            CSGOEmpireBool::No
        }
    }
}

impl From<CSGOEmpireBool> for String {
    fn from(value: CSGOEmpireBool) -> Self {
        value.to_string()
    }
}

impl From<CSGOEmpireBool> for bool {
    fn from(value: CSGOEmpireBool) -> Self {
        match value {
            CSGOEmpireBool::Yes => true,
            CSGOEmpireBool::No => false,
        }
    }
}
