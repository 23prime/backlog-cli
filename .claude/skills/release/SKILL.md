---
name: release
description: "Cut a new backlog-cli release by bumping the version and pushing a tag. Use when the user asks to release, cut a release, ship, or publish a new version of the bl CLI."
---

# Release

## Step 0 — Sync main

```bash
git branch --show-current
git status
```

- Must be on `main` with a clean working tree. If not, confirm with the user how to proceed.

```bash
git pull origin main
```

## Step 1 — Propose a bump level

Get the latest tag:

```bash
git describe --tags --abbrev=0
```

Take the tag printed above (e.g. `v0.8.0`) and list commits since it, substituting the
literal value in place of `<latest_tag>`:

```bash
git log <latest_tag>..HEAD --oneline
```

Based on Conventional Commit types in that log, propose a level per
[Semantic Versioning](https://semver.org/):

| Commits since last tag | Suggested level |
| --- | --- |
| Any breaking change | `major` |
| `feat` (no breaking change) | `minor` |
| Only `fix` / `chore` / `docs` / etc. | `patch` |

Present the proposal with the reasoning to the user and get confirmation (or their own choice)
before proceeding.

## Step 2 — Run the release task

```bash
mise run release -- <patch|minor|major> -y
```

`mise/tasks/release` will:

1. Bump `Cargo.toml` / `Cargo.lock` via `cargo set-version`
2. Commit `chore: bump version to v<X.Y.Z>`
3. Create annotated tag `v<X.Y.Z>`
4. Push the commit and tag to `origin`

`-y` skips the interactive confirmation prompt — the confirmation with the user already
happened in Step 1, so no need for a second, terminal-only confirmation here.

## Step 3 — Wait for CI to finish

The pushed tag triggers `.github/workflows/release.yml`, which creates the GitHub Release,
builds binaries for all platforms, uploads them, and moves the `latest` tag.

Get the run's database ID:

```bash
gh run list --workflow=release.yml --limit 1 --json databaseId,status,url
```

Take the `databaseId` printed above (e.g. `30054336679`) and watch it until it finishes,
substituting the literal value in place of `<run_id>`:

```bash
gh run watch <run_id> --exit-status
```

If the command exits non-zero, the run failed — inspect the failing job
(`gh run view <run_id> --log-failed`) and resolve it before telling the user the release is
done. Once it succeeds, report the run URL and confirm the release is complete.
