# 1. Record architecture decisions

Date: 2026-07-22

## Status

Accepted

## Context

We need to record the architectural decisions made on this project.
This project is transitioning from a hobbyist "single-file clone" to a professional-grade engineering portfolio piece. As the complexity scales (including hardware integration and telemetry), we must ensure that the "why" behind significant architectural choices is preserved.

## Decision

We will use Architecture Decision Records, as described by Michael Nygard in this article: http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions.

We will write them in Markdown and store them in the `docs/adr` directory.

## Consequences

See Michael Nygard's article, linked above. For a lightweight approach, this ensures context is never lost as the project evolves across versions.
