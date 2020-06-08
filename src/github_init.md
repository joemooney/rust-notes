# Storing on github

Create a repo locally:

- ```mdbook rust-book``` ⬅️ this will create the directory with a bare bones book
- ```cd rust-book```
- ```git init```

Then I installed *hub* command to simplify working with github:

- ```sudo snap install [hub](https://hub.github.com/) --classic```
- ```hub create``` > [!TIP]
    > This will create the repo on GitHub, you will be prompted from user/password first time thru.
- ```git remote show origin```
- ```git push --set-upstream origin master```

After creating some content for the book and pushing the changes to GitHub,
I then ran ```mdbook build``` which generated a *book* directory.

- ```mdbook build --open```
- Once you are happy with the changes you can publish.
- ```mv book docs```
- ```git add docs``` ⬅️ this must be the name. It is what GitHub Pages requires.
- ```git commit -m 'adding my book for the first time'```
- On GitHub, under *Settings* for the repo, under *GitHub Pages* section, I selected *Source master branch/docs*
- ```git push```
- The link to your site will be shown as [https://<your_id>.github.io/<your_repo>/](https://joemooney.github.io/rust-notes/)
- After you make changes to the book you can sync the *book* and *docs* directories.
- ```rsync -avx --delete --info=progress2 ./book/ ./docs/```
- ```git push```
