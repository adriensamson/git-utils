git-status-prompt
=================

## Installation

Add `$(/path/to/git-status-prompt/git-status-prompt.sh)` to the PS1 variable in your `~/.bashrc`.

For instance, mine is

     PS1='${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]$(/home/adrien/workspace/git-status-prompt/git-status-prompt.sh)\$ '

