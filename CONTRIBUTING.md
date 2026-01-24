# Contributing to Hexyg

## Workflow

- **Main branch**: `main`. This is the stable branch, don't commit directly to it.
- **Feature branches**: create a branch from `main`, make changes, open a Pull Request.
- **Naming**: name branches meaningfully: `feature/parser-optimization`, `fix/address-validation`, `docs/grammar-spec`.

## Commits

- Write meaningful commit messages. "fix bug" is not meaningful.
- Format: `type: brief description`. Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`.
- Before committing, update `specs/00.03_changelog.md` with a description of changes.

## Pull Requests

- One PR — one logical unit of work. Don't mix fixes, features, and refactoring.
- PR description should explain **what** and **why**, not just "fixes #123".
- Make sure code compiles and tests pass.
- If adding functionality — add tests.

## Issues

Issues are **not a support line**. Use for:

- **Bug reports**: error description, reproduction steps, expected and actual behavior, version/environment.
- **Feature proposals**: concrete proposal with justification, not "it would be cool if".

Don't use for:
- Questions "how does this work" — read documentation in `/specs` and `/docs`.
- Requests for help with your code.
- Discussions "what if".

Poorly formatted issues will be closed without discussion.

## Code Style

- **Rust**: follow standard `rustfmt` and `clippy`. Run before committing.
- **TypeScript/JavaScript**: use project ESLint configuration.
- **Kotlin**: follow official style guide.
- For other languages — use common practices.

## Testing

- New code must be covered by tests.
- Use shared test corpus from `/tests/corpus` for cross-language validation.
- Tests must be deterministic and isolated.

## Grammar Development

The reference grammar is located in `grammar/hexyg.g4`. To verify grammar correctness:

1. You must have **Java (JRE)** installed and available in your `PATH`.
2. Run `cargo test -p hexyg-grammar-check`.
3. This will automatically download ANTLR, generate the parser, and validate corpus files.

The grammar file is used as documentation for LLM-assisted development and for validation, but **not** for generating the production Rust parser (which may use pest or a hand-written parser for better performance).

## Documentation

- Code is written in English, comments too.
- Documentation in `/specs` and `/docs` — in Russian and English.
- Update documentation when changing API or behavior.

## Architecture

Before implementing major changes:

1. Study `/specs/00_master_plan.md` and `/specs/03_architecture.md`.
2. Ensure the change aligns with the architecture.
3. If deviation is needed — discuss in an issue or PR.

## Review Process

- PR must get approval from a maintainer before merge.
- Review comments must be addressed, not ignored.
- If you disagree with a comment — discuss, don't argue.

## License

By submitting a PR, you agree that your code will be licensed under Apache 2.0.
