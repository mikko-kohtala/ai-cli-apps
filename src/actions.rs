use crate::tools::{self, InstallMethod, Tool};
use anyhow::{Context, Result};
use colored::*;
use inquire::MultiSelect;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

pub async fn handle_install_command(tool_name: Option<&str>) -> Result<()> {
    let tools = tools::catalog();

    if let Some(name) = tool_name {
        let tool = find_tool(&tools, name).with_context(|| {
            format!(
                "Tool '{}' not found. Available tools: {}",
                name,
                format_available_tools(&tools)
            )
        })?;

        if tool.is_installed() {
            println!("{} {} is already installed!", "✓".green(), tool.name);
            return Ok(());
        }

        install_tool(tool).await?;
        return Ok(());
    }

    let mut uninstalled_tools: Vec<&Tool> = tools.iter().filter(|t| !t.is_installed()).collect();
    let installed_tools: Vec<&Tool> = tools.iter().filter(|t| t.is_installed()).collect();

    if uninstalled_tools.is_empty() {
        println!("{}", "All tools are already installed! ✓".green());
        return Ok(());
    }

    uninstalled_tools.sort_by(|a, b| a.name.cmp(&b.name));

    println!("{}", "\nSelect tools to install:".bright_cyan().bold());

    let options: Vec<String> = uninstalled_tools
        .iter()
        .map(|t| {
            format!(
                "{} ({})",
                t.name,
                match &t.install_method {
                    InstallMethod::Npm(pkg) => format!("npm: {}", pkg),
                    InstallMethod::GitHub(repo) => format!("github: {}", repo),
                    InstallMethod::Bootstrap(_) => "bootstrap".to_string(),
                    InstallMethod::Amp(_) => "amp installer".to_string(),
                    InstallMethod::Custom(_) => "custom".to_string(),
                }
            )
        })
        .collect();

    if !installed_tools.is_empty() {
        println!("\n{}", "Already installed:".bright_black());
        for tool in &installed_tools {
            println!("  {} {}", "✓".green(), tool.name.bright_black());
        }
        println!();
    }

    let selected = MultiSelect::new("Tools:", options)
        .with_help_message("↑↓ to move, space to select, enter to confirm")
        .prompt();

    match selected {
        Ok(selections) if !selections.is_empty() => {
            println!("\n{}", "Starting installation...".bright_cyan());

            for selection in selections {
                if let Some(tool) = uninstalled_tools
                    .iter()
                    .find(|t| selection.starts_with(&t.name))
                {
                    if let Err(e) = install_tool(tool).await {
                        println!("{} Failed to install {}: {}", "✗".red(), tool.name, e);
                    }
                }
            }

            println!("\n{}", "Installation complete!".green().bold());
        }
        Ok(_) => println!("{}", "No tools selected.".yellow()),
        Err(e) => println!("{} Selection cancelled: {}", "✗".red(), e),
    }

    Ok(())
}

pub async fn handle_uninstall_command(
    tool_name: Option<&str>,
    remove_config: bool,
    force: bool,
) -> Result<()> {
    let tools = tools::catalog();

    if let Some(name) = tool_name {
        let tool = find_tool(&tools, name).with_context(|| {
            format!(
                "Tool '{}' not found. Available tools: {}",
                name,
                format_available_tools(&tools)
            )
        })?;

        if !tool.is_installed() {
            println!("{} {} is not installed!", "!".yellow(), tool.name);
            return Ok(());
        }

        uninstall_tool(tool, remove_config, force).await?;
        return Ok(());
    }

    let mut installed_tools: Vec<&Tool> = tools.iter().filter(|t| t.is_installed()).collect();

    if installed_tools.is_empty() {
        println!("{}", "No tools are currently installed.".yellow());
        return Ok(());
    }

    installed_tools.sort_by(|a, b| a.name.cmp(&b.name));

    println!("{}", "\nSelect tools to uninstall:".bright_cyan().bold());

    let options: Vec<String> = installed_tools.iter().map(|t| t.name.clone()).collect();

    let selected = MultiSelect::new("Tools:", options)
        .with_help_message("↑↓ to move, space to select, enter to confirm")
        .prompt();

    match selected {
        Ok(selections) if !selections.is_empty() => {
            println!("\n{}", "Starting uninstallation...".bright_cyan());

            for selection in selections {
                if let Some(tool) = installed_tools.iter().find(|t| t.name == selection) {
                    if let Err(e) = uninstall_tool(tool, remove_config, force).await {
                        println!("{} Failed to uninstall {}: {}", "✗".red(), tool.name, e);
                    }
                }
            }

            println!("\n{}", "Uninstallation complete!".green().bold());
        }
        Ok(_) => println!("{}", "No tools selected.".yellow()),
        Err(e) => println!("{} Selection cancelled: {}", "✗".red(), e),
    }

    Ok(())
}

