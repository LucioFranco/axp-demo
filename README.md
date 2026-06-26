# axp-demo

A tiny end-to-end demo of the AXP **`/axp run` PR trigger**.

It contains three pieces that together exercise the whole loop:

1. **`demo-cli`** — a small, zero-dependency Rust CLI (the *product under test*).
   `demo-cli hash <text>` prints a deterministic FNV-1a-64 digest, e.g.

   ```console
   $ demo-cli hash axp-pr-trigger
   aadea662ab8b6ad0
   ```

2. **`.axp/experiment.yaml`** — an AXP experiment that stages the freshly built
   `demo-cli` into a sandbox (via a *source-less* `files:` slot named `demo-cli`),
   asks an agent to use it to hash a fixed string, and verifies the answer
   matches the binary's real output.

3. **`.github/workflows/axp.yml`** — on every PR, builds `demo-cli` and ships it
   to AXP with `axp ci upload demo-cli ./target/release/demo-cli`, keyed to the
   PR head commit.

## The loop

```
open / push to a PR
        │
        ▼
GitHub Actions builds demo-cli → axp ci upload (keyed to head commit)
        │
        ▼
comment  /axp run  on the PR
        │
        ▼
AXP reads .axp/experiment.yaml at the PR head, rendezvouses on the uploaded
artifact, runs the experiment, and posts a result comment + a non-blocking
"axp run" check
```

## Try it

1. Open a PR against this repo (or push a commit to an existing PR). The **axp**
   workflow builds and uploads `demo-cli`.
2. Comment **`/axp run`** on the PR (on its own line).
3. AXP replies with a live status comment, then an updated result comment with
   per-variant pass/fail, plus a non-blocking **axp run** check.

## Local development

```console
# build + smoke-test the CLI
cargo run --release -- hash axp-pr-trigger

# run the experiment locally, binding the source-less slot to your local build
cargo build --release
axp local run .axp/experiment.yaml \
  --file demo-cli=./target/release/demo-cli \
  --managed-model-access
```
