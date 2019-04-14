#!/bin/bash

CWD=$(pwd)

cd
mkdir playground

cd playground

mkdir dir1 dir2

cp /etc/passwd .
ls -l

cp -v /etc/passwd .
ls -l

cp -i -v /etc/passwd .
ls -l

mv passwd fun

mv fun dir1
mv dir1/fun dir2
mv dir2/fun .

mv fun dir1

mv dir1 dir2
ls -l dir2
ls -l dir2/dir1

mv dir2/dir1 .
mv dir1/fun .
ls -l

# hard links
ln fun fun-hard
ln fun dir1/fun-hard
ln fun dir2/fun-hard
ls -li

# symlinks
ln -s fun fun-sym
ln -s ../fun dir1/fun-sym # a symlink is a text reference!
ln -s ../fun dir2/fun-sym # a symlink is a text reference!
ls -li . dir1

ln -s dir1 dir1-sym
ls -l

# rm
rm fun-hard
ls -l

rm -i fun
ls -l
file fun-sym # broken link

rm fun-sym dir1-sym

cd
rm -r playground

cd ${CWD}

