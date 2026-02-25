# interactive-worktree

Interactive CLI wrapper for [git-worktree-runner](https://github.com/coderabbitai/git-worktree-runner).

![Demo](demo.gif)

## Requirements

- [git-worktree-runner](https://github.com/coderabbitai/git-worktree-runner)
- Git 2.17+
- Bash 3.2+ (4.0+ recommended)

## Installation

### Using shell script installer (recommended)

Installs a prebuilt binary to `~/.cargo/bin`:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/takumi3488/interactive-worktree/releases/latest/download/interactive-worktree-installer.sh | sh
```

Supported platforms: macOS (ARM64, x86_64), Linux (x86_64)

### Using cargo

```bash
cargo install interactive-worktree
```

## Usage

```bash
iwt
```