pub async fn handle_upgrade_command(tool_name: Option<&str>) -> Result<()> {
    let tools = tools::catalog();

    let Some(name) = tool_name else {
        println!(
            "{} Specify a tool to upgrade, e.g., `ai-cli-apps upgrade amp`.",
            "!".yellow()
        );
        return Ok(());
    };

    let tool = find_tool(&tools, name).with_context(|| {
        format!(
            "Tool '{}' not found. Available tools: {}",
            name,
            format_available_tools(&tools)
        )
    })?;

    if !tool.is_installed() {
        println!(
            "{} {} is not installed. Run `ai-cli-apps install {}` first.",
            "!".yellow(),
            tool.name,
            name
        );
        return Ok(());
    }

    upgrade_tool(tool).await
}

async fn install_tool(tool: &Tool) -> Result<()> {
    println!("Installing {}...", tool.name.bright_cyan());

    match &tool.install_method {
        InstallMethod::Bootstrap(url) => {
            run_install_script(url, "bootstrap.sh", "bootstrap script").await?;
            println!("{} {} installed successfully!", "✓".green(), tool.name);
        }
        InstallMethod::Amp(url) => {
            run_install_script(url, "amp_install.sh", "Amp installer").await?;
            println!("{} {} installed successfully!", "✓".green(), tool.name);
        }
        InstallMethod::Npm(package) => {
            let status = Command::new("npm")
                .args(&["install", "-g", package])
                .status()
                .context("Failed to run npm install")?;

            if status.success() {
                println!("{} {} installed successfully!", "✓".green(), tool.name);
            } else {
                anyhow::bail!("npm install failed for {}", tool.name);
            }
        }
        InstallMethod::GitHub(repo) => {
            println!("{} Fetching latest release from GitHub...", "→".cyan());

            let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
            let client = reqwest::Client::new();
            let response = client
                .get(&url)
                .header("User-Agent", "ai-cli-apps")
                .send()
                .await
                .context("Failed to fetch GitHub release")?;

            let release: serde_json::Value = response.json().await?;
            let tag_name = release["tag_name"]
                .as_str()
                .context("No tag_name in release")?;

            let assets = release["assets"]
                .as_array()
                .context("No assets in release")?;

            let os_keywords = ["darwin", "macos", "mac", "universal"];
            let binary_asset = assets.iter().find(|asset| {
                if let Some(name) = asset["name"].as_str() {
                    let name_lower = name.to_lowercase();
                    os_keywords
                        .iter()
                        .any(|keyword| name_lower.contains(keyword))
                        && !name_lower.ends_with(".sha256")
                        && !name_lower.ends_with(".txt")
                } else {
                    false
                }
            });

            if let Some(asset) = binary_asset {
                let download_url = asset["browser_download_url"]
                    .as_str()
                    .context("No download URL")?;
                let asset_name = asset["name"].as_str().unwrap_or("binary");

                println!("{} Downloading {}...", "→".cyan(), asset_name);

                let binary_data = reqwest::get(download_url).await?.bytes().await?;

                let install_dir = Path::new("/usr/local/bin");
                let binary_name = tool
                    .binary_name
                    .as_deref()
                    .unwrap_or_else(|| tool.name.as_str());
                let install_path = install_dir.join(binary_name);

                fs::write(&install_path, &binary_data).context("Failed to write binary")?;

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&install_path)?.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&install_path, perms)?;
                }

                println!(
                    "{} {} installed successfully to {}!",
                    "✓".green(),
                    tool.name,
                    install_path.display()
                );
            } else {
                println!("{} No suitable binary found for your platform.", "✗".red());
                println!(
                    "Please visit https://github.com/{}/releases/tag/{}",
                    repo, tag_name
                );
                anyhow::bail!("No binary available for platform");
            }
        }
        InstallMethod::Custom(message) => {
            println!("{}", message.yellow());
            anyhow::bail!("Custom installation required");
        }
    }

    Ok(())
}

