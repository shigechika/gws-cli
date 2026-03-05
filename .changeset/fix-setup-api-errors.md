---
"gws": patch
---

fix(setup): enable APIs individually and surface gcloud errors

Previously `gws auth setup` used a single batch `gcloud services enable` call
for all Workspace APIs. If any one API failed, the entire batch was marked as
failed and stderr was silently discarded. APIs are now enabled individually and
in parallel, with error messages surfaced to the user.
