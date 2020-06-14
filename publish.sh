#!/bin/bash

opt_force=0

url=$(git config --get remote.origin.url | sed 's,git@github.com:,,;s,/,.github.io/,;s,^,https://,;s,.git$,/,')

git_no_updates() { x=$(git status | grep 'nothing added to commit but untracked files present' | wc -l); [ $x -ne 0 ]; return $?; }
git_untracked() { x=$(git status | grep 'Untracted files' | wc -l); [ $x -ne 0 ]; return $?; }

git_no_untracked_() {
    git_untracked || { echo "[info] No untracked files"; return 0; }
    [ $opt_force -eq 1 ] && echo "[warn] You have untracked files" && return 0
    echo "[error] You have untracked files, use --force" && exit 1 
}
git_no_untracked_
git_untracked && echo "[warn] You have untracked files" && exit 1
git_no_updates && echo "[info] No changes to publish" && exit 1

publish() { 
    mdbook build                                          && \
    rsync -avx --delete --info=progress2 ./book/ ./docs/  &&\
    git status                                            && \
    echo 'git commit -am'                                 && \
    echo -n "Comment: " && read comment                   && \
    git commit -am "$comment"                             && \
    git push                                              && \
    echo "Published: $url"
}

# Parse Args
PARAMS=""
while (( "$#" )); do
  case "$1" in
    -f|--force)
      opt_force=1
      shift
      ;;
    -b|--my-flag-with-argument)
      if [ -n "$2" ] && [ ${2:0:1} != "-" ]; then
        MY_FLAG_ARG=$2
        shift 2
      else
        echo "Error: Argument for $1 is missing" >&2
        exit 1
      fi
      ;;
    -*|--*=) # unsupported flags
      echo "Error: Unsupported flag $1" >&2
      exit 1
      ;;
    *) # preserve positional arguments
      PARAMS="$PARAMS $1"
      shift
      ;;
  esac
done
set positional arguments in their proper place
eval set -- "$PARAMS"

publish