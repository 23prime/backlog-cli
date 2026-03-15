---
name: creating-issue
description: "Create a well-structured GitHub Issue following the project template. Use when the user wants to file a new Issue for a feature, bug, refactoring, docs change, or AI config change."
---

# Creating Issue

## Step 1 — Gather information

Collect the following from the user's request (infer what you can; ask only for what is unclear):

| Field | Values |
| ----- | ------ |
| **Title** | Concise summary, written as `<type>: <description>` |
| **Type** | `feat` / `fix` / `refactor` / `perf` / `docs` / `ai` / `chore` |
| **Summary** | What and why (1–3 sentences) |
| **Changes** | Bullet list of concrete file/behavior changes |
| **Notes** | Optional: links, constraints, related Issues |

## Step 2 — Confirm and create

Present the draft Issue to the user and wait for confirmation.
Then create it with `gh issue create`:

```bash
gh issue create -R 23prime/backlog-cli \
  --title "<type>: <description>" \
  --body "$(cat <<'EOF'
## Type

<type>

## Summary

<summary>

## Changes

<changes>

## Notes

<notes>
EOF
)"
```

## Step 3 — Return the Issue URL

Output the created Issue URL so the user can open it immediately.
