use std::collections::HashMap;

use colored::*;
use futures::future::join_all;
use serde::Deserialize;

use crate::tools::ToolVersion;

#[derive(Deserialize)]
struct NpmPackageInfo {
    version: String,
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

async fn get_npm_latest(package: &str) -> Option<String> {
    let url = format!("https://registry.npmjs.org/{}", package);
    let response = reqwest::get(&url).await.ok()?;
    let info: NpmPackageInfo = response.json().await.ok()?;
    Some(info.version)
}

async fn get_github_latest(repo: &str) -> Option<String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "ai-cli-apps")
        .send()
        .await
        .ok()?;
    let release: GitHubRelease = response.json().await.ok()?;
    Some(release.tag_name)
}

pub async fn check_latest_versions(tools: &mut [ToolVersion]) {
    println!("{}", "Checking latest versions...".cyan());

    let sources = vec![
        (
            "Claude Code",
            tokio::spawn(get_github_latest("anthropics/anthropic-quickstarts")),
        ),
        ("Codex", tokio::spawn(get_npm_latest("@openai/codex"))),
        ("Copilot", tokio::spawn(get_npm_latest("@github/copilot"))),
        ("Gemini", tokio::spawn(get_npm_latest("@google/gemini-cli"))),
        ("Cline", tokio::spawn(get_github_latest("cline/cline"))),
        ("Kilo", tokio::spawn(get_github_latest("Kilo-Org/kilocode"))),
    ];

    let resolved = join_all(
        sources
            .into_iter()
            .map(|(name, handle)| async move { (name, handle.await.ok().and_then(|r| r)) }),
    )
    .await;

    let latest_map: HashMap<_, _> = resolved.into_iter().collect();

    for tool in tools.iter_mut() {
        if let Some(latest) = latest_map.get(tool.name.as_str()) {
            tool.latest = latest.clone();
        }
    }
}

pub fn print_version(tool: &ToolVersion, check_latest: bool) {
    let status = match &tool.installed {
        Some(version) => {
            let version_str = version.to_string();
            if check_latest {
                if let Some(latest) = &tool.latest {
                    if version.contains(latest) || latest.contains(version) {
                        format!("{} ✓", version_str.green())
                    } else {
                        format!(
                            "{} → {} available",
                            version_str.yellow(),
                            latest.bright_blue()
                        )
                    }
                } else {
                    version_str.green().to_string()
                }
            } else {
                version_str.green().to_string()
            }
        }
        None => {
            if check_latest && tool.latest.is_some() {
                format!(
                    "{} ({})",
                    "not installed".red(),
                    tool.latest.as_ref().unwrap().bright_blue()
                )
            } else {
                "not installed".red().to_string()
            }
        }
    };

    println!("{:12} {}", format!("{}:", tool.name).bold(), status);
}
