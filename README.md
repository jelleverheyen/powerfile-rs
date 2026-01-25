# PowerFile
PowerFile is a small experimental CLI for declarative filesystem scaffolding.

It expands compact patterns into directory structures and files, making it easier to create repetitive or structured layouts without script or generators.

## Example
```
powerfile create "src/features/(customers,billing)/README.md"
```
Creates:
```
src/
└── features/
    ├── customers/
    │   └── README.md
    └── billing/
        └── README.md
```
---
## Patterns
### Groups
Parentheses introduce alternatives:
```
Features/(Orders,Users,Chat)/
```
Expands to:
```
Features/Orders/
Features/Users/
Features/Chat/
```

### Nesting
Groups can be nested to express hierarchy:
```
Features/(Users,Chat)/(Commands,Queries)/
```
Expands to:
```
Features/
├── Users/
│   ├── Commands/
│   └── Queries/
└── Chat/
    ├── Commands/
    └── Queries/
```

### Ranges
Numeric and character ranges are supported:
```
tests/test[1..5]
```
Expands to:
```
Tests/Test_1
Tests/Test_2
Tests/Test_3
Tests/Test_4
Tests/Test_5
```
---

## Files
Patterns apply equally to files and directories:
```
src/(api,core,infra)/mod.rs
```

Creates `mod.rs` in each directory.
File extensions are treated as literal text.

---
## Preview
You can inspect the resulting AST without creating files:
```
powerfile preview "src/features/(customers,billing)/README.md"
```
Result:
```
TextGroup
└── ExpandableGroup
    ├── Text("src/features/")
    ├── TextGroup
    │   ├── ExpandableGroup
    │   │   └── Text("customers")
    │   ├── ExpandableGroup
    │   │   └── Text("billing")
    │   └── ExpandableGroup
    │       └── Text("vibes")
    ├── Text("/README")
    ├── Text(".")
    └── Text("md")

```

## Templates (WIP)
PowerFile supports simple file templates.

Templates are matched deterministically using:
- filename prefixes
- filename suffixes
- optional tags

If no template matches, an empty file is created.

Template handling is intentionally conservative, there is no implicit logic or code execution.

