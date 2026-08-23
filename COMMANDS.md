> [!note]
> Variables are wrapped with `<` and `>`, you must specify these yourself (and remove the angled brackets).

# Vultr

```json
"registryCommands": {
  "<subdomain>.vultrcr.com/<registry>": {
    "deleteImage": "VULTR_API_KEY=\"<your-api-key>\" && REGISTRY_ID=\"<registry-id>\" && IMAGE_REPOSITORY=$(curl -L --fail \"https://api.vultr.com/v2/registry/${REGISTRY_ID}/repositories\" -H \"Authorization: Bearer ${VULTR_API_KEY}\" | jq -r \".repositories | .[] | select(.name == \\\"<registry>/{repository}\\\") | .image\") && curl --fail -LX DELETE \"https://api.vultr.com/v2/registry/${REGISTRY_ID}/repository/${IMAGE_REPOSITORY}/artifact/sha256:{digest}\" -H \"Authorization: Bearer ${VULTR_API_KEY}\"",
  "listImages": "VULTR_API_KEY=\"<your-api-key>\" && REGISTRY_ID=\"<registry-id>\" && IMAGE_REPOSITORY=$(curl -L --fail \"https://api.vultr.com/v2/registry/${REGISTRY_ID}/repositories\" -H \"Authorization: Bearer ${VULTR_API_KEY}\" | jq -r \".repositories | .[] | select(.name == \\\"<registry>/{project}\\\") | .image\") && curl --fail -L \"https://api.vultr.com/v2/registry/${REGISTRY_ID}/repository/${IMAGE_REPOSITORY}/artifacts\" -H \"Authorization: Bearer ${VULTR_API_KEY}\" | jq \".artifacts | map({digest: .digest, tags: .tags | map(.name)})\""
  }
}
```
