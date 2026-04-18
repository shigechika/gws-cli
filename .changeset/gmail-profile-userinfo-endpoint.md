---
"@googleworkspace/cli": patch
---

fix(gmail): switch display-name lookup in `gmail +send` from People API to the OIDC userinfo endpoint (`https://openidconnect.googleapis.com/v1/userinfo`). The People API path returned 403 on some personal Gmail accounts even when `userinfo.profile` was granted, which made the "grant the profile scope" tip fire for users who already had the scope, and sent messages with a null From display name (upstream #644). The userinfo endpoint accepts the same `userinfo.profile` scope and behaves uniformly across Workspace and personal accounts. The 401/403 fallback message was also reworded so it no longer misdiagnoses transient permission denials as a missing scope.
