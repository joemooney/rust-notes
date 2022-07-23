# Create a Book

How to create a markdown book using `mdbook` crate.
This is the same crate used to create the official
Rust Programming Book.
It is also the crate used to this book.
`mdbook` allows you to add preprocessors so that you can have your own custom markdown extensions (like I do with `mdbook-plus`).

- cargo install mdbook
- mkdir mybook
- cd mybook
- mdbook init

Another example preprocessor is [mdbook-toc](https://github.com/badboy/mdbook-toc/blob/master/Cargo.toml)
This book's git repo includes my `mdbook-plus` preprocessor that adds question/answer and other capabilities.
For a Table of Contents, I use `mdbook-toc`.

Preprocessors are powerful, they have access to the entire parsed token tree. This allows for finer grain control over changes. ⚠️ `mdbook-plus` is very rudimentary.

To publish this book there is a simple shell script `publish.sh` the outline of which is:

```bash

url=$(git config --get remote.origin.url | sed 's,git@github.com:,,;s,/,.github.io/,;s,^,https://,;s,.git$,/,')

    lang=shell
    mdbook build                                          && \
    postprocess                                           && \
    rsync -avx --delete --info=progress2 ./book/ ./docs/  &&\
    git status                                            && \
    echo 'git commit -am'                                 && \
    echo -n "Comment: " && read comment                   && \
    git add .                                             && \
    git commit -am "$comment"                             && \
    git push                                              && \
    echo "Published: $url

```

There are some more guards such as checking for no git untracked files and no other git uncommitted updates.

As I am editing the book, I have ```watch.sh``` running which opens a browser and regenerates the book every time it changes.

## mdbook-mermaid Preprocessor

Hmm, I am not using this any longer?
Looks like I switched to use `plantuml` (not sure why)

Upon ```cargo install mdbook-mermaid``` in *book.toml* add:

```toml
[preprocessor.mermaid]
renderer = ["html", "epub"]
```

Then we go:
stateDiagram
    state "Also Cool" as s1
    s1: foo/bar
    [*] --> Cool
    Cool --> s1
    s1 --> Coolest
    Coolest --> [*]
