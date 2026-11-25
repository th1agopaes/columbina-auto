# Architecture decision records

An ADR is a short file that records one important decision: the context, what was decided and what it costs. Decisions are never edited away: a new ADR supersedes an old one, and both stay here.

| # | Decision | Status |
| --- | --- | --- |
| [0001](0001-use-winit-and-softbuffer.md) | Use winit and softbuffer for the window and software rendering | Accepted |

## Writing a new one

1. Copy the template below into `NNNN-short-title.md`, with the next number.
2. Open it in the same PR as the change it explains.
3. Add a row to the table above.

```markdown
# N. Title as a decision ("Use X for Y")

- **Status:** Proposed | Accepted | Superseded by [NNNN](NNNN-title.md)
- **Date:** YYYY-MM-DD

## Context

What problem we have, and what limits the answer.

## Decision

What we will do.

## Alternatives considered

| Option | Why not |
| --- | --- |

## Consequences

What gets easier, what gets harder, and the risks.
```

The format follows Michael Nygard's [Documenting Architecture Decisions](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions).
