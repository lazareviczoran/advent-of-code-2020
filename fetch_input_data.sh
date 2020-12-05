#!/bin/bash

DAY=$(date|awk '{ printf("%02d\n", $3) }')
cd ~/projects/advent-of-code-2020
if [[ -z $(ls|grep "day-$DAY") ]]; then
    cargo init day-$DAY
fi
cd day-$DAY
cp ~/Library/Application\ Support/Firefox/Profiles/*.default-release/cookies.sqlite ./cookies_copy.sqlite

SESSION_COOKIE=$(sqlite3 ./cookies_copy.sqlite "select name,value from moz_cookies where host = '.adventofcode.com'"\
    | sed 's/|/=/')
rm ./cookies_copy.sqlite*

curl -b $SESSION_COOKIE https://adventofcode.com/2020/day/$(echo $DAY| bc)/input > input.txt