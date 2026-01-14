# versync

Synchronizes version numbers and git tags from a single source of truth.

## Installation

```bash
cargo install --path .
```

## Usage

Create a `version.toml` in your repository root:

```toml
version = "0.1.0"

[[targets]]
file = "Cargo.toml"
key = "package.version"

[[targets]]
file = "pyproject.toml"
key = "project.version"

[[targets]]
file = "package.json"
key = "version"

[git]
tag_prefix = "v"
```

### Commands

```bash
# Check if all versions match
versync check

# Apply version to all targets
versync apply

# Create git tag
versync tag
```

### Options

- `--config <path>` - Config file path (default: `version.toml`)
- `--quiet` - Suppress output
- `--verbose` - Enable verbose output

## Workflow

```bash
# 1. Edit version.toml
# 2. Apply to all files
versync apply

# 3. Commit changes
git add -A && git commit -m "chore: bump version"

# 4. Create tag
versync tag

# 5. Push
git push --follow-tags
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Version mismatch (check only) |
| 2 | Execution error |

## License

MIT
