### Rusty Engine

github: `CleanCut/rusty_engine`

assets = sprite, music, fonts etc

```
dependencies
rusty_engine = "1.1" (latest 3.0.0, it seems)
```

various

 - can create a string on the fly with `format!`
 - from borrowed string slice to String - can call `into()` (or `to_string`)
 - can create `const` anywhere
 - a macro `matches!` returns true comparing with an enum element
