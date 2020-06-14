#!/bin/bash
####################################################################

####################################################################
# Options
####################################################################
opt_force=0

####################################################################
# Initialization
####################################################################
url=$(git config --get remote.origin.url | sed 's,git@github.com:,,;s,/,.github.io/,;s,^,https://,;s,.git$,/,')


####################################################################
# Functions
####################################################################
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

####################################################################
# Parse Args
####################################################################
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

####################################################################
# Main
####################################################################
. ./git_functions.sh

git_check_untracked
git_no_updates && echo "[info] No changes to publish" && exit 1

publish
