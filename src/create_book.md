# Create a Book

Create a markdown book with mdbook crate.
This is the crate used to create the office Rust Programming Book.
It is also the crate used to this book.
If you want to get fancy you can add preprocessors so that you can have your own custom markdown (like I am doing).

- cargo install mdbook
- mkdir mybook
- cd mybook
- mdbook init

For my preprocessor I googled and found [mdbook-toc](https://github.com/badboy/mdbook-toc/blob/master/Cargo.toml)
My preprocessor so far is just a simple global search an replace.
Preprocessors are much more powerful than that, they have access to the entire parsed token tree. This allows you much finer grain control over the substitutions.

To publish the book I use a simple shell script:

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

As I am editing the book, I have ```mdbook watch --open``` running which opens a browser and regenerates the book every time it changes.

## Preprocessors

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
