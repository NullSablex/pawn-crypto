# AI usage policy

This policy covers AI assistance in code, documentation, examples, issues,
pull requests and reviews for pawn-crypto.

## Principles

- AI assistance is permitted. Contributors may use it to write, review,
  translate or refactor material. Contributions are evaluated on correctness,
  clarity and compliance with project requirements, rather than the tools used.
- The human contributor is responsible for the submitted work. Review and
  understand every change, check its provenance and report only validation
  actually performed. Do not submit code you cannot explain or maintain.
- Authorship and certification remain human. Do not add AI co-author trailers,
  tool signatures or generated-by attribution to commits, pull requests,
  source files or documentation. No tool-use declaration is required.
- Reviewers may use AI assistance, but a human maintainer makes the acceptance
  decision. An automated assessment does not replace required review.

## Review requirements

Before submitting AI-assisted work:

- Check APIs, native names, configuration fields and behavior against the actual
  rust-samp, SA-MP/open.mp and project sources. Do not rely on plausible names
  or unsupported claims about compatibility or security.
- Verify dependency names, official sources and licenses. Review direct and
  transitive dependencies with cargo-about when changing the dependency graph.
- Check suggested material for third-party code and preserve required notices.
  AI output is not evidence that material is original or that you have the
  rights to submit it. Identify third-party sources and restrictions.
- Use established cryptographic libraries. Validate changes to cryptographic
  behavior and the AMX boundary with appropriate tests, including known-answer
  vectors and runtime coverage where relevant.
- Do not submit secrets, real passwords, keys, tokens, production password
  hashes, private server configuration or personal data to AI services or
  include them in contributions. Use synthetic data in reproductions.

## Contribution requirements

The [contribution guide](CONTRIBUTING.md), [Apache-2.0 license](LICENSE),
[DCO](DCO), [CLA](CLA.md) and [security policy](SECURITY.md) apply equally to
AI-assisted and other contributions. AI assistance does not waive sign-offs,
CLA acceptance, rights-holder authorization, license review or required checks.

Each contributor must be able to make the DCO certification and CLA
representations truthfully. A tool cannot certify origin, accept the CLA or
grant rights on someone else's behalf. Material with unresolved provenance or
rights must be resolved before it is merged.
