#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
GRAMMAR_DIR="$( realpath "$SCRIPT_DIR/../grammar" )"
GRAMMAR_FILE="$( realpath "$SCRIPT_DIR/../grammar.pest" )"

# echo info: locating grammar files...
readarray -d '' FILES < <(find $GRAMMAR_DIR -name "*.pest" -print0)
# iterate over files and print their relative paths
# for file in "${FILES[@]}"
# do
#     echo - $( realpath --relative-to="${PWD}" "$file" )
# done
# echo info: found ${#FILES[@]} grammar files

# remove grammar.pest
# echo info: removing existing grammar file...
rm -rf grammar.pest
# generate the new grammar by concatenating files
# echo info: generating grammar file...
touch $GRAMMAR_FILE
for file in "${FILES[@]}"
do
    # add comment to grammar file
    echo "// source: $( realpath --relative-to="${GRAMMAR_DIR}" "$file" )" >> $GRAMMAR_FILE
    cat $file >> $GRAMMAR_FILE
done
echo info: generated grammar.pest $( du -sh $GRAMMAR_FILE | awk '{ print $1 }' )
