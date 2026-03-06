---
"@googleworkspace/cli": patch
---

Replace strip_suffix(".readonly").unwrap() with unwrap_or fallback

Two call sites used `.strip_suffix(".readonly").unwrap()` which would
panic if a scope URL marked as `is_readonly` didn't actually end with
".readonly". While the current data makes this unlikely, using
`unwrap_or` is a defensive improvement that prevents potential panics
from inconsistent discovery data.