async fn uninstall_tool(tool: &Tool, remove_config: bool, force: bool) -> Result<()> {
    println!("Uninstalling {}...", tool.name.bright_cyan());

    match &tool.install_method {
        InstallMethod::Bootstrap(_) => {
            let home = std::env::var("HOME").context("HOME environment variable not set")?;
            let binary_name = tool
                .binary_name
                .as_deref()
                .unwrap_or_else(|| tool.name.as_str());

            let symlink_path = Path::new(&home)
                .join(".local")
                .join("bin")
                .join(binary_name);

            let versions_path = Path::new(&home)
                .join(".local")
                .join("share")
                .join(binary_name)
                .join("versions");
            let config_path = Path::new(&home).join(format!(".{}", binary_name));

            let mut removed_items = Vec::new();

            if symlink_path.exists() {
                fs::remove_file(&symlink_path).context("Failed to remove binary symlink")?;
                removed_items.push(format!("binary: {}", symlink_path.display()));
            }

            if versions_path.exists() {
                if let Some(parent) = versions_path.parent() {
                    fs::remove_dir_all(parent).context("Failed to remove versions directory")?;
                    removed_items.push(format!("versions: {}", parent.display()));
                }
            }

            if config_path.exists() {
                println!(
                    "{} Config directory found at: {}",
                    "→".cyan(),
                    config_path.display()
                );

                if remove_config {
                    let should_remove = if force {
                        true
                    } else {
                        println!(
                            "{} Remove config directory? (contains settings and history) [y/N]",
                            "?".yellow()
                        );
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        input.trim().eq_ignore_ascii_case("y")
                    };

                    if should_remove {
                        fs::remove_dir_all(&config_path)
                            .context("Failed to remove config directory")?;
                        removed_items.push(format!("config: {}", config_path.display()));
                    } else {
                        println!("{} Keeping config directory", "→".cyan());
                    }
                } else {
                    println!(
                        "{} Keeping config directory (use --remove-config to remove it)",
                        "→".cyan()
                    );
                }
            }

            if removed_items.is_empty() {
                println!("{} {} not found on system", "!".yellow(), tool.name);
            } else {
                println!("{} {} uninstalled successfully!", "✓".green(), tool.name);
                println!("{} Removed:", "→".cyan());
                for item in removed_items {
                    println!("  - {}", item);
                }
            }
        }
        InstallMethod::Amp(_) => {
            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .context("HOME environment variable not set")?;
            let home_path = Path::new(&home);
            let amp_home = home_path.join(".amp");
            let local_bin = home_path.join(".local").join("bin");
            let mut removed_items = Vec::new();

            for shim in ["amp", "amp.bat"] {
                let shim_path = local_bin.join(shim);
                if shim_path.exists() {
                    fs::remove_file(&shim_path)
                        .with_context(|| format!("Failed to remove {}", shim_path.display()))?;
                    removed_items.push(format!("shim: {}", shim_path.display()));
                }
            }

            if amp_home.exists() {
                fs::remove_dir_all(&amp_home).context("Failed to remove AMP_HOME directory")?;
                removed_items.push(format!("AMP_HOME: {}", amp_home.display()));
            }

            let config_home = std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_path.join(".config"));
            let data_home = std::env::var("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_path.join(".local").join("share"));
            let cache_home = std::env::var("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_path.join(".cache"));

            if remove_config {
                let should_remove = if force {
                    true
                } else {
                    println!(
                        "{} Remove Amp config/cache directories? [y/N]",
                        "?".yellow()
                    );
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    input.trim().eq_ignore_ascii_case("y")
                };

                if should_remove {
                    for path in [
                        config_home.join("amp"),
                        data_home.join("amp"),
                        cache_home.join("amp"),
                    ] {
                        if path.exists() {
                            let metadata = fs::metadata(&path)?;
                            if metadata.is_file() {
                                fs::remove_file(&path).with_context(|| {
                                    format!("Failed to remove {}", path.display())
                                })?;
                            } else {
                                fs::remove_dir_all(&path).with_context(|| {
                                    format!("Failed to remove {}", path.display())
                                })?;
                            }
                            removed_items.push(format!("config/data/cache: {}", path.display()));
                        }
                    }
                } else {
                    println!("{} Keeping Amp config/cache directories", "→".cyan());
                }
            } else {
                println!(
                    "{} Keeping Amp config/cache directories (use --remove-config to delete them)",
                    "→".cyan()
                );
            }

            if removed_items.is_empty() {
                println!("{} Amp files not found on system", "!".yellow());
            } else {
                println!("{} {} uninstalled successfully!", "✓".green(), tool.name);
                println!("{} Removed:", "→".cyan());
                for item in removed_items {
                    println!("  - {}", item);
                }
                println!(
                    "{} Remove any PATH entries for ~/.local/bin/amp in your shell rc files.",
                    "→".cyan()
                );
            }
        }
        InstallMethod::Npm(package) => {
            let status = Command::new("npm")
                .args(&["uninstall", "-g", package])
                .status()
                .context("Failed to run npm uninstall")?;

            if status.success() {
                println!("{} {} uninstalled successfully!", "✓".green(), tool.name);
            } else {
                anyhow::bail!("npm uninstall failed for {}", tool.name);
            }
        }
        InstallMethod::GitHub(_) => {
            let binary_name = tool
                .binary_name
                .as_deref()
                .unwrap_or_else(|| tool.name.as_str());
            let install_path = Path::new("/usr/local/bin").join(binary_name);

            if install_path.exists() {
                fs::remove_file(&install_path).context("Failed to remove binary")?;
                println!("{} {} uninstalled successfully!", "✓".green(), tool.name);
            } else {
                println!(
                    "{} {} binary not found at {}",
                    "!".yellow(),
                    tool.name,
                    install_path.display()
                );
            }
        }
        InstallMethod::Custom(_) => {
            println!(
                "{} {} requires manual uninstallation",
                "!".yellow(),
                tool.name
            );
            println!("Please remove it manually from your system");
        }
    }

    Ok(())
}

async fn upgrade_tool(tool: &Tool) -> Result<()> {
    println!("Upgrading {}...", tool.name.bright_cyan());

    match &tool.install_method {
        InstallMethod::Amp(_) => {
            println!("{} Running `amp update`...", "→".cyan());
            let status = Command::new("amp")
                .arg("update")
                .status()
                .context("Failed to run `amp update`")?;

            if status.success() {
                println!("{} {} upgraded successfully!", "✓".green(), tool.name);
                Ok(())
            } else {
                anyhow::bail!("`amp update` failed - see output above for details");
            }
        }
        _ => anyhow::bail!("Upgrade not implemented for {}", tool.name),
    }
}

async fn run_install_script(url: &str, temp_filename: &str, description: &str) -> Result<()> {
    println!("{} Downloading {}...", "→".cyan(), description);

    let script = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to download {}", description))?
        .text()
        .await
        .with_context(|| format!("Failed to read {}", description))?;

    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join(temp_filename);
    fs::write(&script_path, script).with_context(|| format!("Failed to write {}", description))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    println!("{} Running {}...", "→".cyan(), description);
    println!();

    let status = Command::new("bash")
        .arg(&script_path)
        .status()
        .context("Failed to run install script")?;

    let _ = fs::remove_file(&script_path);

    println!();
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("Installation failed - see output above for details");
    }
}

fn format_available_tools(tools: &[Tool]) -> String {
    tools
        .iter()
        .map(|t| {
            if let Some(bin) = &t.binary_name {
                format!("{} ({})", t.name, bin)
            } else {
                t.name.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn find_tool<'a>(tools: &'a [Tool], name: &str) -> Option<&'a Tool> {
    tools.iter().find(|t| {
        t.name.eq_ignore_ascii_case(name)
            || t.binary_name
                .as_ref()
                .map(|b| b.eq_ignore_ascii_case(name))
                .unwrap_or(false)
    })
}
