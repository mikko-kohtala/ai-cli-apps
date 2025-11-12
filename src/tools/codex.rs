use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Codex",
        InstallMethod::Npm("@openai/codex".to_string()),
        vec!["codex".to_string(), "--version".to_string()],
    )
    .with_binary_name("codex")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("codex", &["--version"]);
    ToolVersion::new("Codex").with_installed(installed)
}
