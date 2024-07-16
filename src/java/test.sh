#!/bin/sh

cat ../../resources/csv-testcases/RMLTC0001a-CSV/mapping.ttl | java -Djava.library.path=$(pwd)/../../target/release Translator
