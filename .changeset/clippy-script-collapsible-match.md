---
"@googleworkspace/cli": patch
---

chore(clippy): collapse nested `if` inside the `"json"` arm of the Apps Script file classifier into a match guard. No behavior change — only satisfies `clippy::collapsible_match` which is now enforced in CI.
