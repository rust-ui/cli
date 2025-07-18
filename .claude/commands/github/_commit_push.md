# Commit and Push Changes

Commit current changes and push them to the GitHub repository.

## Usage
Use this command when you want to:
- Commit and immediately push changes to the remote repository
- Share work with the team or save to remote backup
- Complete a development cycle with remote synchronization

## Behavior
1. Performs commit using `_commit` command behavior (see _commit.md for details)
2. Push changes to origin/master (or current branch)
3. Verify the push succeeded

## Push Notes
- Combines local commit with remote push in one action
- Use `_commit` for local-only commits during development
- Only pushes when explicitly requested (this command implies explicit request)
- Requires network access and push permissions to remote repository

## Commit Details
See `_commit.md` for detailed information about:
- Conventional commit message format with robot emoji 🤖 at start
- Staging behavior and file handling
- Claude Code attribution
