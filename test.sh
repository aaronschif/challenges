#! /bin/bash -ex

test ! -z "$1" || (echo "Need command"; exit 1)

dir="$(dirname $1)"

diff <(cat $dir/stdout.txt) <(cat $dir/stdin.txt | ./$1)
