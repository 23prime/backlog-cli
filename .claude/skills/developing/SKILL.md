---
name: developing
description: "General development workflow for backlog-cli: branch → implement → fix → coderabbit review → (API test) → commit → push → PR. Covers feature additions, refactoring, performance tuning, and bug fixes. Use when the user asks to implement, refactor, tune, or fix something in the codebase."
---

# Development Workflow

Read `AGENTS.md` for architecture conventions and `docs/CONTRIBUTING.md` for branch/commit/PR rules.

## Step 1 — Clarify task type

Determine which type applies:

| Type | Branch prefix | Commit type |
| ---- | ------------- | ----------- |
| New feature | `feature/` | `feat` |
| Refactoring | `feature/` | `refactor` |
| Performance | `feature/` | `perf` |
| Bug fix | `feature/` | `fix` |

Confirm the task and type with the user before proceeding.

## Step 2 — Create branch

Follow `docs/CONTRIBUTING.md`:

```bash
git switch -c feature/<name>
```

## Step 3 — Pick a feature (feature tasks only)

Read `website/docs/commands.md` and pick the first "Planned" entry from the command coverage table.
Confirm the selection with the user before proceeding.

For feature tasks, also read `references/patterns.md` for code patterns and known gotchas, and check the official API docs before writing structs:

- **API docs**: <https://developer.nulab.com/docs/backlog/>
- **Official SDK**: <https://github.com/nulab/backlog-js/> (ground truth for field names and types)

## Step 4 — Implement

Follow `AGENTS.md` conventions. For feature tasks, the typical file order is:

1. `src/api/<resource>.rs` — response struct + `BacklogClient` method
2. `src/api/mod.rs` — trait declaration + impl
3. `src/cmd/<resource>/<subcommand>.rs` — `<cmd>()` + `<cmd>_with()` + tests
4. `src/cmd/<resource>/mod.rs` — re-export
5. `src/main.rs` — clap wiring
6. `website/docs/commands.md` and `website/i18n/ja/docusaurus-plugin-content-docs/current/commands.md` — add command docs and mark as implemented in the coverage table

## Step 5 — Auto-fix and check

```bash
mise run fix
```

If any errors remain that cannot be auto-fixed, resolve them manually, then verify:

```bash
mise run check
```

Repeat until clean.

## Step 6 — CodeRabbit review

Invoke the `coderabbit` skill. Fix each actionable finding and re-review until clean.

## Step 7 — Test against real API (feature / fix: required; refactor / perf: confirm with user)

```bash
mise run rs-run <subcommand args>
```

This is the only way to catch issues like nullable fields from the real Backlog API.
If deserialization fails, the raw JSON is printed — use it to identify the exact null field, fix, and re-run.

**For POST / PATCH / DELETE commands: always confirm with the user before running — these affect real data.**

For refactor and perf tasks, ask the user whether a real API test is needed before running.

## Step 8 — Commit

Follow `docs/CONTRIBUTING.md`:

- Conventional Commits, English only
- **No `Co-Authored-By:` line**

```bash
git add <files>
git commit -m "<type>: <description>"
```

## Step 9 — Push and open PR

```bash
git push -u origin feature/<name>
```

Create a PR following `.github/PULL_REQUEST_TEMPLATE.md`. PR body must be in English.

```bash
gh pr create --title "<type>: ..." --body "$(cat <<'EOF'
## Checklist

- [ ] Target branch is `main`
- [ ] Status checks are passing

## Summary

## Reason for change

## Changes

## Notes
EOF
)"
```

## Step 10 — Address PR review comments

After opening the PR, GitHub Copilot and CodeRabbit will post review comments automatically. Check whether reviews have arrived:

```bash
gh pr view <PR_NUMBER> --json reviews | jq '[.reviews[].author.login]'
```

- If the array is empty, reviews have not arrived yet. Wait ~2–3 minutes and check again.
- If still empty after ~5 minutes, automated reviews are likely disabled — skip this step.
- If `copilot-pull-request-reviewer` or `coderabbitai` appear, invoke the `implementing-pr-review` skill to evaluate and apply valid suggestions.
