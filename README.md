# signal-dev-doc-deck

`signal-dev-doc-deck` is a compact Rust repository for developer tools, centered on this goal: Build a Rust toolkit that studies doc behavior through capacity fixtures, with allocation and spill reports and no network dependency.

## Project Rationale

This is intentionally local and self-contained so it can be inspected without credentials, services, or seeded history.

## Signal Dev Doc Deck Review Notes

For a quick review, compare `review cost` with `change width` before reading the middle cases.

## Feature Set

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/signal-dev-doc-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `review cost` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Architecture

The implementation keeps the scoring rule plain: reward signal and confidence, preserve slack, penalize drag, then classify the result into a review lane.

The Rust addition stays small enough to inspect in one sitting.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Test Command

That command is also the regression path. It verifies the domain cases and catches mismatches between the CSV, metadata, and code.

## Next Improvements

The fixture set is small enough to audit by hand. The next useful expansion is malformed input coverage, not extra surface area.
