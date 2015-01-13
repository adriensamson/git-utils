# Git utils

## Bash status prompt

### Installation

* Add git-status-prompt in your PATH `ln -s /path/to/git-utils/git-status-prompt ~/bin/git-status-prompt`
* Add `$(git-status-prompt)` to the PS1 variable in your `~/.bashrc`.

For instance, mine is
```sh
PS1='${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]$(git-status-prompt)\$ '
```

Quick install script:
```
sed -i "s@\(\\\\\]\)\(\\\\\$ '\)@\1\$($(pwd)/git-status-prompt)\2@" ~/.bashrc
```

## Aliases

* `git a` adds all files to staging area
* `git u` pulls changes from remote branch (ff-only)
* `git r` rebases local branch from corresponding remote branch
* `git p` pushes current branch to corresponding remote branch
* `git s` for status
* `git d` for diff
* `git ds` for diff to be commited
* `git ci` for commit
* `git co` for checkout

### Installation

In your `~/.gitconfig`:
```
[include]
        path = /path/to/git-utils/aliases
```

Quick install script:
```
cat >>~/.gitconfig <<eof
[include]
    path = $(pwd)/aliases
eof
```

## Global excludes

### Installation

In your `~/.gitconfig`:
```
[core]
        excludesfile = /path/to/git-utils/global-excludes
```

Quick install script:
```
cat >>~/.gitconfig <<eof
[core]
    excludefiles = $(pwd)/global-excludes
eof
```
