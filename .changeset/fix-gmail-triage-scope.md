---
"@googleworkspace/cli": patch
---

Fix gmail +triage 403 error by using gmail.readonly scope instead of gmail.modify to avoid conflict with gmail.metadata scope that does not support the q parameter
