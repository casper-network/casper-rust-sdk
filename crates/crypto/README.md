# sdk-crypto

- Exposes low level cryptographic functions utlilised by l1 platform.
- Minimal higher level types.

## Hashing

- Supported algos: blake2b & blake3

- Default algo = blake2b

- Default digest length = 32

- Functions operate over pure bytes, i.e. any domain type instance bytes encoding is done prior to hash computation

- Core functions:

  - get_hash

## Signature Schemes

- Supported algos: secp256k1 & ed25519

- Default algo = ed25519

- Functions operate over pure bytes, i.e. any domain type instance bytes encoding is done prior to signature computation

- Suggested core functions:

  - get_key_pair
  - get_signature
  - is_signature_valid

- Suggested helper factory functions:

  - get_key_pair
  - get_key_pair_from_base16
  - get_key_pair_from_base64
  - get_key_pair_from_bytes
  - get_key_pair_from_pem_file
  - get_key_pair_from_seed
  - get_signature_from_pem_file
