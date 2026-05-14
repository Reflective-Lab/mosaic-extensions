# Mosaic Integration Harness

Executable cross-extension checks for the extensions container.

This crate is intentionally not a workspace root and not a Converge
foundation crate. It is a product-side assembly harness that depends on
extension crates by path and verifies selected flows across repositories.

Current golden flow:

```text
Mnemos knowledge recall
  + Prism fuzzy risk signal
  -> Arbiter Cedar expense gate
```

Run it from the container root:

```sh
just integration-test
```

