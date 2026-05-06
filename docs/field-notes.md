# Field Notes

The useful part of this repository is the small rule set around change width and review cost.

The domain cases cover `change width`, `diagnostic quality`, `review cost`, and `safe rewrite`. They sit beside the smaller starter fixture so the project has both a compact scoring check and a domain-flavored review check.

The widest spread is between `review cost` and `change width`, so those are the first two cases I would preserve during a refactor.

The language-specific addition keeps the review model strongly typed and covered by cargo tests.
