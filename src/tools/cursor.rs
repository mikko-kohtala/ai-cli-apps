use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Cursor",
        InstallMethod::Custom("Visit https://cursor.sh to download".to_string()),
        vec![
            "defaults".to_string(),
            "read".to_string(),
            "/Applications/Cursor.app/Contents/Info.plist".to_string(),
            "CFBundleShortVersionString".to_string(),
        ],
    )
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output(
        "defaults",
        &[
            "read",
            "/Applications/Cursor.app/Contents/Info.plist",
            "CFBundleShortVersionString",
        ],
    );
    ToolVersion::new("Cursor").with_installed(installed)
}
