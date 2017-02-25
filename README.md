# Tetanus

This is a minimal, extensible markup language, loosely inspired by TeX and BBCode. It looks like this:

```tetanus
\q{Let's wrap a \q{sentence} in some \q{quotes}.}
```

Currently there's no built-in evaluation model, so behavior for all tags is completely user-defined. All this gives you is a parse tree.

The original implementation was hacked together in Perl in 2012. It had built-in definitions for certain tags, and additional syntax with somewhat magic behavior. The Rust version is easier to use by a wide margin.

This library was built for internal use, and is provided with no guarantee that it will be of any use to anybody.
