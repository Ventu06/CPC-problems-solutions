#!/bin/sh

cd $1
rustc $1.rs

n=$(ls input* | wc -l)

for i in $(seq 0 $(($n-1))); do
	out=$(cat input$i.txt | ./$1)
	diff=$(echo $out | diff output$i.txt -)
	if [ -z "$diff" ]; then
		echo "Test $i passed"
	else
		echo "Test $i failed"
		echo ""
		echo "Expected output:"
		cat output$i.txt
		echo ""
		echo "Actual output:"
		echo "$out"
		exit
	fi
done
