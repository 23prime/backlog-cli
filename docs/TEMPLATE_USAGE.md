# How to Use This Template

## Getting started

### Using setup script

1. Create new remote repository on GitHub.

2. Run the setup script.

    ```bash
    curl -fsSL https://raw.githubusercontent.com/23prime/mise-template/main/setup.sh | bash -s -- <new-remote-url> [new-repo-name]
    ```

    - `<new-remote-url>` — remote URL of the pre-created repository
    - `[new-repo-name]` — optional; defaults to the repository name derived from the URL

### Manual steps

1. Create new remote repository on GitHub.

2. Clone this repository.

    ```bash
    git clone git@github.com:23prime/mise-template.git
    ```

3. Copy the cloned repository to anywhere.

    ```bash
    cp -ar mise-template <new-repo-path>
    ```

4. Into new repository.

    ```bash
    cd <new-repo-path>
    ```

5. Rename remote repository to `upstream`.

    ```bash
    git remote rename origin upstream
    ```

6. Add new remote repository as `origin`.

    ```bash
    git remote add origin <new-remote-url>
    ```

7. Check remote repositories.

    ```bash
    $ git remote -v
    origin  <new-remote-url> (fetch)
    origin  <new-remote-url> (push)
    upstream        git@github.com:23prime/mise-template.git (fetch)
    upstream        git@github.com:23prime/mise-template.git (push)
    ```

8. Push to `origin`.

    ```bash
    git push -u origin main
    ```

9. If you use GitHub CLI, set the default repository.

    ```bash
    gh repo set-default <new-repo-name>
    ```

## Initial customization

After setting up the derived repository, update the following files to match your project.

### `README.md`

Replace this file with content describing your project.

### `AGENTS.md`

Fill in the `## Project Overview` section with a description of your project.

### `.github/copilot-instructions.md`

Replace the `@AGENTS.md` reference (or add inline instructions) with guidance tailored to your project.

### `setup.sh`

Delete it — it is only needed in the template repository and is not required in derived repositories.

```bash
rm -f setup.sh
```

## Merge from upstream

1. Fetch upstream changes.

    ```bash
    git fetch upstream
    ```

2. Merge.

    ```bash
    git merge upstream/main
    ```
