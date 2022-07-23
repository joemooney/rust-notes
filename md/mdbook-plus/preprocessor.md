# Custom mdbook-plus preprocessor for mdbook

## mdbook-plus

The mdbook-plus is a rudimentary preprocessor for mdbook which does simple search and replace without tokenizing the input files.
It takes as stdin markdown text with custom markdown and converts these substrings to HTML.

- Colorized Text
- Q&A Blocks

Here we take a look at the markdown you write looks like before and after going through the preprocessor - the markdown output by the preprocessor is fed into by mdbook to generate the final html for the book (in the `book` subdirectory) and see how it is rendered.
Note that the custom markers are replaced with html which the mdbook markdown to html leaves as is.
Also note you can use '[#open_bracket\]' instead of brackets etc. from being processed as markdown for the special cases - though you probably will never need to worry about that.

The preprocessing happens as part of `mdbook build`.
`publish.sh` will run this and publish the generated book to github.

### Before Colorized Preprocessor (markdown who you write)

```markdown
- text color [#open_bracket]red[#close_bracket]red font[#open_bracket]/red[#close_bracket] [#open_bracket]blue[#close_bracket]blue[#open_bracket]/blue[#close_bracket] [#open_bracket]green[#close_bracket]green[#open_bracket]/green[#close_bracket] [#open_bracket]yellow[#close_bracket]yellow[#open_bracket]/yellow[#close_bracket] [#open_bracket]gray[#close_bracket]gray[#open_bracket]/gray[#close_bracket]<br>
- subscripted text [#open_bracket]small[#close_bracket]subscript font[#open_bracket]/small[#close_bracket]
```

### After Colorized Preprocessor

```markdown
- text color {red}red font{/red} {blue}blue{/blue} {green}green{/green} {yellow}yellow{/yellow} {gray}gray{/gray}
- subscripted text {small}subscript font{/small}
```

### Final Rendering by mdbook

- text color {red}red font{/red} {blue}blue{/blue} {green}green{/green} {yellow}yellow{/yellow} {gray}gray{/gray}
- subscripted text {small}subscript font{/small}

## Question/Answer Blocks

Support collapsable question/answer blocks where the answer is hidden until you click the dropdown arrow.
For this we substitute into the html details tag.

### Before Q&A Preprocessor (markdown who you write)

> [#question_mark]Q Example question goes
>    here until we get to the answer<br>
> [#question_mark]A You answer goes here until we get to the end<br>
>    but it is hidden so that the reader has a chance<br>
>    to think before they reveal the answer<br>
> [#question_mark]E

### After Q&A Preprocessor (html after preprocessing)

> {question}Example question goes here until we get to the answer ?{answer}You answer goes here until we get to the end but it is hidden so that the reader has a chance to think before they reveal the answer{/question}

### Final Q&A Rendering in Browser

{question}Example question goes here until we get to the answer ?{answer}You answer goes here until we get to the end but it is hidden so that the reader has a chance to think before they reveal the answer{/question}

## Install

`cargo install --path . --bin mdbook-plus`

## Escaping

- The preprocessing is just dumb string replacement. So, if you want literal \{red\} in your output then you can put a backslash before ending brackets so the text substitution does not match (i.e. \{red\\\} which should show as {red\})
- You can also put [#open_bracket\]red[#close_bracket\] and likewise for '[#dash\]' for '-', '[#question_mark\]' for '?'.

## Install `mdbook-plus`

`cargo install --path . --bin mdbook-plus`

## Escaping Braces

- The preprocessing is just dumb string replacement. So, if you want literal \{red\} in your output then you can put a backslash before ending brackets so the text substitution does not match (i.e. \{red\\\} which should show as {red\})
- You can also put [#open_bracket\]red[#close_bracket\] and likewise for '[#dash\]' for '-', '[#question_mark\]' for '?'.

### Markdown with escaped characters

```markdown
[#question_mark\]Q Example question goes
   here until we get to the answer<br>
[#question_mark\]A You answer goes here until we get to the end<br>
   but it is hidden so that the reader has a chance<br>
   to think before they reveal the answer<br>
[#question_mark\]E

```

```markdown
- text color [#open_bracket\]red[#close_bracket\]red font[#open_bracket\]/red[#close_bracket\] [#open_bracket\]blue[#close_bracket\]blue[#open_bracket\]/blue[#close_bracket\] [#open_bracket\]green[#close_bracket\]green[#open_bracket\]/green[#close_bracket\] [#open_bracket\]yellow[#close_bracket\]yellow[#open_bracket\]/yellow[#close_bracket\] [#open_bracket\]gray[#close_bracket\]gray[#open_bracket\]/gray[#close_bracket\]<br>
- subscripted text [#open_bracket\]small[#close_bracket\]subscript font[#open_bracket\]small[#close_bracket\]
```

## Limitations

- Since these special markers in the input markdown files are not legitimate for normal markdown, intellisense in VS Code (for example) may flag lines with warnings.
- Note: `code in backticks` does not render correctly inside color blocks.

```
