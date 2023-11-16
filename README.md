# gitalias rust

a rewrite of my personal gitalias from **C++** to **rust**

###### because in every c++ developer is a future rust developer

# gitalias

A simple terminal program that simply parses your arguments\
in to a bunch of **git commands** that is passed into bash and runned

# Examples

```bash
gitlias-rust -i -a . -c -m 'commit added' -r 'test' -D 'test repo' -t true -o punixcorn/test
```

#### will be Translated into

```bash
# -i 
git init 
# -a ...
git add . 
# -c -m '...'
git commit -m 'commit added'

# -r , -D , -t will  create a repo online with your key called "test" 
# with the description of "test repo" and publicity : true

# -o 
git remote add origin  punixcorn/test 
git branch -M main 
git push -u origin main
```

---

# Special cases

```sh
ga -c
```

- where you do not have a local repository created it will ask\
  and automatically create it for you
- since `add` was not specified it will add all files
- since `message` was not specified it will use a default message

#### Translation:

```sh
git init 
git add .
git commit -m "init commit" # will use `init commit` if the local
# repository was just created else it will use `changes made`
```

# Build

```bash
cargo build
```

### binary should be in `./target/debug/gitalias-rust`
