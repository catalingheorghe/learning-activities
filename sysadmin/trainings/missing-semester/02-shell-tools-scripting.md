https://missing.csail.mit.edu/2020/shell-tools/

## Shell scripting basics

Info about variables, importance of spaces in assignment, special shell
characters.

**tip** 
`sudo !!` - execute last command with `sudo`

Exit codes and conditional operators (`&&`, `||`).

Command substitution - `$(ls)`
Process substitution - `<(ls bar)` 
  - places the output in a temporary file.

`test` command; - `[[` - more portable than `[`.

Globbing - wildcards (`*, ?`), curly braces (`{xcs,dsad} {0..2}`. Can also do
carthesian product of all of them.

**tip** [1]
you can see the expansion of globbing before Enter - there is a
function `glob-expand-word` that is usually already bound to a key combination
with 'C-x'.
Do a `bind -p | grep 'C-x'`to see. 
Can be modified in `~/.inputrc`.
Default is `C-x*` on ubuntu 18.04 at least.

**tip** [2]
Use `shellcheck` to check your scripts (static analysis).

Which interpreter should the shell use to run a script - `shebang` line. Good
practice for protability, use `#!/usr/bin/env python`, for example.

Functions vs shell scripts
  - executing shell scripts is done in their own process.

## Shell tools

**tldr** use tldr for quick info about common usage of commands. [3]

You can find files, but also execute commands with `find`.

	``` 
    # Find all PNG files and convert them to JPG find . -name '*.png'
	-exec convert {} {.}.jpg \; 
	```

**fd** use `fd` as an alternative - quicker, more user friendly options. [4]

ubuntu install 

    ```
	$ wget https://github.com/sharkdp/fd/releases/download/v8.1.1/fd-musl_8.1.1_amd64.deb
    $ sudo dpkg -i fd-musl_8.1.1_amd64.deb 
	$ rm fd-musl_*.deb
	```

To search for patterns - `grep`. Common flags - `-C, -v, -R`. Recursive grep
can be improved -> consider `ack, ag, rg`.

**rg** (ripgrep)

example: 

   ```
   $ rg -t c -c "include .hmpf"
   ``` 

print files and count of matches that include something from hmpf

ubuntu install 

   ``` 
   $ wget https://github.com/BurntSushi/ripgrep/releases/download/12.1.1/ripgrep_12.1.1_amd64.deb
   $ sudo dpkg -i ...  
   $ rm ...
   ```
**fzf** fuzzy finder [5]

Powerfull fuzzy finder that can work on history, files autocompletion, location
switcher. Works on any list of strings, like the list of git branch, for
example, or the output of find.

Installs some default key binding for use in reverse history commands,
searching for files, searching for location to change into.

Has a vim plugin also.

install

   ```
   git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
   ~/.fzf/install
   ```

**tip**
Starting a command with leading space will not put that command into history.
Also, any entry can be deleted from history (file `.bash_history`).

**fasd** [6]

This could be another useful tool - allows to change to recent directories,
open recent files and other things. Could be tried out as well.

Note: a single script, cool to checkout the repo - man page, simple makefile.

## Exercises



## Resources

  - https://www.tldp.org/LDP/abs/html/special-chars.html
  - https://www.man7.org/linux/man-pages/man1/test.1.html
  - [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix))
  - https://www.man7.org/linux/man-pages/man1/env.1.html
  - https://www.man7.org/linux/man-pages/man1/find.1.html
  - https://unix.stackexchange.com/questions/60205/locate-vs-find-usage-pros-and-cons-of-each-other


  [1] https://superuser.com/questions/215950/how-to-expand-on-bash-command-line
  [2] https://github.com/koalaman/shellcheck 
  [3] https://tldr.sh/ 
  [4] https://github.com/sharkdp/fd
  [5] https://github.com/junegunn/fzf
  [5] https://github.com/junegunn/fzf/wiki/Configuring-shell-key-bindings#ctrl-r
  [6] https://github.com/clvv/fasd

