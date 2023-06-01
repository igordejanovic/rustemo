# Handling errors

The error can occur at multiple places during development and usage of the
parser. This chapter gives an overview of each class of errors and actions that
can be taken to solve them.

The errors you can encounter with Rustemo are:
- Parser development errors:
    - Syntax errors in the grammar
    - Semantic errors in the grammar - invalid grammar (e.g. undefined symbol,
      infinite loop on a symbol)
    - LR conflicts - non-deterministic grammar (these errors apply only for LR
      not for GLR)
- Parser usage errors:
    - Syntax errors in the parsed input

When an error happens Rustemo tries to provide a detailed information of the
problem. In the sections that follows an overview of each class is given.

# Investigating grammar syntax errors

If your grammar is not written according to the [Rustemo grammar language
rules](../grammar_language.md) the Rustemo compiler will report an error with
the information about the location and expected tokens at that location.

Here is an example of a grammar syntax error:

```
Error at json.rustemo:[3,7]:
	...a] "}";
	Member -->JsonString ":" ...
	Expected one of Colon, OBrace.
```

The error report have a file and line/column location. You can also see the
context where the error occurred with `-->` mark at the position.

# Investigating grammar semantic errors

A grammar may be syntactically correct but still semantically incorrect. For
example, you may have a reference to undefined grammar symbol.

Here is an example of a semantic error:

```
Error at json.rustemo:[3,8-3,17]:
	Unexisting symbol 'JsonStrin' in production '13:  JsonStrin  11  Value '.
```

# Resolving LR conflicts

```admonish note
Please see the [chapter on parsing](./parsing/parsing.md) for the introduction
to LR parsing.
```

LR parsing is based on a deterministic finite-state automata. Because the
generated automaton must be deterministic there can be exactly one operation the
parser can perform at each state and for each input token.

