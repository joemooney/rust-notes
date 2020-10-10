#!/bin/bash
####################################################################

####################################################################
# Options
####################################################################
opt_force=0    # -f, --force
opt_noexec=0   # -n, --no-exec

####################################################################
# Initialization
####################################################################
url=$(git config --get remote.origin.url | sed 's,git@github.com:,,;s,/,.github.io/,;s,^,https://,;s,.git$,/,')

# Local directory containing markdown for book - must match src in book.toml
src=md

# Note: .gitignore should have an entry for $src/README.md
[ $(grep "$src/README.md" .gitignore 2>/dev/null | wc -l) -ne 1 ] && echo "$src/README.md" >> .gitignore


####################################################################
# Functions
####################################################################
#           s,{/small},</sub>,g; s,{small},<sub>,g;
#           s,{question},<details><summary>Q: ,g; s,{/question},</details>,g; s,{small},<sub>,g;
#           s,{answer},</summary>,g; s,{/question},</details>,g; s,{small},<sub>,g;
#           s,{red},<span='color:red'>,g; s,{/red},</span>,g;
#           s,{blue},<span='color:blue'>,g; s,{/blue},</span>,g;
#           s,{green},<span='color:green'>,g; s,{/green},</span>,g;
#           s,{yellow},<span='color:yellow'>,g; s,{/yellow},</span>,g;
#           s,{gray},<span='color:gray'>,g; s,{/gray},</span>,g;
#           s,{code},<code>,g; s,{/code},</code>,g;
postprocess() { 
    for i in book/*.html
    do
        perl -p -i -e "
            s,{timestamp},$(ts),g;
        " $i
    done
}

changed() {
  changes=$(git  diff  -U0 docs | egrep -v '^(\+\+\+|---) ' | egrep '^[+-]' | egrep -v 'Last Updated:' | wc -l)
  [ $changes -eq 0 ] && echo "No Changes" && return 1
  return 0
}

publish() { 
    mdbook build                                          && \
    postprocess                                           && \
    rsync -avx --delete --info=progress2 ./book/ ./docs/  &&\
    [ $opt_noexec -eq 0 ]                                 &&\
    changed                                               && \
    git status  | grep -v 'docs/'                         && \
    echo 'git commit -am'                                 && \
    echo -n "Comment: " && read comment                   && \
    git add .                                             && \
    git commit -am "$comment"                             && \
    git push                                              && \
    echo "Published: $url"                                && \
    echo "This takes a few minutes for GitHub to update"
}

####################################################################
# Parse Args
####################################################################
PARAMS=""
while (( "$#" )); do
  case "$1" in
    -n|--no-exec) opt_noexec=1; shift ;;
    -f|--force)   opt_force=1; shift; ;;
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
      echo "Error: Unsupported flag $1" >&2; exit 1;;

    *) # preserve positional arguments
      PARAMS="$PARAMS $1"; shift ;;
  esac
done
set positional arguments in their proper place
eval set -- "$PARAMS"

####################################################################
# Main
####################################################################
. ./git_functions.sh

# Update README.md in book if necessary
# cmp -s README.md $src/README.md || cp README.md $src/README.md
[[ README.md -nt $src/README.md ]] && cp -vp README.md $src/README.md
[[ $src/README.md -nt README.md ]] && cp -vp $src/README.md README.md

git_check_untracked
git_no_updates && echo "[info] No changes to publish" && exit 1

publish
