#!/bin/bash

tmp=$(mktemp)

if git status -b -s > $tmp 2>/dev/null
then
  echo -ne "("
  BRANCH_LINE=$(grep '^##' $tmp | colrm 1 3)
  echo -ne "\001\033[01;35m\002"
  echo -n $BRANCH_LINE | sed 's/\.\.\..*//'
  echo -ne "\001\033[0m\002"

  if echo $BRANCH_LINE | grep behind >/dev/null
  then
   echo -ne "↓"$(echo $BRANCH_LINE | sed 's/.* \([0-9]\+\)\]/\1/')
  fi
  if echo $BRANCH_LINE | grep ahead >/dev/null
  then
    echo -ne "↑"$(echo $BRANCH_LINE | sed 's/.* \([0-9]\+\)\]/\1/')
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
