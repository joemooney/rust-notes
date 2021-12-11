#!/bin/bash
####################################################################
# Build, commit and push changes to github
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
postprocess() { 
    # update last edit date
    for i in book/*.html
    do
        perl -p -i -e "
            s,{timestamp},$(ts),g;
        " $i
    done
}

book_changed() {
    changes=$(git  diff  -U0 docs | egrep -v '^(\+\+\+|---) ' | egrep '^[+-]' | egrep -v 'Last Updated:' | wc -l)
    [ $changes -eq 0 -a $opt_force -eq 0 ] && echo "[publish] No Book Changes" && return 1
    return 0
}

repo_url() {
    git config --get remote.origin.url | sed 's,:,/,;s,git@,https://,;s/.git$//'
}

# [optional] If repo has an mdbook
build_book() {
    git status -s | grep " md/" | sed 's/^/[git]/'
    [ ! -e book.toml ] && return 0
    mdbook build                                          && \
    postprocess                                           && \
    rsync -ax --delete --info=progress2 ./book/ ./docs/  && \
    [ $opt_noexec -eq 0 ]                                 && \
    book_changed                                          && \
    return 0
    echo "[publish] Book not changed, used -f option to force publish"
    exit 0
}

# [optional] If repo has an mdbook
update_book_readme() {
    # Update README.md in book if necessary
    # cmp -s README.md $src/README.md || cp README.md $src/README.md
    [ ! -e book.toml ] && return 0
    [[ README.md -nt $src/README.md ]] && cp -vp README.md $src/README.md
    [[ $src/README.md -nt README.md ]] && cp -vp $src/README.md README.md
}

publish() { 
    build_book                                            && \
    echo 'git commit -am'                                 && \
    echo -n "Comment: " && read comment                   && \
    git add .                                             && \
    git commit -am "$comment"                             && \
    git push                                              && \
    echo "Published: $url"                                && \
    echo "This takes a few minutes for GitHub to update"  && \
    repo_url
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

git status -s | grep -v ' docs/' | grep -v " md/" | sed 's/^/[git]/'

update_book_readme

git_check_untracked
git_no_updates && echo "[publish] No changes to publish" && exit 1

publish
