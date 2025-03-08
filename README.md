# CampHash

A proof-of-concept non-cryprographic hash function utilizing AES hardware instructions.

Goals:

- 128-bit output.
- High quality [by construction](https://blog.cessen.com/post/2024_07_10_hash_design_and_goodharts_law).
- Bulk speeds of around 50 GB/s.
