---
name: implementing-backlog-feature
description: "Implement a planned backlog-cli feature end-to-end following the project workflow: branch → implement → check → coderabbit review → real API test → fix → commit → push → PR. Use when the user asks to implement a feature listed as Planned in docs/user-guide.md, or says 次の機能を実装して / implement the next feature."
---

# Implementing a backlog-cli Feature

Read `AGENTS.md` for architecture conventions and `docs/CONTRIBUTING.md` for branch/commit rules.
See `references/patterns.md` for code patterns and known gotchas.

## Workflow

### 1. Pick a feature

Read `docs/user-guide.md` and pick the first "Planned" entry from the command coverage table.
Confirm with the user before proceeding.

### 2. Create branch

```bash
git switch -c feature/<name>   # e.g. feature/space-disk-usage
```

### 3. Implement

Before writing any structs, check the official references for the exact response schema:

- **API docs**: https://developer.nulab.com/docs/backlog/
- **Official SDK**: https://github.com/nulab/backlog-js/ (use as ground truth for field names and types)

Then follow AGENTS.md: `src/api/<resource>.rs` → `src/api/mod.rs` (trait) → `src/cmd/<command>.rs` → `src/main.rs` → `docs/user-guide.md`.

See `references/patterns.md` for concrete patterns and common mistakes.

### 4. Check

```bash
mise run rs-check
```

Fix all errors and warnings before continuing.

### 5. Coderabbit review

Invoke the `coderabbit` skill. Fix each actionable finding and re-review until clean.

### 6. Test against real API

```bash
mise run rs-run <subcommand args>
```

This is the only way to catch issues like nullable fields from the real Backlog API.
If deserialization fails, the raw JSON is printed — use it to identify the exact null field.
Fix, then re-run to confirm.

**For POST / PATCH / DELETE commands: always confirm with the user before running — these affect real data.**

### 7. Commit

```bash
git add <files>
git commit -m "feat: <description>"
```

- Conventional Commits, English only
- **No `Co-Authored-By:` line**
- Run `mise run rs-check` again if the pre-commit hook fails

### 8. Push and open PR

```bash
git push -u origin feature/<name>
gh pr create --title "feat: ..." --body "$(cat <<'EOF'
## Checklist
- [x] Target branch is `main`
- [ ] Status checks are passing

## Summary
...

## Reason for change
...

## Changes
...
EOF
)"
```

PR body must be in English and follow `.github/PULL_REQUEST_TEMPLATE.md`.
