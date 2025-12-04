# Crate `ring`

# Feature Flags

<table>
<tr><th>Feature
    <th>Description
<tr><td><code>alloc (default)</code>
    <td>Enable features that require use of the heap, RSA in particular.
<tr><td><code>less-safe-getrandom-custom-or-rdrand</code>
    <td>Treat user-provided ("custom") and RDRAND-based <code>getrandom</code>
        implementations as secure random number generators (see
        <code>SecureRandom</code>). This feature only works with
        <code>os = "none"</code> targets. See
        <a href="https://docs.rs/getrandom/0.2.10/getrandom/macro.register_custom_getrandom.html">
            <code>register_custom_getrandom</code>
        </a> and <a href="https://docs.rs/getrandom/0.2.10/getrandom/#rdrand-on-x86">
            RDRAND on x86
        </a> for additional details.
<tr><td><code>less-safe-getrandom-espidf</code>
    <td>Treat getrandom as a secure random number generator (see
        <code>SecureRandom</code>) on the esp-idf target. While the esp-idf
        target does have hardware RNG, it is beyond the scope of ring to
        ensure its configuration. This feature allows ring to build
        on esp-idf despite the likelihood that RNG is not secure.
        This feature only works with <code>os = espidf</code> targets.
        See <a href="https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/random.html">
<tr><td><code>std</code>
    <td>Enable features that use libstd, in particular
        <code>std::error::Error</code> integration. Implies `alloc`.
<tr><td><code>wasm32_unknown_unknown_js</code>
    <td>When this feature is enabled, for the wasm32-unknown-unknown target,
        Web APIs will be used to implement features like `ring::rand` that
        require an operating environment of some kind. This has no effect
        for any other target. This enables the `getrandom` crate's `js`
        feature.
</table>

## Modules

- [`aead`](aead/index.md) - Authenticated Encryption with Associated Data (AEAD).
- [`agreement`](agreement/index.md) - Key Agreement: ECDH, including X25519.
- [`io`](io/index.md) - Serialization and deserialization.
- [`digest`](digest/index.md) - SHA-2 and the legacy SHA-1 digest algorithm.
- [`error`](error/index.md) - Error reporting.
- [`hkdf`](hkdf/index.md) - HMAC-based Extract-and-Expand Key Derivation Function.
- [`hmac`](hmac/index.md) - HMAC is specified in [RFC 2104].
- [`pbkdf2`](pbkdf2/index.md) - PBKDF2 derivation and verification.
- [`pkcs8`](pkcs8/index.md) - PKCS#8 is specified in [RFC 5958].
- [`rand`](rand/index.md) - Cryptographic pseudo-random number generation.
- [`rsa`](rsa/index.md) - RSA.
- [`signature`](signature/index.md) - Public key signatures: signing and verification.

## Macros

### `test_file!`

References a test input file.

