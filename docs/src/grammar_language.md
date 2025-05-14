# Grammar language
This section describe the grammar language, its syntax and semantics rules.

The Rustemo grammar specification language is based on [BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form) with [syntactic
sugar extensions](#syntactic-sugar-bnf-extensions) which are optional and build on top of a pure BNF. Rustemo
grammars are based on [Context-Free Grammars (CFGs)](https://en.wikipedia.org/wiki/Context-free_grammar) and are written
declaratively. This means you don't have to think about the parsing process like
in e.g. [PEGs](https://en.wikipedia.org/wiki/Parsing_expression_grammar). Ambiguities are dealt with explicitly (see the [section on
conflicts](#resolving-lr-conflicts)).

## The structure of the grammar
Each grammar file consists of two parts:

- derivation/production rules,
- terminal definitions which are written after the keyword `terminals`.

Each derivation/production rule is of the form:

    <symbol>: <expression> ;

where `<symbol>` is a grammar non-terminal and `<expression>` is one or more
sequences of grammar symbol references separated by the choice operator `|`.

For example:

    Fields: Field | Fields "," Field;

Here `Fields` is a non-terminal grammar symbol and it is defined as either a
single `Field` or, recursively, as `Fields` followed by a string terminal `,`
and then by another `Field`. It is not given here, but `Field` could also be
defined as a non-terminal. For example:

    Field: QuotedField | FieldContent;

Or it could be defined as a terminal in terminals section:

    terminals
    Field: /[A-Z]*/;

This terminal definition uses [regular expression recognizer](#regular-expression-recognizer).


## Terminals
Terminal symbols of the grammar define the fundamental or atomic elements of
your language, tokens or lexemes (e.g. keywords, numbers).

Terminals are specified at the end of the grammar file, after production rules,
following the keyword `terminals`.

Tokens are recognized from the input by a `lexer` component. Rustemo provides a
string lexer out-of-the-box which enables lexing based on recognizers provided in
the grammar. If more control is needed, or if non-textual context has to be
parsed, a custom lexer must be provided. See the [lexers section](./lexers.md)
for more.

Each terminal definition is in the form:

    <terminal name>: <recognizer>;

where `<recognizer>` can be omitted if a custom lexer is used.

The default string lexer enables specification of two kinds of terminal
recognizers:

- String recognizer
- Regex recognizer


### String recognizer
A string recognizer is defined as a plain string inside single or double quotes.
For example, in a grammar rule:

```
MyRule: "start" OtherRule "end";
```

`"start"` and `"end"` will be terminals with string recognizers that match
exactly the words `start` and `end`. In this example we have recognizers inlined
in the grammar rule.

For each string recognizer you must provide its definition in the `terminals`
section in order to define a terminal name.

```
terminals
Start: "start";
End: "end";
```

You can reference the terminal from the grammar rule, like:

```
MyRule: Start OtherRule End;
```

or use the same string recognizer inlined in the grammar rules, as we have
seen before. It is your choice. Sometimes it is more readable to use string
recognizers directly. But in any case, you must always declare the terminal in the
`terminals` section for the sake of providing names which are used in the code
of the generated parser.


### Regular expression recognizer
A regex recognizer, or regex for short, is a regex pattern written inside slashes
(`/.../`).

For example:

```
terminals
Number: /\d+/;
```

This rule defines terminal symbol `Number` which has a regex recognizer that
will recognize one or more digits from the input.

```admonish note
You cannot write regex recognizers inline like you can with string
recognizers. This constraint is introduced because regexes are not that easy to
write and they don't add to readability, so it is always better to reference
regex terminals by name in grammar rules.
```

```admonish warning
During regex construction, a `^` prefix is added to the regex from the grammar to
make sure that the content is matched at the current input position. This can be
an issue if you use a pattern like `A|B` in your regex as it translates to
`^A|B`, which matches either `A` at the current position or `B` in the rest of
the input. The workaround for now is to use `(A|B)`, i.e., always wrap
alternative choices in parentheses.
```

## Usual patterns
This section explains how some common grammar patterns can be written using just
a plain Rustemo BNF-like notation. Afterwards, we'll see some syntax sugar
extensions that can be used to write these patterns in a more compact and
readable form.

### One or more

This pattern is used to match one or more things.

For example, the `Sections` rule below will match one or more `Section`s.

```
Sections: Section | Sections Section;
```
Notice the recursive definition of the rule. You can read this as:

> `Sections` is either a single `Section` or `Sections` followed by a `Section`.

```admonish note
Please note that you could do the same with this rule:

    Sections: Section | Section Sections;

which will give you similar results but the resulting tree will be different.
Notice the recursive reference is now at the end of the second production.

The previous example will reduce sections early and then add another section to it,
thus the tree will be expanding to the left. The example in this note will
collect all the sections and then start reducing from the end, thus building a
tree expanding to the right. These are subtle differences that are important
when you start writing your semantic actions. Most of the time you don't care,
so use the first version as it is more efficient in the context of LR parsing.
```


### Zero or more

This pattern is used to match zero or more things.

For example, the `Sections` rule below will match zero or more `Section`s.

```
Sections: Section | Sections Section | EMPTY;
```

Notice the addition of the `EMPTY` choice at the end. This means that matching
nothing is a valid `Sections` non-terminal. Basically, this rule is the same as
one-or-more except that matching nothing is also a valid solution.

The same note from above applies here too.


### Optional

To optionally match something, use this pattern:

```
OptHeader: Header | EMPTY;
```

In this example, `OptHeader` is either a `Header` or nothing.


## Syntactic sugar - BNF extensions

The previous section covered basic BNF syntax. If you've used BNF extensions (like the [Kleene star](https://en.wikipedia.org/wiki/Kleene_star)), you might find writing patterns in basic BNF awkward. Since these patterns are common in grammars (zero-or-more, one-or-more, etc.), Rustemo provides syntactic sugar for them using regular expression syntax.


### Optional

Optional match can be specified using `?`. For example:

```
{{#include ../../tests/src/sugar/optional/optional_1.rustemo}}

```

Here, we will recognize `B` which is optionally preceded with `c` and followed
by `Num`.


Lets see what the parser will return optional inputs.

In [this test](https://github.com/igordejanovic/rustemo/blob/main/tests/src/sugar/optional/mod.rs):

```rust
{{#include ../../tests/src/sugar/optional/mod.rs:optional1}}
```

for input `c b 1` the `result` will be:

```
{{#include ../../tests/src/sugar/optional/optional_1_1.ast}}
```

If we leave the number out and try to parse `c b`, the parse will succeed and the result will be:

```
{{#include ../../tests/src/sugar/optional/optional_1_2.ast}}
```

Notice that returned type is `A` struct with fields `tc_opt` and `num_opt` of
`Optional` type. These types are auto-generated based on the grammar. To learn
more see [section on AST types/actions code generation](./builders.md#ast-typesactions-code-generation).

```admonish note

Syntax equivalence for `optional` operator

    S: B?;

    terminals
    B: "b";

is equivalent to:

    S: BOpt;
    BOpt: B | EMPTY;

    terminals
    B: "b";

Behind the scenes Rustemo will create `BOpt` rule. All syntactic sugar
additions operate by creating additional rules in the grammar during
parser compilation.
```


### One or more

One-or-more match is specified using `+` operator.

For example:

```
{{#include ../../tests/src/sugar/one_or_more/one_or_more_2.rustemo}}
```

After `c` we expect to see one or more `B` (which will match a number) and at
the end we expect `a`.

Let's see what the parser will return for input `c 1 2 3 4 a`:

```rust
{{#include ../../tests/src/sugar/one_or_more/mod.rs:one-or-more}}
```

The result will be:

```
{{#include ../../tests/src/sugar/one_or_more/one_or_more_2_2.ast}}
```

```admonish note
We see in the previous example that default AST building actions will drop
string matches as fixed content is not interesting for analysis and usually
represent syntax noise which is needed only for performing correct parsing.
Also, we see that one-or-more will be transformed to a `Vec` of matched values
(using the `vec` [annotation](#rule-annotations), see bellow). Of, course, this
is just the default. You can change it to fit your needs. To learn more see the
section on [builders](./builders.md).
```

```admonish note
Syntax equivalence for `one or more`:

    S: A+;

    terminals
    A: "a";

is equivalent to:

    S: A1;
    @vec
    A1: A1 A | A;

    terminals
    A: "a";
```


### Zero or more

Zero-or-more match is specified using `*` operator. 

For example:

```
{{#include ../../tests/src/sugar/zero_or_more/zero_or_more_2.rustemo}}
```


This syntactic sugar is similar to `+` except that it doesn't require rule to
match at least once. If there is no match, resulting sub-expression will be an
empty list. 

Let's see what the parser based on the given grammar will return for input `c 1
2 3 a`.


```rust
{{#include ../../tests/src/sugar/zero_or_more/mod.rs:zero-or-more-1}}
```

The result will be:

```
{{#include ../../tests/src/sugar/zero_or_more/zero_or_more_2_1.ast}}
```

But, contrary to one-or-more we may match zero times. For example, if input is
`c a` we get:

```
{{#include ../../tests/src/sugar/zero_or_more/zero_or_more_2_2.ast}}
```

```admonish note
Syntax equivalence for `zero or more`:

    S: A*;

    terminals
    A: "a";

is equivalent to:

    S: A0;
    @vec
    A0: A1 {nops} | EMPTY;
    @vec
    A1: A1 A | A;

    terminals
    A: "a";

So using of `*` creates both `A0` and `A1` rules. Action attached to `A0`
returns a list of matched `a` and empty list if no match is found. Please note
the [usage of `nops`](./disambiguation.md#nops-and-nopse). In case
`prefer_shift` strategy is used, using `nops` will perform both `REDUCE` and
`SHIFT` during GLR parsing if what follows zero or more might be another
element in the sequence. This is most of the time what you need.
```


### Repetition modifiers

Repetitions (`+`, `*`, `?`) may optionally be followed by a modifier in square
brackets. Currently, this modifier can only be used to define a separator. The
separator is defined as a terminal rule reference.

For example, for this grammar:

```
{{#include ../../tests/src/sugar/one_or_more/one_or_more_1_sep.rustemo}}
```

We expect to see `c`, followed by optional `B`, followed by one or more numbers
separated by a comma (`Num+[Comma]`).

If we give input `c b 1, 2, 3, 4` to the parser:

```rust
{{#include ../../tests/src/sugar/one_or_more/mod.rs:one-or-more-sep}}
```

we get this output:

```
{{#include ../../tests/src/sugar/one_or_more/one_or_more_1_1_sep.ast}}
```

```admonish note
Syntax equivalence of `one or more with separator `:

    S: A+[Comma];

    terminals
    A: "a";
    Comma: ",";

is equivalent to:

    S: A1Comma;
    @vec
    A1Comma: A1Comma Comma A | A;

    terminals
    A: "a";
    Comma: ",";

Making the name of the separator rule a suffix of the additional rule
name makes sure that only one additional rule will be added to the
grammar for all instances of `A+[Comma]`, i.e. same base rule with the
same separator.
```

### Parenthesized groups

```admonish danger
This is not yet implemented.
```

You can use parenthesized groups at any place you can use a rule reference. For
example:

    S: a (b* a {left} | b);
    terminals
    a: "a";
    b: "b";

Here, you can see that `S` will match `a` and then either `b* a` or `b`. You can
also see that [meta-data](#user-meta-data) can be applied at a per-sequence
level (in this case `{left}` applies to sequence `b* a`).

Here is a more complex example which uses repetitions, separators, assignments
and nested groups.

    S: (b c)*[comma];
    S: (b c)*[comma] a=(a+ (b | c)*)+[comma];
    terminals
    a: "a";
    b: "b";
    c: "c";
    comma: ",";

    Syntax equivalence `parenthesized groups`:

        S: c (b* c {left} | b);
        terminals
        c: "c";
        b: "b";

    is equivalent to:

        S: c S_g1;
        S_g1: b_0 c {left} | b;
        b_0: b_1 | EMPTY;
        b_1: b_1 b | b;
        terminals
        c: "c";
        b: "b";

    So using parenthesized groups creates additional `_g<n>` rules (`S_g1` in the
    example), where `n` is a unique number per rule starting from `1`. All other
    syntactic sugar elements applied to groups behave as expected.


### Greedy repetitions

```admonish danger
This is not yet implemented.
```

`*`, `+`, and `?` operators have their greedy counterparts. To make an
repetition operator greedy add `!` (e.g. `*!`, `+!`, and `?!`). These
versions will consume as much as possible before proceeding. You can
think of the greedy repetitions as a way to disambiguate a class of
ambiguities which arises due to a sequence of rules where earlier
constituent can match an input of various length leaving the rest to the
next rule to consume.

Consider this example:

    S: "a"* "a"*;

It is easy to see that this grammar is ambiguous, as for the input:

    a a

We have 3 solutions:

    1:S[0->3]
    a_0[0->1]
        a_1[0->1]
        a[0->1, "a"]
    a_0[2->3]
        a_1[2->3]
        a[2->3, "a"]
    2:S[0->3]
    a_0[0->0]
    a_0[0->3]
        a_1[0->3]
        a_1[0->1]
            a[0->1, "a"]
        a[2->3, "a"]
    3:S[0->3]
    a_0[0->3]
        a_1[0->3]
        a_1[0->1]
            a[0->1, "a"]
        a[2->3, "a"]
    a_0[3->3]

If we apply greedy zero-or-more to the first element of the sequence:

    S: "a"*! "a"*;

We have only one solution where all `a` tokens are consumed by the first
part of the rule:

    S[0->3]
    a_0[0->3]
        a_1[0->3]
        a_1[0->1]
            a[0->1, "a"]
        a[2->3, "a"]
    a_0[3->3]


## `EMPTY` built-in rule
There is a special `EMPTY` rule you can reference in your grammars. `EMPTY` rule
will reduce without consuming any input and will always succeed, i.e. it is
empty recognition.


## Named matches (*assignments*)
In the section on [builders](./builders.md) you can see that struct fields
deduced from rules, as well as generated semantic actions parameters, are named
based on the `<name>=<rule reference>` part of the grammar. We call these `named
matches` or `assignments`.

`Named matches` enable giving a name to a rule reference directly in the
grammar.

In the calculator example:

```
{{#include ../../examples/calculator/src/ast_actions/calculator04_ambig_lhs.rustemo}}
```

we can see usage of assignments to name recursive references to `E` in the first
four alternatives as `left` and `right` since we are defining binary operations,
while the fifth alternative for power operation uses more descriptive names
`base` and `exp`.

Now, with this in place, generated type for `E` and two operations (`/` and
`^`), and the semantic action for `+` operation will be:

```rust
{{#include ../../examples/calculator/src/ast_actions/calculator04_ambig_lhs_actions.rs:named-matches}}
```

```admonish note
This is just a snippet from the calculator example for the sake of brevity.
```

Notice the names of fields in `Div` and `Pow` structs. Also, the name of
parameters in `e_add` action. They are derived from the assignments.

Without the usage of assignments, the same generated types and action would be:

```rust
{{#include ../../examples/calculator/src/ast_actions/calculator03_ambig_prodkind_actions.rs:named-matches}}
```

Where these names are based on the name of the referenced rule and the position
inside the production.


## Rule/production meta-data
Rules and productions may specify additional meta-data that can be used to guide
parser construction decisions. Meta-data is specified inside curly braces right
after the name of the rule, if it is a rule-level meta-data, or after the
production body, if it is a production-level meta-data. If a meta-data is
applied to the grammar rule it is in effect for all production of the rule, but
if the same meta-data is defined for the production it takes precedence.

```admonish note
See the example bellow.
```

Currently, kinds of meta-data used during parser construction are as follows:
- disambiguation rules
- production kinds
- user meta-data

### Disambiguation rules

These are special meta-data that are used during by Rustemo during grammar
compilation to influence decision on LR automata states' actions.

```admonish note
See sections on [parsing](./parsing/parsing.md) and [resolving LR
conflicts](./handling_errors/handling_errors.md#resolving-lr-conflicts).
```

There are some difference on which rules can be specified on the production and
terminal level.

Disambiguation rules are the following:
- _priority_ - written as an integer number. Default priority is 10. Priority
  defined on productions have influence on both reductions on that production
  and shifts of tokens from that production. Priority defined on terminals
  influence the priority during tokenization. When multiple tokens can be
  recognized on the current location, those that have higher priority will be
  favored.
- _associativity_ - `right`/`left` or `shift`/`reduce`. When there is a state
  where competing shift/reduce operations could be executed this meta-data will
  be used to disambiguate. These meta-data can be specified on both productions
  and terminals level. If during grammar analysis there is a state where
  associativity is defined on both production and terminal the terminal
  associativity takes precedence.

    ```admonish note
    See the [calculator tutorial](./tutorials/calculator/calculator.md) for an
    example of priority/associativity usage. There is also an example in the
    [section on resolving LR
    conflicts](./handling_errors/handling_errors.md#resolving-lr-conflicts).
    ```

- _global shift preference control_ - `nops` and `nopse`. One of the standard
  techniques to resolve shift/reduce conflicts is to prefer shift always which
  yields a greedy behavior. This global settings can be altered during grammar
  compilation. `nops` (_no prefer shift_) can be used on a production level to
  disable this preference for the given production if enabled globally. `nopse`
  (_no prefer shift over empty_) is used to disable preferring shift over empty
  reductions only.
  
### Production kinds

These meta-data are introduced to enable better deduction of function/parameter
names in the generated code. The way to write the production kind is to write an
identifier in camel-case.

For example:

```
{{#include ./tutorials/calculator/calculator3/src/calculator.rustemo}}
```

`Add`, `Sub`, `Mul` and `Div` are production kinds. These will influence the
name of the parameters, fields etc. in the generated code.

See the [section of improving
AST](./tutorials/calculator/calculator.md#improving-ast) in the calculator
tutorial for more info.


### User meta-data
Arbitrary meta-data can be attached to rules or productions. The form of each is
`<name>: <value>` where `<name>` should be any valid Rust identifier while
`<value>` can be any of:

- integer number
- float number
- string in double or single quotes
- keywords `true` or `false` for boolean values

These meta-data are supported syntactically but are not used at the moment. In
the future semantic actions will have access to these values which could be used
do alter building process in a user defined way.

### Example
This test shows various meta-data applied at both rule and production level.

```rust
{{#include ../../rustemo-compiler/src/grammar/tests/mod.rs:meta-data-inheritance}}
```

## Rule annotations
Rule annotation are written before grammar rule name using `@action_name`
syntax. Annotations are special built-in meta-data used to change the generated
AST types and/or actions.

Currently, there is only one annotation available - `vec`, which is used to
annotate rules that represent zero-or-more or one-or-more patterns. When this
annotation is applied the resulting AST type will be `Vec`. Automatically
generated actions will take this into account if default builder is used (see
[the section on builders](./builders.md)).

`vec` annotation is implicitly used in `*` and `+` syntax sugar. See [the
relevant sections](#one-or-more-1) for the equivalent grammars using the `vec`
annotation.

For example, you can use `@vec` annotation in grammar rules that have the
following patterns:

```
// This will be a vector of Bs. The vector may be empty.
@vec
A: A B | B | EMPTY;

// This is the same but the vector must have at least one element after
// a successful parse (and here we've changed the order in the first production)
@vec
A: B A | B;
```

This is just a convenience and a way to have a default type generated up-front.
You can always change AST types manually.


## Grammar comments
In Rustemo grammar, comments are available as both line comments and block
comments:

    // This is a line comment. Everything from the '//' to the end of line is a comment.

    /*
      This is a block comment.
      Everything in between `/*`  and '*/' is a comment.
    */



## Handling keywords in your language

```admonish danger "Not implemented"
This is currently not implemented.
```
By default parser will match given string recognizer even if it is part of some
larger word, i.e. it will not require matching on the word boundary. This is not
the desired behavior for language keywords.

For example, lets examine this little grammar:

    S: "for" name=ID "=" from=INT "to" to=INT;

    terminals
    ID: /\w+/;
    INT: /\d+/;

This grammar is intended to match statement like this one:

    for a=10 to 20

But it will also match:

    fora=10 to20

which is not what we wanted.

Rustemo allows the definition of a special terminal rule `KEYWORD`. This rule
must define a [regular expression recognizer](#regular-expression-recognizer).
Any string recognizer in the grammar that can be also recognized by the
`KEYWORD` recognizer is treated as a keyword and is changed during grammar
construction to match only on word boundary.

For example:

    S: "for" name=ID "=" from=INT "to" to=INT;

    terminals
    ID: /\w+/;
    INT: /\d+/;
    KEYWORD: /\w+/;

Now,

    fora=10 to20

will not be recognized as the words `for` and `to` are recognized to be
keywords (they can be matched by the `KEYWORD` rule).

This will be parsed correctly:

    for a=10 to 20

As `=` is not matched by the `KEYWORD` rule and thus doesn't require to be
separated from the surrounding tokens.

```admonish note
    Rustemo uses integrated scanner so this example:

        for for=10 to 20

    will be correctly parsed. `for` in `for=10` will be recognized as `ID` and
    not as a keyword `for`, i.e. there is no lexical ambiguity due to tokenizer
    separation.
```

## Handling whitespaces and comments (a.k.a Layout) in your language
The default string lexer skips whitespaces. You can take control over this
process by defining a special grammar rule `Layout`. If this rule is found in
the grammar the parser will use it to parse layout before each token. This is
usually used to parse whitespaces, comments, or anything that is not relevant
for the semantics analysis of the language.

For example, given the grammar:

```
{{#include ../../tests/src/layout/ast/layout.rustemo}}
```

We can parse an input consisting of numbers and words but we will get only
numbers in the output.

```rust
{{#include ../../tests/src/layout/ast/mod.rs:input}}
```
If default AST builder is used, the result will be:

```
{{#include ../../tests/src/layout/ast/layout.ast}}
```

You can see that all layout is by default dropped from the result. Of course,
you can change that by changing the generated actions. The layout is passed to
each action through the `Context` object (`ctx.layout`).

For example, the generic tree builder preserves the layout on the tree nodes. The result from the above parse if generic tree builder is used will be:

```
{{#include ../../tests/src/layout/generic_tree/layout.ast}}
```

Here is another example that gives support for both line comments and
block comments like the one used in the grammar language itself:

```
Layout: LayoutItem*;
LayoutItem: WS | Comment;
Comment: '/*' Corncs '*/' | CommentLine;
Corncs: Cornc*;
Cornc: Comment | NotComment | WS;

terminals
WS: /\s+/;
CommentLine: /\/\/.*/;
NotComment: /((\*[^\/])|[^\s*\/]|\/[^\*])+/;
```
