# ai-cli-installer

A Rust-based CLI tool to manage AI development tools from the command line.

## Installation

```bash
make install
```

## Commands

### Check Versions
```bash
ai-cli-installer          # Show installed versions
ai-cli-installer check    # Check for latest versions available
```

### Interactive Install/Uninstall
```bash
ai-cli-installer install    # Select and install tools
ai-cli-installer uninstall  # Select and uninstall tools
```

## Supported Tools

- **Amp** - Amp CLI
- **Claude Code** - Claude CLI
- **Codex** - OpenAI Codex CLI
- **Cursor CLI** - Cursor Agent CLI
- **Copilot CLI** - GitHub Copilot CLI
- **Kilo** - Kilo code tool
- **Gemini** - Google Gemini CLI
- **Cline** - Cline CLI

## Development

```bash
make build    # Build release binary
make clean    # Clean build artifacts
make test     # Run tests
```
