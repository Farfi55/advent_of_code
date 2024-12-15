#!/usr/bin/bash

# get day number from the argument
day=$1

# if no argument is given, use the last day_dir + 1
if [ -z $day ]; then
	day=$(ls -d day_* | sort -n | tail -n 1 | sed 's/day_0\{0,1\}//')
	day=$((day + 1))

fi

# make sure it has 2 digits
if [ $day -lt 10 ]; then
	day="0$day"
fi

day_dir="day_$day"

# if there is no day_00 directory, abort
if [ ! -d day_00 ]; then
	echo "day_00 directory not found"
	exit 1
fi

# create a new directory
cp -r day_00 $day_dir
cd $day_dir

sed -i "s/^name = \".*\"/name = \"$day_dir\"/" Cargo.toml
sed -i "s/^name = \".*\"/name = \"$day_dir\"/" Cargo.lock

code . src/bin/*
