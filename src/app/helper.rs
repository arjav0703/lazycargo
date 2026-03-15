#[derive(Debug, Clone, PartialEq)]
pub enum Panel {
    Sidebar,
    Main,
    Output,
}

#[derive(Debug, Clone, PartialEq)]
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
            SidebarSection::Info => "Package Info",
            SidebarSection::Dependencies => "Dependencies",
            SidebarSection::Commands => "Commands",
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
