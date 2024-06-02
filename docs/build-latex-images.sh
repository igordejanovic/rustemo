#!/bin/sh
# Creates images from LaTeX files.
while IFS= read -d '' -r file ; do
    printf 'File found: %s\n' "$file"
    pdflatex -output-directory=$(dirname $file) $file 2>&1 >> ${file%.tex}.log
    pdftoppm ${file%tex}pdf ${file%.tex} -png -singlefile
    #convert -density 200 -alpha deactivate ${file%.tex}.pdf ${file%.tex}.png
    # rm ${file%.tex}.{log,pdf,aux}
done < <(find . -iname '*.tex' -print0)
