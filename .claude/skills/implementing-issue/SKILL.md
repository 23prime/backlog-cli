---
name: implementing-issue
description: "Issue-driven development workflow for backlog-cli: reads a GitHub Issue, implements the change, and opens a PR that closes the Issue. Use when the user references a GitHub Issue number and asks to implement or fix it."
---

# Implementing Issue

Read `AGENTS.md` for architecture conventions and `docs/CONTRIBUTING.md` for branch/commit/PR rules.

## Step 0 — Read the Issue

Determine the issue number from the user's message or context. Fetch the Issue:

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
gh issue view <N> -R "$REPO"
```

Read the title, body, and comments to understand the task fully before proceeding.

The Issue body follows `.github/ISSUE_TEMPLATE/default.md`. Extract the **Type** field
from the `## Type` section if present.

## Step 1 — Clarify task type

Use the Type extracted from the Issue body. If the `## Type` section is missing or unclear,
determine the type from context:

| Type | Branch prefix | Commit type |
| ---- | ------------- | ----------- |
| `feat` | `feature/` | `feat` |
| `fix` | `feature/` | `fix` |
| `refactor` | `feature/` | `refactor` |
| `perf` | `feature/` | `perf` |
| `docs` | `feature/` | `docs` |
| `ai` | `feature/` | `ai` |
| `chore` | `feature/` | `chore` |

Confirm the task and type with the user before proceeding.

## Step 2 — Create branch

Include the Issue number in the branch name:

```bash
git switch -c feature/<N>-<short-description>
```

## Step 3 — Pick a feature (Backlog CLI feature tasks only)

Read `website/docs/commands.md` and pick the first "Planned" entry from the command coverage table.
Confirm the selection with the user before proceeding.

For feature tasks, also read `docs/PATTERNS.md` for code patterns and known gotchas, and check the official API docs before writing structs:

- **API docs**: <https://developer.nulab.com/docs/backlog/>
- **Official SDK**: <https://github.com/nulab/backlog-js/> (ground truth for field names and types)

## Step 4 — Implement

Follow `AGENTS.md` conventions. For Backlog CLI feature tasks, the typical file order is:

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

## Step 7 — Test against real API (feature / fix: required; refactor / perf / docs / ai: confirm with user)

```bash
mise run rs-run <subcommand args>
```

This is the only way to catch issues like nullable fields from the real Backlog API.
If deserialization fails, the raw JSON is printed — use it to identify the exact null field, fix, and re-run.

**For POST / PATCH / DELETE commands: always confirm with the user before running — these affect real data.**

For non-feature tasks (refactor, perf, docs, ai), ask the user whether a real API test is needed.

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
git push -u origin feature/<N>-<short-description>
```

Create a PR following `.github/PULL_REQUEST_TEMPLATE.md`. PR body must be in English and include `Closes #<N>`.

```bash
gh pr create --title "<type>: ..." --body "$(cat <<'EOF'
## Checklist

- [ ] Target branch is `main`
- [ ] Status checks are passing
- [ ] Documentation updated if user-visible behavior changed (`website/docs/`, `website/i18n/ja/`, `README.md`)

## Summary

## Reason for change

## Changes

## Notes

Closes #<N>
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

## Step 11 — Reflect learnings into `docs/PATTERNS.md`

After the PR is merged, review what was encountered during implementation and PR review.
If anything is **generalizable** — useful for future implementations in this project — add it to `docs/PATTERNS.md`.

Sources to review:

- **Accepted review comments**: patterns or pitfalls that reviewers pointed out (e.g. clap flag conventions, validation boundaries, httpmock gotchas)
- **User corrections**: when the user said "no, not that — do X instead", consider whether X is a project-wide rule worth documenting
- **Compilation/check failures**: if `mise run check` failed for a non-obvious reason, note the fix

Criteria for adding to `docs/PATTERNS.md`:

- Would a future implementer likely make the same mistake?
- Is it specific to this codebase (not just "read the Rust docs")?
- Is it concrete enough to be actionable?

If any of the above apply, append to the relevant section of `docs/PATTERNS.md` and commit directly to `main`:

```bash
git add docs/PATTERNS.md
git commit -m "docs: add patterns from <issue-N> implementation"
```

If nothing generalizable was found, skip this step.
