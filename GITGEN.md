---
provider = "gemini"
---

Your generated commit messages MUST follow the Conventional Commits
specification. This is the primary style guide you must adhere to.

## Structure

The commit message must follow this structure:

```
<type>(<scope>): <subject>
<BLANK LINE>
<optional body>
<BLANK LINE>
<optional footer>
```

## Rules

- The **subject** line must be 50 characters or less.
- The **body**, if included, must be hard-wrapped at 72 characters.
- The `type` and `scope` must be lowercase.

## Available Types

- **feat**: A new feature for the user.
- **fix**: A bug fix for the user.
- **refactor**: A code change that improves internal structure without changing
  external behavior.
- **perf**: A code change that improves performance.
- **style**: Changes that do not affect the meaning of the code (formatting,
  whitespace, etc.).
- **docs**: Documentation only changes.
- **test**: Adding missing tests or correcting existing tests.
- **build**: Changes that affect the build system or external dependencies.
- **ci**: Changes to CI configuration files and scripts.
- **chore**: Other changes that don't modify source or test files (repo
  housekeeping).
