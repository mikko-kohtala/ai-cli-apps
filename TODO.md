# Tool Support Status

This document tracks the implementation status for each AI CLI tool across different operations.

**⚠️ Note: Currently supports macOS only**

## Legend

- ✅ Implemented
- ⚠️ Partial/Needs work
- ❌ Not implemented
- 🔍 Research needed

## Amp

**Documentation**: [Docs](https://ampcode.com/manual)

| Operation                | Status         | Method                                               |
| ------------------------ | -------------- | ---------------------------------------------------- |
| Version Check            | ✅ Implemented | `amp --version`                                      |
| Current Version          | ✅ Implemented | Parse CLI output                                     |
| Latest Available Version | ⚠️ Partial     | Fetched from npm registry (no API for release notes) |
| Install                  | ✅ Implemented | Official bootstrap script                            |
| Uninstall                | ✅ Implemented | Remove ~/.amp + XDG config                           |
| Upgrade                  | ✅ Implemented | `amp update`                                         |

## Claude Code

**Documentation**: [Docs](https://code.claude.com/docs)

| Operation                | Status         | Method                                            |
| ------------------------ | -------------- | ------------------------------------------------- |
| Version Check            | ✅ Implemented | `claude --version`                                |
| Current Version          | ✅ Implemented | Parse CLI output                                  |
| Latest Available Version | ✅ Implemented | GitHub releases                                   |
| Install                  | ✅ Implemented | `curl -fsSL https://claude.ai/install.sh \| bash` |
| Uninstall                | ✅ Implemented | Removes binary, versions, and config (optional)   |
| Upgrade                  | ✅ Implemented | Re-install via bootstrap script                   |

## Codex CLI

**Documentation**: [Docs](https://developers.openai.com/codex/cli/)

| Operation                | Status         | Method                      |
| ------------------------ | -------------- | --------------------------- |
| Version Check            | ✅ Implemented | `codex --version`           |
| Current Version          | ✅ Implemented | Parse CLI output            |
| Latest Available Version | ✅ Implemented | `brew info --json=v2 codex` |
| Install                  | ✅ Implemented | `brew install codex`        |
| Uninstall                | ✅ Implemented | `brew uninstall codex`      |
| Upgrade                  | ✅ Implemented | `brew upgrade codex`        |

## Cursor CLI

**Documentation**: [Docs](https://cursor.com/docs/cli/overview)

| Operation                | Status             | Method                   |
| ------------------------ | ------------------ | ------------------------ |
| Version Check            | ✅ Implemented     | `cursor-agent --version` |
| Current Version          | ✅ Implemented     | Parse CLI output         |
| Latest Available Version | ✅ Implemented     | Vendor download metadata |
| Install                  | ✅ Implemented     | `curl https://cursor.com/install -fsS \| bash` |
| Uninstall                | ✅ Implemented     | Removes `~/.local/bin/cursor-agent` + `~/.local/share/cursor-agent` |
| Upgrade                  | ✅ Implemented     | `cursor-agent upgrade`   |

## Copilot CLI

**Documentation**: [Docs](https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli)

| Operation                | Status             | Method              |
| ------------------------ | ------------------ | ------------------- |
| Version Check            | ✅ Implemented     | `copilot --version` |
| Current Version          | ✅ Implemented     | Parse CLI output    |
| Latest Available Version | ✅ Implemented     | npm registry        |
| Install                  | ✅ Implemented     | `npm install -g @github/copilot` |
| Uninstall                | ✅ Implemented     | `npm uninstall -g @github/copilot` |
| Upgrade                  | ✅ Implemented     | `npm install -g @github/copilot` |

## Kilo Code CLI

**Documentation**: [Docs](https://kilocode.ai/docs/cli)

| Operation                | Status             | Method           |
| ------------------------ | ------------------ | ---------------- |
| Version Check            | ✅ Implemented     | `kilo --version` |
| Current Version          | ✅ Implemented     | Parse CLI output |
| Latest Available Version | ✅ Implemented     | npm registry     |
| Install                  | ✅ Implemented     | `npm install -g @kilocode/cli` |
| Uninstall                | ✅ Implemented     | `npm uninstall -g @kilocode/cli` |
| Upgrade                  | ✅ Implemented     | `npm install -g @kilocode/cli` |

## Gemini CLI

**Documentation**: [Docs](https://docs.cloud.google.com/gemini/docs/codeassist/gemini-cli)

| Operation                | Status         | Method                           |
| ------------------------ | -------------- | -------------------------------- |
| Version Check            | ✅ Implemented | `gemini --version`               |
| Current Version          | ✅ Implemented | Parse CLI output                 |
| Latest Available Version | ✅ Implemented | `brew info --json=v2 gemini-cli` |
| Install                  | ✅ Implemented | `brew install gemini-cli`        |
| Uninstall                | ✅ Implemented | `brew uninstall gemini-cli`      |
| Upgrade                  | ✅ Implemented | `brew upgrade gemini-cli`        |

## Cline CLI

**Documentation**: [Docs](https://docs.cline.bot/cline-cli/overview)

| Operation                | Status             | Method           |
| ------------------------ | ------------------ | ---------------- |
| Version Check            | ✅ Implemented     | `cline version`  |
| Current Version          | ✅ Implemented     | Parse CLI output |
| Latest Available Version | ✅ Implemented     | npm registry     |
| Install                  | ✅ Implemented     | `npm install -g cline` |
| Uninstall                | ✅ Implemented     | `npm uninstall -g cline` |
| Upgrade                  | ✅ Implemented     | `npm install -g cline` |

## Next Steps

1. ✅ ~~Research Cursor CLI (`cursor-agent`) - verify version check, install, uninstall, upgrade commands~~ (Done)
2. ✅ ~~Research Amp's uninstall process~~ (Done)
3. ✅ ~~Research Claude Code install/uninstall/upgrade process~~ (Done)
4. ✅ ~~Implement install/uninstall commands for remaining tools~~ (Done)
5. ✅ ~~Research upgrade commands for remaining tools~~ (Done)
6. ✅ ~~Implement upgrade functionality in the CLI~~ (Done)
