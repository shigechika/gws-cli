---
"@googleworkspace/cli": patch
---

fix(auth): stop auto-injecting cloud-platform scope after TUI scope picker selection. This scope is restricted by Google and blocked by some Workspace admin policies, which caused `admin_policy_enforced` login failures for users who picked narrower, permitted scopes (upstream #562). Users who need cloud-platform (e.g. for the modelarmor helper) can tick it in the picker or pass `--full` / `--scopes https://www.googleapis.com/auth/cloud-platform`.
