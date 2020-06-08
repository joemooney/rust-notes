#!/bin/bash

url=$(git config --get remote.origin.url | sed 's,git@github.com:,,;s,/,.github.io/,;s,^,https://,;s,.git$,/,')

mdbook build                         && \
git status                           && \
echo 'git commit -am'                && \
echo -n "Comment: " && read comment  && \
git commit -am "$comment"            && \
git push                             && \
echo "Published: $url"
