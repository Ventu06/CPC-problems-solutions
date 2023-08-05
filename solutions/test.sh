#!/bin/sh

cd $1
rustc $1.rs

n=$(ls input* | wc -l)

for i in $(seq 0 $(($n-1))); do
	exp_out=$(cat output$i.txt)
	act_out=$(cat input$i.txt | ./$1)
	diff=$(diff <(echo "$exp_out") <(echo "$act_out"))
	if [ -z "$diff" ]; then
		echo "Test $i passed"
	else
		echo "Test $i failed"
		echo ""
		echo "Expected output:"
		echo "$exp_out"
		echo ""
		echo "Actual output:"
		echo "$act_out"
		echo ""
		#echo "Difference:"
		#echo "$diff"
		#echo ""
		echo ""
	fi
done
