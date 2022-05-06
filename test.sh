#!/bin/bash
DAY=$(date +"%m%d")
TIME=$(date +"%H%M")
YEAR=$(date +"%y")
INPUT_PATH=./test/original_test.ply
OUTPUT_PATH=./out/final_cut_auto_test.ply

LOG=./log/test_${DAY}_${TIME}_${YEAR}.log
LINE="---------"
echo $LINE >> $LOG
echo "TEST START" >> $LOG
date >> $LOG
echo $LINE >> $LOG
cargo run -- cut -p x ny red -i $INPUT_PATH -o $OUTPUT_PATH &>> $LOG
echo $LINE >> $LOG
echo "TEST FINISH" >> $LOG
date >> $LOG
echo $LINE >> $LOG