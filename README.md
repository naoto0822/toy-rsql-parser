# toy-rsql-parser

[![Rust](https://github.com/naoto0822/toy-rsql-parser/actions/workflows/rust.yml/badge.svg)](https://github.com/naoto0822/toy-rsql-parser/actions/workflows/rust.yml)

toy SQL parser written in Rust. (ongoing)

## Lexer

```
// input query
SELECT * FROM user WHERE id = 1;

// output tokens
[
  Annot { value: Select },
  Annot { value: Ast },
  Annot { value: FromTable },
  Annot { value: Ident("user") },
  Annot { value: Where },
  Annot { value: Ident("id") },
  Annot { value: EqOp },
  Annot { value: Number(1) },
  Annot { value: SemiColon }
]
```

## Features

- [x] Minimum lexer
- [ ] Lexer
- [ ] Parser
