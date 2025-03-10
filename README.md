# CampHash

A proof-of-concept non-cryprographic hash function utilizing AES hardware instructions.

Goals:

- 128-bit output.
- High quality [by construction](https://blog.cessen.com/post/2024_07_10_hash_design_and_goodharts_law).
- Bulk speeds of around 50 GB/s.

**Findings:** at least with this simple (and probably robust) construction, 50 GB/s
second doesn't seem achievable on current hardware.  Nevertheless, it exceeds 40
GB/s, which is still quite good.

**To examine next:** using AVX AES instructions might make 50 GB/s or more
achievable, depending on how many AVX AES units there are per core.  I haven't
looked into this yet.  Notably, the hash construction itself wouldn't need to
change to take advantage of this, so if there is more than one AVX AES unit per
core, it would basically be free performance on hardware that supports it.
