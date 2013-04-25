#!/bin/bash

tmp=$(mktemp)

if git status -b -s > $tmp 2>/dev/null
then
  echo -ne "("
  BRANCH_LINE=$(grep '^##' $tmp | colrm 1 3)
  echo -ne "\e[01;35m"$(echo $BRANCH_LINE | sed 's/\..*//')"\e[0m"
  if echo $BRANCH_LINE | grep behind >/dev/null
  then
   echo -ne "↓"$(echo $BRANCH_LINE | sed 's/.* \([0-9]\+\)\]/\1/')
  elif echo $BRANCH_LINE | grep ahead >/dev/null
  then
    echo -ne "↑"$(echo $BRANCH_LINE | sed 's/.* \([0-9]\+\)\]/\1/')
  fi

  STAGED=$(grep '^[MADRC]' $tmp | wc -l)
  if [ $STAGED -gt 0 ]
  then
    echo -ne "|\e[01;32m$STAGED\e[0m"
  fi

  NOTSTAGED=$(grep '^.[MD]' $tmp | wc -l)
  if [ $NOTSTAGED -gt 0 ]
  then
    echo -ne "|\e[01;31m$NOTSTAGED\e[0m"
  fi

  UNTRACKED=$(grep '??' $tmp | wc -l)
  if [ $UNTRACKED -gt 0 ]
  then
    echo -ne "|\e[01;33m$UNTRACKED\e[0m"
  fi

  echo -ne ")"
fi
rm $tmp
