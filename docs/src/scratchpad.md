# Just some quick notes

This is a WIP document with quick notes that should be reworked and go elsewhere.

## `@vec` rule annotation

Rustemo performs a usual rule pattern recognition to decide what type to use but
for better control user should annotate rules where more complex types are used (like Vec).

E.g. 

```
A: As A | A | EMPTY;
```

Is a standard patter for zero or more of A. Automatically generated actions
will recognize this but you need to annotate the rule in order to use `Vec`.

```
@vec
A: As A | A | EMPTY;
```

When syntax suggar for regex-like operators is finished this will just be:

```
A*
```

