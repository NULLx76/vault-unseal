# Vault Unsealer [![ci status badege](https://ci.0x76.dev/api/badges/v/vault-unseal/status.svg)](https://ci.0x76.dev/v/vault-unseal)
| :exclamation: this method of unsealing the vault is not recommended if you have high security requirements!  |
|-----------------------------------------|

This is a simple Rust program that automatically unseals a hashicorp vault instance given a list of keys.

## Environment Variables

| env var    | default | description |
| ---------- | :-------: | ----------- |
| VAULT_ADDR |    -    | address of the vault server |
| VAULT_KEY_FILE | - | a JSON file containing vault unseal key(s), see [./example_keys.json](./example_keys.json). |
| UNSEAL_INTERVAL | 15 | seconds to wait between checks / unseal attempts |
