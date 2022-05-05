#!/bin/bash
DAY=$(date +"%m%d")
TIME=$(date +"%H%M")
YEAR=$(date +"%y")

LOG=./log/test_${DAY}_${TIME}_${YEAR}.log
LINE="---------"
echo $LINE >> $LOG
echo "TEST START" >> $LOG
date >> $LOG
echo $LINE >> $LOG
cargo run -- cut -c 3  &>> $LOG
echo $LINE >> $LOG
echo "TEST FINISH" >> $LOG
date >> $LOG
echo $LINE >> $LOG