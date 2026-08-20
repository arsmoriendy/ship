# ship 🚢

A terminal UI for managing Docker registries and images.

## Features

- Push, delete, and prune images from your registry
- Configurable commands for any API compatible registry
- Configurable keymaps

## Installation

```sh
cargo install ship-tui
```

Or build from source:

```sh
git clone https://github.com/arsmoriendy/ship --revision=latest
cd ship
cargo install --path .
```

## Glossary

- **Repository** - the name of an image
- **Project** - a group of related images, named after the last segment of the image repository (e.g. `myrepo/myapp` → project `myapp`)
- **Registry** - the remote Docker registry a project is pushed to (configured via `projectRegistries`)
- **Image** - a local container image as listed by Docker, identified by its ID
- **Digest** - the content hash (SHA-256) of an image in a registry, used to detect whether a local image matches the remote, **different from the image id**

## Configuration

The config file lives at `~/.config/ship/config.json` (per platform conventions) and is auto-created with defaults on first run.

### Top-level fields

| Field               | Type                      | Purpose                                                               |
| ------------------- | ------------------------- | --------------------------------------------------------------------- |
| `projectRegistries` | `map<project, registry>`  | Associates a project name with its registry[^1]                       |
| `registryCommands`  | `map<registry, commands>` | Per-registry shell commands (`deleteImage`, `listDigests`)[^2]        |
| `commandBehaviours` | object                    | `pushImage`/`deleteImage`: `"async"` (default) or `"interactive"`[^3] |
| `keymaps`           | `map<action, keymap[]>`   | Key bindings per action                                               |

[^1]: E.g., `"projectRegistries": { "<project-name>": "<subdomain>.vultrcr.com/<registry>" }`

[^2]: See [`COMMANDS.md`](https://github.com/arsmoriendy/ship/blob/master/COMMANDS.md) for examples

[^3]: `interactive` commands leave the TUI so you can interact with them (e.g. docker login prompts); `async` runs in the background.

### Command templating

Commands run via `sh -c` and support placeholders:

- `deleteImage`: `{id}`, `{repository}`, `{digest}`
- `listDigests`: `{project}`

### Keymaps

Each action maps to an array of `{ "key": KeyCode, "modifiers"?: string }`. Keys use crossterm's serde format (`{ "Char": "j" }`, `"Up"`, `"F1"`, etc.); modifiers are crossterm names like `"CONTROL"`:

```json
"quit": [{ "key": { "Char": "c" }, "modifiers": "CONTROL" }]
```

Available actions: `selectUp`, `selectDown`, `focusImages`, `focusProjects`, `pushImage`, `deleteImage`, `fetchDigests`, `pruneImages`, `quit`, `closePopup`.

## License

MIT
