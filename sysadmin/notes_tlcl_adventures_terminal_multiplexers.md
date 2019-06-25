# Terminal Multiplexers

*Source: [TLCL terminal multiplexers](http://linuxcommand.org/lc3_adv_termmux.php)*

## General

When computer terminals connected to a central Unix server provided you with an 80 char wide, 24 line high display with a single "window" blinking the shell prompt, having a way to show multiple sessions and applications was an important aspect. Modern desktop environments now have terminal emulators (like gnome-terminal or kconsole) that offer multiple tabs, besides the fact that you can open multiple windows. Nevertheless, terminal multiplexers still have some features that can increase the CLI user's productivity.

## GNU Screen

GNU screen was the first terminal multiplexer. screen manages a session, that can have one or more windows each containing a shell. Also, screen can divide the terminal display into multiple regions each with the contents of a window.

To control screen a "command prefix" is used: `Ctrl-a`. 

 - `Ctrl-a c` - create a new window
 - `Ctrl-a p` - move to previous window
 - `Ctrl-a n` - next window
 - `Ctlr-a "` - list windows
 - `Ctrl-a 0..9` - select window
 - `Ctrl-a ?` - list of all commands

 - `Ctrl-a A` - change name of window

 - `Ctlr-a k` - close a window (by killing the program that is refusing to terminate gracefully)

It is possible to create windows and run programs without and underlying shell: `$ screen vim ~/.bashrc`. This also work in a screen session, it will create a new window, not a new screen session inside the current screen session.

To inspect, **copy and paste** output text, screen offers "scrollback" mode.

 - `Ctrl-a [` - enter scrollback mode
 - pressing space marks the beginning of the text, then move to end and press space again; this will copy the text and exit scrollback mode
 - `Ctrl-a ]` - paste the copied text

Screen can divide the terminal display into **regions**. Note that a region is independent from a window. When created, a region has nothing inside it: either you create a new window, or select one to display. Also, deleting regions does not kill the windows.

 - `Ctrl-a S` - split horizontally
 - `Ctrl-a |` - split vertically
 - `Ctrl-a TAB` - move focus
 - `Ctrl-a Q` - remove all regions except current one
 - `Ctrl-a X` - remove current region

**Detaching sessions** is the most interesting feature. You can start a screen session on one PC, detach from it, go to another PC, log in remotely and attach to the previous session. All programs continued to run inside the session.

 - `screen -list` - list running screen sessions. To identify a screen session, if there are more than one: `pid.tty.host` as argument to -d / -D / -r / -R
 - `screen -d -r` - detach a session from previous terminal and reattach it to the current one
 - `screen -D -R` - detach a session from previous terminal, log the user off the old terminal, attach session to new terminal (create a new session if none existed)

To **customize** screen the used files are: `/etc/screenrc` and `~/.screenrc` (if they exist). Below is a sample file.

    ```
    # This is a comment
    
    # Set some key bindings
    
    bind k              # Un-bind the "k" key (set it to do nothing)
    bind K kill         # Make `Ctrl-a K` destroy the current window
    bind } history      # Make `Ctrl-a }` copy and paste the current
                        # command line
    
    # Define windows 7, 8, and 9 at startup
    
    screen -t "mdnght cmdr" 7 mc
    screen -t htop 8 htop
    screen -t syslog 9 tailf /var/log/syslog
    ```

For the list of screen commands, see the man page (like history).

As it can be seen, windows with programs already running can be created at the startup of a screen session.

## tmux

GNU screen has gathered criticism that the code is complex, to the point of unmaintainable, resource hungry and not actively developed. This allowed tmux to gain momentum.

The complete invocation is: `tmux new -s "my sessions" -n "window 1"`, giving a name to the session and to the first window. All of that is option, even the *new* command.

To control tmux, a command prefix is also used, like in GNU screen, but it is `Ctrl-b`. Which of bash is not bad, since `Ctrl-a` moves the cursor to the beginning of the line.

 - `Ctrl-b ?` - help, show key bindings
 - `Ctrl-b c` - create a new window
 - `Ctrl-b n` - next window
 - `Ctrl-b p` - previous window
 - `Ctlr-b 0..9` - window number 0..9
 - `Ctrl-b w` - show window list
 - `Ctrl-b ,` - rename current window

A new window can also be created with `tmux neww -n Window1` from the current window.

Like screen, tmux can device the terminal display into sections called **panes**. But unlike screen, a pane is a complete pseduo-terminal associated with that window, so a single tmux window can contain multiple terminals.

 - `Ctrl-b "` - split horizontally
 - `Ctrl-b %` - split veritically
 - `Ctrl-b arrow` - move to pane
 - `Ctrl-b Ctrl-arrow` - resize pane by 1 character
 - `Ctrl-b Alt-arrow` - resize pane by 5 characters
 - `Ctrl-b x` - destroy current pane (destroy the pseudo-terminal and associated program)

Just for fun

 - `Ctrl-b t` - display a digital clock in the current pane.

It is possible to break a pane out into a window of its own

 - `Ctrl-b !`

**Copy mode** is invoked like in screen with `Ctrl-b [`.

 - `Ctrl-b [` - enter copy mode/scrollback mode
 - mark text for copying by starting with `Ctrl-space`, then move the cursor and copy the text with `Alt-w` (also exits copy mode)
 - `q` - exit copy mode
 - `Ctrl-b ]` - paste copied text

**Managing multiple session** is easier than with screen. Sessions can have descriptive names. You can switch sessions on the fly by displaying a list of sessions and switching to another one.

 - `Ctrl-b $` - rename current session
 - `Ctrl-b s` - display list of sessions
 - `Ctrl-b d` - detach from session
 - `tmux ls` - list sessions
 - `tmux attach -d -t <session_name>` - attach to a session; -d will detach the session from the previous terminal; note that you can omit this and the session will be attached to both terminal at the same time

**Customizing** tmux is even more flexible than GNU screen. At startup, it reads the files `/etc/tmux.conf` and `~/.tmux.conf` if they exist, but it also has a `-f` command line option for an alternat configuraiton file.

An example configuration file is below. It changes the prefix key and creates a session with some windows and panes. Since it creates a session, tmux should be called with `attach` command to not create the default session.

    ```
    # Sample tmux.conf file
    
    # Change the command prefix from Ctrl-b to Ctrl-a
    unbind-key C-b
    set-option -g prefix C-a
    bind-key C-a send-prefix
    
    #####
    # Create session with 4 windows
    #####
    
    # Create session and first window
    new-session -d -s MySession
    
    # Create second window and vertically split it
    new-window
    split-window -d -h
    
    # Create third window (and name it) running Midnight Commander
    new-window -d -n MdnghtCmdr mc
    
    # Create fourth window (and name it) running htop
    new-window -d -n htop htop
    
    # Give focus to the first window in the session
    select-window -t 0
    ```

## byobu

TODO


