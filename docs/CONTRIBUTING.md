# Contributing Guide

## Branch Strategy

### Branches

- `main`: Main (default) branch.
- `feature/xxxx`: Topic branch.

### Approval Flow

- Changes to the `main` branch are made exclusively through pull requests.
- Merging requires approval from at least one reviewer. Reviewers should be discussed and assigned as needed.

## Commit Messages

- Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
- Scopes are optional. Use them when clarifying which part of the project is affected.
- Commit messages should be written in English.
- Commit message content:
  - Line 1: A concise summary of the change.
  - Line 3: (Optional) Reason for the change or non-obvious supplementary notes.

### Types

- `feat`: Feature additions or changes
- `fix`: Bug fixes or corrections
- `build`: Build tool configuration changes
- `ai`: AI configuration changes
- `ci`: CI configuration changes
- `docs`: Documentation changes
- `style`: Code style changes only
- `refactor`: Refactoring
- `perf`: Performance improvements
- `test`: Test additions or modifications
- `deps`: Dependency updates
- `chore`: Miscellaneous changes

### Scopes

Scopes are optional. Define them per project as needed. Examples for a monorepo:

- `backend`
- `frontend`
- `infra`
- `shared`

### Examples

```txt
feat(backend): Add user management API
```

```txt
chore: Update linter configuration
```

## Quality Assurance

- Consolidates all auto-fix commands into `mise run fix`.
- Before committing, run the `mise run fix` to auto-fix.
- If there are any errors that cannot be automatically fixed, fix them manually and verify that the `mise run check` passes.

## Pull Requests

Create a pull request following the [template](../.github/PULL_REQUEST_TEMPLATE.md).
