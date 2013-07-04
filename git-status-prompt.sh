#!/bin/bash

tmp=$(mktemp)

if git status --porcelain > $tmp 2>/dev/null
then
  BRANCH=$(git symbolic-ref HEAD)
  BRANCH=${BRANCH#refs/heads/}
  echo -ne "("
  echo -ne "\001\033[01;35m\002"
  echo -n $BRANCH
  echo -ne "\001\033[0m\002"

  REMOTE=$(git config branch.$BRANCH.remote)
  UPSTREAM=$(git config branch.$BRANCH.merge)
  UPSTREAM=${UPSTREAM#refs/heads/}
  AHEAD=$(git log $REMOTE/$UPSTREAM..$BRANCH --oneline | wc -l)
  BEHIND=$(git log $BRANCH..$REMOTE/$UPSTREAM --oneline | wc -l)
  if [[ $BEHIND -gt 0 ]]
  then
   echo -ne "↓"$BEHIND
  fi
  if [[ $AHEAD -gt 0 ]]
  then
    echo -ne "↑"$AHEAD
  fi

  CONFLICT=$(grep '^\(DD\|AA\|.U\|U.\)' $tmp | wc -l)
  if [ $CONFLICT -gt 0 ]
  then
    echo -ne "|\001\033[01;31m\002"
    echo -n $CONFLICT
    echo -ne "\001\033[0m\002"
  fi


  STAGED=$(grep '^\([MARC]\|D[ M]\)' $tmp | wc -l)
  if [ $STAGED -gt 0 ]
  then
    echo -ne "|\001\033[01;33m\002"
    echo -n $STAGED
    echo -ne "\001\033[0m\002"
  fi

  NOTSTAGED=$(grep '^\([ MARC][MD]\|DM\)' $tmp | wc -l)
  if [ $NOTSTAGED -gt 0 ]
  then
    echo -ne "|\001\033[01;34m\002"
    echo -n $NOTSTAGED
    echo -ne "\001\033[0m\002"
  fi

  UNTRACKED=$(grep '??' $tmp | wc -l)
  if [ $UNTRACKED -gt 0 ]
  then
    echo -ne "|\001\033[01;36m\002"
    echo -n $UNTRACKED
    echo -ne "\001\033[0m\002"
  fi

  echo -ne ")"
fi
rm $tmp
