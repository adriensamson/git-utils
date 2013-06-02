#!/bin/bash

tmp=$(mktemp)

if git status -b -s > $tmp 2>/dev/null
then
  echo -ne "("
  BRANCH_LINE=$(grep '^##' $tmp | colrm 1 3)
  echo -ne "\001\033[01;35m\002"$(echo $BRANCH_LINE | sed 's/\.\.\..*//')"\001\033[0m\002"
  if echo $BRANCH_LINE | grep behind >/dev/null
  then
   echo -ne "↓"$(echo $BRANCH_LINE | sed 's/.* \([0-9]\+\)\]/\1/')
  fi
  if echo $BRANCH_LINE | grep ahead >/dev/null
  then
    echo -ne "↑"$(echo $BRANCH_LINE | sed 's/.* \([0-9]\+\)\]/\1/')
  fi

  STAGED=$(grep '^[MADRC]' $tmp | wc -l)
  if [ $STAGED -gt 0 ]
  then
    echo -ne "|\001\033[01;32m\002"s$STAGED"\001\033[0m\002"
  fi

  NOTSTAGED=$(grep '^.[MD]' $tmp | wc -l)
  if [ $NOTSTAGED -gt 0 ]
  then
    echo -ne "|\001\033[01;31m\002"m$NOTSTAGED"\001\033[0m\002"
  fi

  UNTRACKED=$(grep '??' $tmp | wc -l)
  if [ $UNTRACKED -gt 0 ]
  then
    echo -ne "|\001\033[01;33m\002"u$UNTRACKED"\001\033[0m\002"
  fi

  echo -ne ")"
fi
rm $tmp
