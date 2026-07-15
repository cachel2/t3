# Commit conventions — t3c

Format:

```
area: imperative summary
```

Lowercase after the colon. ("add", not "added"). Keep the
subject under ~50 characters. If it needs more, the *body* explains **why**,
not what, the diff already shows what.

## Areas

The area is the subsystem the change lives in — usually one of your crates.

| Area       | Covers                                             |
|------------|----------------------------------------------------|
| `lexer:`   | tokens, the scanner                                |
| `parser:`  | recursive-descent + Pratt, the grammar             |
| `hir:`     | name resolution, desugaring (`defer`, `?`, `for`)  |
| `types:`   | type checking, inference, exhaustiveness           |
| `mir:`     | lowering, the CFG, explicit checks/traps/drops     |
| `interp:`  | the reference interpreter / sanitizer / const-eval |
| `llvm:`    | codegen via inkwell, LLVM IR emission              |
| `diag:`    | diagnostics, spans, fix-its, `t3 explain`          |
| `driver:`  | the CLI, `t3 build/run/check/test/fmt`             |
| `build:`   | Cargo, toolchain, dependencies                     |
| `test:`    | the corpus, golden diagnostic tests                |
| `docs:`    | README, spec, this file                            |
| `repo:`    | scaffolding, `.gitignore`, housekeeping            |

If a change genuinely spans two areas, it's usually two commits. If it truly
can't be split, pick the area that owns the *intent* of the change.

With a body (the blank line between subject and body is required):

```
mir: lower `?` before defer registration

`?` is a return, so any defers registered above it must run on the
early-exit path. Lowering `?` first keeps defer's LIFO order correct
without a special case in the backend.
```

 Keep commits **small and atomic** — one logical change each.

 Keep the build green on every commit.
