#!/bin/bash
mkdir chapter${1}
for i in `seq "${2}"`
do
    echo "${i}"
    touch ./chapter"${1}"/"${1}"_"${i}"_.md
done
# sh mkchapter.sh [chapterNo] [section count]