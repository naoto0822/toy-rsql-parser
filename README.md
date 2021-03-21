# toy-rsql-parser

[![Rust](https://github.com/naoto0822/toy-rsql-parser/actions/workflows/rust.yml/badge.svg)](https://github.com/naoto0822/toy-rsql-parser/actions/workflows/rust.yml)

toy SQL parser written in Rust. (ongoing)

## Lexer

```rust
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

## Parser

```rust
// input query
SELECT 1+1+2, id, name FROM user WHERE id = 1;

// output AST
Select { 
  columns: [
    Column { 
      value: Infix { 
        op: Plus, 
        left: Infix { 
          op: Plus, 
          left: Number(1), 
          right: Number(1) 
        }, 
        right: Number(2)
      }, alias: ""
    }, 
    Column {
      value: Identifier("id"), 
      alias: ""
    }, 
    Column { 
      value: Identifier("name"), 
      alias: ""
    }
  ], 
  table: TableExpression { 
    from: "user", 
    where_cond: Some(
      Infix { 
        op: Eq,
        left: Identifier("id"),
        right: Number(1)
      }
    ), 
    group_by: None 
  } 
}
```

## Features

- [x] Minimum Lexer
- [x] Minimum Parser
- [ ] Lexer
- [ ] DML Parse
  - [ ] SELECT
  - [ ] INSERT
  - [ ] UPDATE
  - [ ] DELETE
- [ ] DDL Parse
- [ ] DCL Parse
