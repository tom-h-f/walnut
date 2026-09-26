#!/bin/bash
# Point every commit on the current branch at the GitHub user this machine
# commits as. Author dates stay as they are. The committer date is set to
# the author date, so the two timestamps are the same moment.
#
# Does not change git config. Rewrites the checked-out branch only.
# Usage: scripts/rewrite-identity.sh
#        GIT_REWRITE_EMAIL=you@example.com scripts/rewrite-identity.sh

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

if [ -n "$(git status --porcelain)" ]; then
  echo "working tree is dirty" >&2
  exit 1
fi

name="${GIT_REWRITE_NAME:-}"
if [ -z "$name" ] && command -v gh >/dev/null 2>&1; then
  name="$(gh api user --jq .name)"
fi
if [ -z "$name" ] || [ "$name" = "null" ]; then
  name="$(git config user.name)"
fi

email="${GIT_REWRITE_EMAIL:-$(git config user.email)}"
if [ -z "$name" ] || [ -z "$email" ]; then
  echo "need a name and email (gh user name, or git config user.name / user.email)" >&2
  exit 1
fi

branch="$(git symbolic-ref --short HEAD)"
echo "rewriting $branch as $name <$email>"

# $name and $email expand now. GIT_AUTHOR_DATE expands once per commit.
FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch -f --env-filter "
export GIT_AUTHOR_NAME='$name'
export GIT_AUTHOR_EMAIL='$email'
export GIT_COMMITTER_NAME='$name'
export GIT_COMMITTER_EMAIL='$email'
export GIT_COMMITTER_DATE=\"\$GIT_AUTHOR_DATE\"
" "$branch"

git update-ref -d "refs/original/refs/heads/$branch" 2>/dev/null || true
rm -rf .git/refs/original

echo "done. author and committer are $name <$email>"
git log -1 --format='%h %an <%ae> %ad | committer %cd' --date=iso
