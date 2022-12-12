#!/bin/bash

rm -rf test_dir/*
mkdir -p test_dir/space test_dir/cammel

touch test_dir/space/hello\ world{1..10}.txt
touch test_dir/cammel/HelloWorld{1..10}.txt

