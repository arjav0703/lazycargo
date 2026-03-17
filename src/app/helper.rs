#[derive(Debug, Clone, PartialEq)]
pub enum Panel {
    Sidebar,
    Main,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SidebarSection {
    Info,
    Dependencies,
    Commands,
}

impl SidebarSection {
    pub fn all() -> Vec<SidebarSection> {
        vec![
            SidebarSection::Info,
            SidebarSection::Dependencies,
            SidebarSection::Commands,
        ]
    }

    pub fn label(&self) -> &str {
        match self {
            SidebarSection::Info => "[1] Package Info",
            SidebarSection::Dependencies => "[2] Dependencies",
            SidebarSection::Commands => "[3] Execute Commands",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PopupMode {
    None,
    AddDependency,
    RemoveConfirm(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandStatus {
    Idle,
    Running,
    Success,
    Failed,
}
