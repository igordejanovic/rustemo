# Lexers

Rustemo provides lexer for parsing text which uses recognizers defined in the
lexical part of the grammar (after keyword `terminals`) for lexing. If you have
special requirements you can write your own lexer.




```admonish todo
Fix the following section
```

### Custom recognizers

If you are parsing arbitrary input (non-textual) you'll have to provide your own
recognizers. In the grammar, you just have to provide terminal symbol without
body, i.e. without string or regex recognizer. You will provide missing
recognizers during grammar instantiation from Python. Although you don't supply
body of the terminal you can define [disambiguation rules](./disambiguation.md)
as usual.

Lets say that we have a list of integers (real list of Python ints, not a text
with numbers) and we have some weird requirement to break those numbers
according to the following grammar:

    Numbers: all_less_than_five  ascending  all_less_than_five;
    all_less_than_five: all_less_than_five  int_less_than_five
                      | int_less_than_five;
    
    
    terminals
    // These terminals have no recognizers defined in the grammar
    ascending: ;
    int_less_than_five: ;

So, we should first match all numbers less than five and collect those, than we
should match a list of ascending numbers and than list of less than five again.
`int_less_than_five` and `ascending` are terminals/recognizers that will be
defined in Python and passed to grammar construction. `int_less_than_five` will
recognize Python integer that is, well, less than five. `ascending` will
recognize a sublist of integers in ascending order.

More on this topic can be found in [a separate section](./recognizers.md).

