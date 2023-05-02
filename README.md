# Vault Unsealer
This is a simple Rust program that automatically unseals a hashicorp vault instance.

| :exclamation: this method of unsealing the vault is not recommended if you have high security requirements!  |
|-----------------------------------------|


## Environment Variables

| env var    | default | description |
| ---------- | ------- | ----------- |
| VAULT_ADDR |    -    | address of the vault server |
| VAULT_KEY_FILE | - | json file containing vault unseal key(s), see [./example_keys.json](./example_keys.json) |
| UNSEAL_INTERVAL | 15 | seconds to wait between checks / unseal attempts |
