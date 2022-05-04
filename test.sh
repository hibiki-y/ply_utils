#!/bin/bash
DAY=$(date +"%m%d")
TIME=$(date +"%H%M")
YEAR=$(date +"%y")
# DATE=$(date +"%m%d%H%M%y")
# LOG=./log/test_${DATE}.log
LOG=./log/test_${DAY}_${TIME}_${YEAR}.log
LINE="---------"
echo $LINE >> $LOG
echo "TEST START" >> $LOG
date >> $LOG
echo $LINE >> $LOG
cargo run -p exec  &>> $LOG
echo $LINE >> $LOG
echo "TEST FINISH" >> $LOG
date >> $LOG
echo $LINE >> $LOG