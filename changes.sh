#!/usr/bin/env bash
# Shows what changed between two versions: the CHANGELOG.md entries, the
# commits, a summary of the files, and the diff of everything written by hand
# (runtime, mods, recompiler, build). The translated code in src/code/ is only
# summarised, because a regeneration rewrites much of it; --code adds its diff.
#
# Usage: ./changes.sh FROM [TO] [--code]
#   FROM, TO: versions such as 0.5-beta (the tag v0.5-beta), or any git
#   revision. TO defaults to the working tree's commit, HEAD.
# Example: ./changes.sh 0.4-beta 0.5-beta > changes.diff
set -euo pipefail
cd "$(dirname "$0")"

code=0
revisions=()
for argument in "$@"; do
    case $argument in
        --code) code=1 ;;
        -h|--help) sed -n '2,12p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
        *) revisions+=("$argument") ;;
    esac
done
[[ ${#revisions[@]} -ge 1 && ${#revisions[@]} -le 2 ]] || { sed -n '2,12p' "$0" | sed 's/^# \{0,1\}//' >&2; exit 2; }

# A version names its tag; anything else is taken as a git revision.
resolve() {
    if git rev-parse -q --verify "refs/tags/v$1" >/dev/null; then
        echo "v$1"
    elif git rev-parse -q --verify "$1^{commit}" >/dev/null; then
        echo "$1"
    else
        echo "No version or revision called $1. Versions: $(git tag -l 'v*' | tr '\n' ' ')" >&2
        exit 2
    fi
}
from=$(resolve "${revisions[0]}")
to=$(resolve "${revisions[1]:-HEAD}")
version_at() { git show "$1:VERSION" 2>/dev/null | tr -d '[:space:]'; }

echo "# Changes from $(version_at "$from") ($from) to $(version_at "$to") ($to)"
echo
# The changelog sections that are new in TO.
echo "## Changelog"
echo
git diff "$from" "$to" -- CHANGELOG.md | sed -n 's/^+\([^+]\)/\1/p; s/^+$//p'
echo
echo "## Commits"
echo
git log --reverse --format='- %h %s' "$from..$to"
echo
echo "## Files"
echo
git diff --stat=100 "$from" "$to" -- . ':!src/code'
echo
echo "Translated code (src/code/):"
git diff --shortstat "$from" "$to" -- src/code
echo
echo "## Diff"
echo
if [[ $code -eq 1 ]]; then
    git diff "$from" "$to"
else
    git diff "$from" "$to" -- . ':!src/code'
fi
