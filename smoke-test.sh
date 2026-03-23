#!/usr/bin/env bash
# Smoke test: run all read-only bl commands against a real Backlog space.
# Usage: ./smoke-test.sh [SPACE_KEY [PROJECT_KEY]]
set -euo pipefail

SPACE_KEY="${1:-sattg}"
PROJECT_KEY="${2:-DOTS_TEST}"
BL="${BL:-$(command -v bl 2>/dev/null || echo "./target/debug/bl")}"
PASS=0
FAIL=0
BL_OUT=$(mktemp)
trap 'rm -f "$BL_OUT"' EXIT

# ── helpers ────────────────────────────────────────────────────────────────────

run() {
    local label="$1"; shift
    if "$BL" --space "$SPACE_KEY" "$@" > "$BL_OUT" 2>&1; then
        echo "  PASS  $label"
        PASS=$((PASS + 1))
    else
        echo "  FAIL  $label"
        sed 's/^/        /' "$BL_OUT"
        FAIL=$((FAIL + 1))
    fi
}

# Run command and capture stdout for later use; still reports PASS/FAIL.
run_capture() {
    local label="$1"; local varname="$2"; shift 2
    local output
    if "$BL" --space "$SPACE_KEY" "$@" > "$BL_OUT" 2>&1; then
        echo "  PASS  $label"
        PASS=$((PASS + 1))
        output="$(cat "$BL_OUT")"
        printf -v "$varname" '%s' "$output"
    else
        echo "  FAIL  $label"
        sed 's/^/        /' "$BL_OUT"
        FAIL=$((FAIL + 1))
        printf -v "$varname" ''
    fi
}

section() { echo; echo "── $* ──────────────────────────────────────────"; }

jq_or_empty() {
    # Extract a value with jq; return empty string on error.
    local input="$1"; local query="$2"
    echo "$input" | jq -r "$query" 2>/dev/null || true
}

# ── Phase 1: space (read-only) ─────────────────────────────────────────────────

section "auth"
run "auth status"  auth status

section "space"
run "space show"        space --json
run "space activities"  space activities --json
run "space disk-usage"  space disk-usage --json
run "space notification" space notification --json
run "space licence"     space licence --json

section "master data"
run "priority list"   priority list --json
run "resolution list" resolution list --json
run "rate-limit"      rate-limit --json

# ── Phase 2: project ──────────────────────────────────────────────────────────

section "project"
run "project list"                  project list --json
run_capture "project show" PROJ_JSON  project show "$PROJECT_KEY" --json
run "project activities"            project activities "$PROJECT_KEY" --json
run "project disk-usage"            project disk-usage "$PROJECT_KEY" --json
run "project user list"             project user list "$PROJECT_KEY" --json
run "project admin list"            project admin list "$PROJECT_KEY" --json
run "project status list"           project status list "$PROJECT_KEY" --json
run "project issue-type list"       project issue-type list "$PROJECT_KEY" --json
run "project category list"         project category list "$PROJECT_KEY" --json
run "project version list"          project version list "$PROJECT_KEY" --json
run "project team list"             project team list "$PROJECT_KEY" --json
run "project webhook list"          project webhook list "$PROJECT_KEY" --json
run "project custom-field list"     project custom-field list "$PROJECT_KEY" --json

PROJECT_ID=$(jq_or_empty "$PROJ_JSON" '.id')

# ── Phase 3: issues ───────────────────────────────────────────────────────────

section "issue"
if [[ -n "$PROJECT_ID" ]]; then
    run "issue list"  issue list --project-id "$PROJECT_ID" --json
    run "issue count" issue count --project-id "$PROJECT_ID" --json
    run_capture "issue show (first)" ISSUE_JSON  issue list --project-id "$PROJECT_ID" --count 1 --json
    ISSUE_KEY=$(jq_or_empty "$ISSUE_JSON" '.[0].issueKey // empty')
else
    echo "  SKIP  issue list/count (project ID unavailable)"
    ISSUE_KEY=""
fi

if [[ -n "$ISSUE_KEY" ]]; then
    run "issue show $ISSUE_KEY"               issue show "$ISSUE_KEY" --json
    run "issue comment list"                  issue comment list "$ISSUE_KEY" --json
    run "issue comment count"                 issue comment count "$ISSUE_KEY" --json
    run "issue attachment list"               issue attachment list "$ISSUE_KEY" --json
    run "issue participant list"              issue participant list "$ISSUE_KEY" --json
    run "issue shared-file list"              issue shared-file list "$ISSUE_KEY" --json

    # comment show (first comment if any)
    run_capture "issue comment list (for id)" CMTS_JSON  issue comment list "$ISSUE_KEY" --json
    COMMENT_ID=$(jq_or_empty "$CMTS_JSON" '.[0].id // empty')
    if [[ -n "$COMMENT_ID" ]]; then
        run "issue comment show $COMMENT_ID"          issue comment show "$ISSUE_KEY" "$COMMENT_ID" --json
        run "issue comment notification list"         issue comment notification list "$ISSUE_KEY" "$COMMENT_ID" --json
    else
        echo "  SKIP  issue comment show/notification (no comments)"
    fi
else
    echo "  SKIP  issue subcommands (no issues found)"
fi

# ── Phase 4: wiki ─────────────────────────────────────────────────────────────

