---
"@googleworkspace/cli": patch
---

Fix `+append --json-values` flattening multi-row arrays into a single row by preserving the `Vec<Vec<String>>` row structure through to the API request body
