#!/bin/bash

# exit code 3 - no arg provided

echo ${1}

[ -z $1 ] && exit 3
[ -f $1 ] && vi $1
[ -d $1 ] && cd $1
# this will not work ok - cd is change inside the subshell
# but cd ${1} && exec bash will start a new shell

#file ${1} | grep directory 2> /dev/null && IS_DIR=yes || IS_DIR=no
#echo $IS_DIR
#[ "$IS_DIR" = "yes" ] && cd ${1} || vi ${1}

