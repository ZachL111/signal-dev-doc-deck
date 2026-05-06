# Review Journal

The repository goal stays the same: build a Rust toolkit that studies doc behavior through capacity fixtures, with allocation and spill reports and no network dependency. This note explains the added review angle.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 141, lane `ship`
- `stress`: `diagnostic quality`, score 146, lane `ship`
- `edge`: `review cost`, score 235, lane `ship`
- `recovery`: `safe rewrite`, score 231, lane `ship`
- `stale`: `change width`, score 115, lane `watch`

## Note

The repository should be understandable without pretending it is larger than it is.
