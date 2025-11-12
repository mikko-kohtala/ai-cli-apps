use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Kilo",
        InstallMethod::GitHub("Kilo-Org/kilocode".to_string()),
        vec!["kilo".to_string(), "--version".to_string()],
    )
    .with_binary_name("kilo")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("kilo", &["--version"]);
    ToolVersion::new("Kilo").with_installed(installed)
}