Not all context-free grammars produce a deterministic automaton. Those that do
are called [deterministic context-free
grammars](https://en.wikipedia.org/wiki/Deterministic_context-free_grammar)
(DCFL) and they are a proper subset of context-free grammars (CFG).

LR parser generators can produce a parser only for DCFL. 

```admonish note
See [this section](./parsing/parsing.md#glr-parsing) for a generalized version of LR.
```

When there is a state in which multiple actions occur we have a conflict.
Depending on the conflicting actions we may have:
- _shift-reduce_ conflicts - where the parser could either shift the token ahead
  or reduce the top of the stack by some production,
- _reduce-reduce_ conflicts - where the parser could reduce by two or more
  reductions for the given lookahead token.
  
In both cases Rustemo compiler produces a detailed information and it is a
crucial to understand what is the problem and what you should do to make the
grammar deterministic.

When a conflict arise we have a local ambiguity, i.e. Rustemo can't build a
state in which the parser will know what to do by just looking at one token
ahead. If the parser would know by looking at more tokens ahead than we have a
local ambiguity, but if there are situations in which the parser couldn't decide
with unlimited lookahead then our language is ambiguous.

Let's investigate a conflict on a famous _if-then-else_ ambiguity problem. For
example, if we have this little grammar which describes language of nested `if`
statement:

```
{{#include if_then_else.rustemo}}
```

Then running `rcomp` would reveal that we have 3 shift/reduce conflicts. Let's
investigate those conflicts one by one.

```
In State 6:Statements
	IfStatement: If Condition Then Statements .    {STOP, If, Else}
	IfStatement: If Condition Then Statements . Else Statements    {STOP, If, Else}
	Statements: Statements . Statement    {STOP, If, Else}
	Statement: . IfStatement    {STOP, If, Else}
	IfStatement: . If Condition Then Statements    {STOP, If, Else}
	IfStatement: . If Condition Then Statements Else Statements    {STOP, If, Else}
When I saw Statements and see token If ahead I can't decide should I shift or reduce by production:
IfStatement: If Condition Then Statements

```

This state is reached when the parser saw `Statements`. The state is described
by so called _LR items_. These items are productions with a dot marking the
position inside the production where the parser may be. At the same time, since
the items are LR(1) (thus one lookahead), we have possible lookahead tokens in
curly braces for that production position.

The end of the report (starting with `When I saw...`) gives the explanation. In
this state, when there is `If` token as a lookahead, the parser can't determine
whether it should shift that token or reduce previously seen `IfStatement`.

To put it differently, the question is if the next `If` statement should be
nested inside the body of the previous `If` statement, in which case we should
shift, or in the body of some outer `If` statement, in which case we should
reduce and treat the next `If` as the statement on the same nesting level.

The rest two conflicts are similar:

```
In State 6:Statements
	IfStatement: If Condition Then Statements .    {STOP, If, Else}
	IfStatement: If Condition Then Statements . Else Statements    {STOP, If, Else}
	Statements: Statements . Statement    {STOP, If, Else}
	Statement: . IfStatement    {STOP, If, Else}
	IfStatement: . If Condition Then Statements    {STOP, If, Else}
	IfStatement: . If Condition Then Statements Else Statements    {STOP, If, Else}
When I saw Statements and see token Else ahead I can't decide should I shift or reduce by production:
IfStatement: If Condition Then Statements

In State 10:Statements
	IfStatement: If Condition Then Statements Else Statements .    {STOP, If, Else}
	Statements: Statements . Statement    {STOP, If, Else}
	Statement: . IfStatement    {STOP, If, Else}
	IfStatement: . If Condition Then Statements    {STOP, If, Else}
	IfStatement: . If Condition Then Statements Else Statements    {STOP, If, Else}
When I saw Statements and see token If ahead I can't decide should I shift or reduce by production:
IfStatement: If Condition Then Statements Else Statements
```

Second is for the same state but different token ahead, `else` in this case. The
third is similar to the first.

All three conflicts are due to the parser not knowing how to nest statements.

There are two approaches to handle these conflicts:
- Use additional disambiguation specification ([priorities and associativities meta-data](../grammar_language.md#ruleproduction-meta-data))
- Change the grammar/language to eliminate ambiguities

When deciding whether to create a shift or reduce operation for a state Rustemo
will look into priorities of terminals and productions and choose the one with
higher priority. If the priorities are the same Rustemo will look at
associativities, and favor the one set on terminal if not default (`None`).

So, in our case we could specify a greedy behavior in all tree conflicts. This
behavior means that we will favor shift operation thus always choosing to nest
under the innermost statement.

```
{{#include if_then_else_shift.rustemo}}
```

Now, the grammar will compile.

Sometimes it is better and more readable to specify associativity on the
production level. See [the calculator
tutorial](../tutorials/calculator/calculator.md) for example.

The other way to solve the issue is by changing our language. The confusion is
due to not knowing when the body of the `If` statement ends. We could add curly
braces to delineate the body of `If`, or simply add keyword `end` at the end.
Let's do that:

```
{{#include if_then_else_end.rustemo}}
```

Now, our language is more flexible as the user can define the nesting by placing
`end` keyword at appropriate places.

# Syntax errors in the parsed input
These are errors which you have to handle in your code as the user supplied an
invalid input according to your grammar specification.

When you call `parse/parse_file` method with the input, you get a `Result` value
which where `Ok` variant will hold the result produced by the configured
builder, while `Err` variant will hold information about the error.

For example, this can be a result of erroneous input where the result value is
converted to a string:

```
{{#include  ../../../tests/src/from_file/parse_from_file_err.err }}
```

You can see the path of the input file, line/column location of the error, the
context where the error location is marked with `-->`, and finally the cause of
the error (`Expected Number`).

The parser is called like this:

```rust
{{#include  ../../../tests/src/from_file/mod.rs:parser-call }}
```

Error type contained in `Err` variant is defined as follows:

```rust
{{#include ../../../rustemo/src/error.rs:parser-error}}
```

As we can see, it either wraps `IOError` or, for Rustemo generated errors,
provide `message`, `file` and `location` inside the file.


# Handling ambiguities

```admonish todo
1. Lexical ambiguities - when there can be recognized multiple tokens at the
   current position.
2. Syntactic ambiguities - aplicable only to GLR - when multiple
   interpretation/trees of the input can be constructed.
   
These are not errors per se so should be moved to some other chapter.
```

