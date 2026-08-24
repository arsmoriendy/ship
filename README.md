# ship 🚢

A TUI for managing **remote** OCI (e.g., Docker, Podman) images.

## What it isn't

A TUI for managing **local** OCI images. For that, use
[`lazydocker`](https://github.com/jesseduffield/lazydocker). Currently, ship
only manipulate local images on image push and pull actions.

## Features

- Push, pull, delete and prune images from your registry
- Configurable commands for any API compatible registry
- Configurable keymaps
- Configurable OCI commands

## Glossary

- **Repository** - the name of an image
- **Project** - a group of related images, named after the last segment of the
  image repository (e.g. `myrepo/myapp` → project `myapp`), _a unique concept to
  ship_
- **Registry** - the remote Docker registry a project is pushed to (configured
  via `projectRegistries`)
- **Image** - a local container image as listed by Docker, identified by its ID
- **Digest** - the content hash (SHA-256) of an image in a registry, used to
  detect whether a local image matches the remote, **different from the image
  id**

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

## Configuration

The config file lives at `~/.config/ship/config.json` (per platform conventions)
and is auto-created with defaults on first run.

### Top-level fields

| Field               | Type                      | Purpose                                                                                 |
| ------------------- | ------------------------- | --------------------------------------------------------------------------------------- |
| `ociCmd`            | `string`                  | Specifies the OCI command used. Defaults to `docker`.                                   |
| `projectRegistries` | `map<project, registry>`  | Associates a project name with its registry[^1]                                         |
| `registryCommands`  | `map<registry, commands>` | Per-registry shell commands (`deleteRemoteImage`, `fetchImages`)[^2]                    |
| `commandBehaviours` | object                    | `pushImage`/`deleteRemoteImage`/`pullImage`: `"async"` or `"interactive"` (default)[^3] |
| `keymaps`           | `map<action, keymap[]>`   | Key bindings per action                                                                 |

[^1]:
    E.g.,
    `"projectRegistries": { "<project-name>": "<subdomain>.vultrcr.com/<registry>" }`

[^2]:
    See
    [`COMMANDS.md`](https://github.com/arsmoriendy/ship/blob/master/COMMANDS.md)
    for examples

[^3]:
    `interactive` commands leave the TUI so you can interact with them (e.g.
    docker login prompts); `async` runs in the background.

### Command templating

Commands run via `sh -c` and support placeholders:

- `deleteRemoteImage`: `{project}`, `{digest}`
- `fetchImages`: `{project}`

### Keymaps

Each action maps to an array of `{ "key": KeyCode, "modifiers"?: string }`. Keys
use crossterm's serde format (`{ "Char": "j" }`, `"Up"`, `"F1"`, etc.);
modifiers are crossterm names like `"CONTROL"`:

```json
"quit": [{ "key": { "Char": "c" }, "modifiers": "CONTROL" }]
```

Available actions: `selectUp`, `selectDown`, `focusImages`, `focusProjects`,
`pushImage`, `pullImage`, `deleteRemoteImage`, `fetchImages`,
`pruneRemoteImages`, `quit`, `closePopup`.
