# Run a rust program without Cargo

Here we are using a simple shell script to invoke rustc compiler
and then run the compiled program and remove the bin after executing.
Our shell script is named ```rust`` and we run main.rs by running
the command ```rust main.rs``` on the command line:

```
#!/bin/bash
name=$(basename $1 .rs)
rustc $@ && ./$name && rm $name
```
