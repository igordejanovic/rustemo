#!/bin/sh
# Creates images from LaTeX files.

# Run only for html renderer
if [  $2 != 'html' ]; then
    exit 1
fi

while IFS= read -d '' -r file ; do
    printf 'File found: %s\n' "$file"
    pdflatex -output-directory=$(dirname $file) $file 2>&1 >> ${file%.tex}.log
    pdftoppm ${file%tex}pdf ${file%.tex} -png -singlefile
    #convert -density 200 -alpha deactivate ${file%.tex}.pdf ${file%.tex}.png
    # rm ${file%.tex}.{log,pdf,aux}
done < <(find . -iname '*.tex' -print0)

# This is required for the script to be called as mdbook preprocessor
# as we don't wan't actually to process md files.
exit 1
