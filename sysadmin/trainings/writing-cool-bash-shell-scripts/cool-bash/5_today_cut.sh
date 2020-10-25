#!/bin/bash
#
# displays current day, month, year

DATE=`date +%d-%m-%y`
echo $DATE

TODAY=$(echo $DATE | cut -d '-' -f 1) 
THISMONTH=$(echo $DATE | cut -d '-' -f 2)
THISYEAR=$(echo $DATE | cut -d '-' -f 3)

echo Today is $TODAY
echo This month is $THISMONTH
echo This year is $THISYEAR

echo or directly with awk
echo $(date +%d-%m-%y) | awk -F- '{ print "day " $1 " month " $2 " year " $3}'