section "wiki"
run "wiki list"                 wiki list "$PROJECT_KEY" --json
run "wiki count"                wiki count "$PROJECT_KEY" --json
run "wiki tag list"             wiki tag list "$PROJECT_KEY" --json

run_capture "wiki list (for id)" WIKIS_JSON  wiki list "$PROJECT_KEY" --json
WIKI_ID=$(jq_or_empty "$WIKIS_JSON" '.[0].id // empty')

if [[ -n "$WIKI_ID" ]]; then
    run "wiki show $WIKI_ID"           wiki show "$WIKI_ID" --json
    run "wiki history"                 wiki history "$WIKI_ID" --json
    run "wiki attachment list"         wiki attachment list "$WIKI_ID" --json
    run "wiki shared-file list"        wiki shared-file list "$WIKI_ID" --json
    run "wiki star list"               wiki star list "$WIKI_ID" --json
else
    echo "  SKIP  wiki show/history/attachment/shared-file/star (no wiki pages)"
fi

# ── Phase 5: documents ────────────────────────────────────────────────────────

section "document"
if [[ -n "$PROJECT_ID" ]]; then
    run "document list"             document list --project-id "$PROJECT_ID" --json
    run "document tree"             document tree "$PROJECT_KEY" --json

    run_capture "document list (for id)" DOCS_JSON  document list --project-id "$PROJECT_ID" --json
    DOC_ID=$(jq_or_empty "$DOCS_JSON" '.list[0].id // .[0].id // empty')
    if [[ -n "$DOC_ID" ]]; then
        run "document show $DOC_ID"  document show "$DOC_ID" --json
    else
        echo "  SKIP  document show (no documents)"
    fi
else
    echo "  SKIP  document (project ID unavailable)"
fi

# ── Phase 6: shared files ─────────────────────────────────────────────────────

section "shared-file"
run "shared-file list"  shared-file list "$PROJECT_KEY" --json

# ── Phase 7: git / PR ─────────────────────────────────────────────────────────

section "git / pr"
run_capture "git repo list" REPOS_JSON  git repo list "$PROJECT_KEY" --json
REPO_NAME=$(jq_or_empty "$REPOS_JSON" '.[0].name // empty')

if [[ -n "$REPO_NAME" ]]; then
    run "git repo show"           git repo show "$PROJECT_KEY" "$REPO_NAME" --json
    run "pr list"                 pr list "$PROJECT_KEY" "$REPO_NAME" --json
    run "pr count"                pr count "$PROJECT_KEY" "$REPO_NAME" --json

    run_capture "pr list (for number)" PRS_JSON  pr list "$PROJECT_KEY" "$REPO_NAME" --json
    PR_NUM=$(jq_or_empty "$PRS_JSON" '.[0].number // empty')
    if [[ -n "$PR_NUM" ]]; then
        run "pr show $PR_NUM"             pr show "$PROJECT_KEY" "$REPO_NAME" "$PR_NUM" --json
        run "pr comment list"             pr comment list "$PROJECT_KEY" "$REPO_NAME" "$PR_NUM" --json
        run "pr comment count"            pr comment count "$PROJECT_KEY" "$REPO_NAME" "$PR_NUM" --json
        run "pr attachment list"          pr attachment list "$PROJECT_KEY" "$REPO_NAME" "$PR_NUM" --json
    else
        echo "  SKIP  pr show/comment/attachment (no pull requests)"
    fi
else
    echo "  SKIP  git repo show / pr (no git repositories)"
fi

# ── Phase 8: user ─────────────────────────────────────────────────────────────

section "user"
run "user list"                    user list --json
run "user recently-viewed"         user recently-viewed --json
run "user recently-viewed-projects" user recently-viewed-projects --json
run "user recently-viewed-wikis"   user recently-viewed-wikis --json

run_capture "user list (for id)" USERS_JSON  user list --json
USER_ID=$(jq_or_empty "$USERS_JSON" '.[] | select(.userId != null) | .id' | head -1)

if [[ -n "$USER_ID" ]]; then
    run "user show $USER_ID"         user show "$USER_ID" --json
    run "user activities"            user activities "$USER_ID" --json
    run "user star list"             user star list "$USER_ID" --json
    run "user star count"            user star count "$USER_ID" --json
    run "watch list"                 watch list "$USER_ID" --json
    run "watch count"                watch count "$USER_ID" --json
else
    echo "  SKIP  user show/activities/star/watch (no users with userId)"
fi

# ── Phase 9: team (space scope, read-only) ────────────────────────────────────

section "team"
run_capture "team list" TEAMS_JSON  team list --json
TEAM_ID=$(jq_or_empty "$TEAMS_JSON" '.[0].id // empty')
if [[ -n "$TEAM_ID" ]]; then
    run "team show $TEAM_ID"  team show "$TEAM_ID" --json
else
    echo "  SKIP  team show (no teams)"
fi

# ── Phase 10: notification (space scope, read-only) ───────────────────────────

section "notification"
run "notification list"   notification list --json
run "notification count"  notification count --json

# ── summary ───────────────────────────────────────────────────────────────────

echo
echo "════════════════════════════════════════"
echo "  PASS: $PASS   FAIL: $FAIL"
echo "════════════════════════════════════════"
[[ "$FAIL" -eq 0 ]]
