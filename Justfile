# Converge extensions workspace commands
# Install: brew install just  |  cargo install just
# Usage:   just --list

set dotenv-load := true

repos := "arbiter-policy atelier-showcase embassy-ports ferrox-solvers manifold-adapters mnemos-knowledge prism-analytics soter-smt"

# Show available recipes
default:
    @just --list

# List extension directories
repos:
    @printf "%s\n" {{repos}}

# Show git status for every local checkout
status:
    #!/usr/bin/env bash
    set -euo pipefail
    for repo in {{repos}}; do
      echo "== ${repo} =="
      if [ -d "${repo}/.git" ]; then
        git -C "${repo}" status --short --branch
      else
        echo "no local .git"
      fi
      echo
    done

# Build every extension, or pass a repo name
build repo="all":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{repo}}" != "all" ]; then
      cd "{{repo}}" && just build
      exit 0
    fi
    for repo in {{repos}}; do
      echo "== build ${repo} =="
      (cd "${repo}" && just build)
    done

# Check every extension, or pass a repo name
check repo="all":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{repo}}" != "all" ]; then
      cd "{{repo}}" && just check
      exit 0
    fi
    for repo in {{repos}}; do
      echo "== check ${repo} =="
      (cd "${repo}" && just check)
    done

# Test every extension, or pass a repo name
test repo="all":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{repo}}" != "all" ]; then
      cd "{{repo}}" && just test
      exit 0
    fi
    for repo in {{repos}}; do
      echo "== test ${repo} =="
      (cd "${repo}" && just test)
    done

# Check formatting in every extension, or pass a repo name
fmt-check repo="all":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{repo}}" != "all" ]; then
      cd "{{repo}}" && just fmt-check
      exit 0
    fi
    for repo in {{repos}}; do
      echo "== fmt-check ${repo} =="
      (cd "${repo}" && just fmt-check)
    done

# Format every extension, or pass a repo name
fmt repo="all":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{repo}}" != "all" ]; then
      cd "{{repo}}" && just fmt
      exit 0
    fi
    for repo in {{repos}}; do
      echo "== fmt ${repo} =="
      (cd "${repo}" && just fmt)
    done

# Run clippy in every extension, or pass a repo name
clippy repo="all":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{repo}}" != "all" ]; then
      cd "{{repo}}" && just clippy
      exit 0
    fi
    for repo in {{repos}}; do
      echo "== clippy ${repo} =="
      (cd "${repo}" && just clippy)
    done

# Run each extension's lint gate
lint repo="all":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{repo}}" != "all" ]; then
      cd "{{repo}}" && just lint
      exit 0
    fi
    for repo in {{repos}}; do
      echo "== lint ${repo} =="
      (cd "${repo}" && just lint)
    done

# Run cross-extension integration harnesses
integration-test:
    cd integration-harness && cargo test --all-targets

# Generate docs for every extension, or pass a repo name
doc repo="all":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{repo}}" != "all" ]; then
      cd "{{repo}}" && just doc
      exit 0
    fi
    for repo in {{repos}}; do
      echo "== doc ${repo} =="
      (cd "${repo}" && just doc)
    done
