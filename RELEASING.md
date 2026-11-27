# Releasing

Three workflows do the work: [`Prepare Release`](.github/workflows/prepare-release.yml) cuts the release, [`Tag Release`](.github/workflows/tag-release.yml) tags it, and [`Release`](.github/workflows/release.yml) publishes it. Nothing is tagged or published by hand.

## Cut a release

1. Run [`Prepare Release`](https://github.com/mongodb-labs/atlas-cli-plugin-v2/actions/workflows/prepare-release.yml) from the Actions tab. It takes one input, `version`: `patch`, `minor`, `major`, or an exact version such as `1.2.3`.
2. The workflow bumps the version of every crate, regenerates `LICENSE-3RD-PARTY.txt` and `CHANGELOG.md`, and opens a pull request titled `chore(release): prepare for X.Y.Z`. The pull request comes from the branch `release/prepare-vX.Y.Z` and targets `main`.
3. Review and approve the pull request, then merge it into `main`.
4. Merging triggers `Tag Release`. It compares the version in the branch name, `release/prepare-vX.Y.Z`, with the version cargo reports for the `atlas-cli-v2-plugin` package. If the two match, it tags the merge commit `vX.Y.Z` and pushes the tag. If they differ, it stops and nothing is tagged.
5. The tag triggers `Release`, which builds an archive for each target in `dist-workspace.toml` (macOS arm64 and x86_64, Linux arm64 and x86_64, Windows x86_64) and attaches them to a GitHub release.

## Why a bot makes the release commit

`main` requires signed commits, and a commit made with `git` on a runner is not signed. `Prepare Release` therefore creates its commit with `mongodb/apix-action/commit-changes`, which goes through the GitHub API, and GitHub signs the commit it creates.

The same rule applies to the release pull request itself: a fixup commit pushed from a laptop is unsigned and would block the merge. To change what a release contains, re-run `Prepare Release`.
