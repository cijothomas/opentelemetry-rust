# Tracing Span-Start Performance: Execution Checklist

## Goal

Reduce span-start overhead by avoiding pre-sampling container allocations while keeping sampling semantics correct.

## Scope decisions (locked)

- [x] Keep `KeyValue` as-is for this phase.
- [x] Focus on container-level overhead (`SpanBuilder`, links/events path).
- [x] Use a borrow-first `SpanBuilder<'a>` model.
- [x] Remove pre-start events from builder path.
- [ ] Evaluate full `Span` events API removal in a future major phase (after deprecation/migration path is ready).
- [ ] Revisit `KeyValue` ownership model in a separate future effort.

## Proposed API shape (target)

```rust
pub struct SpanBuilder<'a> {
    pub name: Cow<'static, str>,
    pub span_kind: Option<SpanKind>,
    pub start_time: Option<SystemTime>,
    pub attributes: &'a [KeyValue],
    pub links: &'a [LinkRef<'a>],
    // events intentionally excluded from pre-start path
}

pub struct LinkRef<'a> {
    pub span_context: SpanContext,
    pub attributes: &'a [KeyValue],
}

pub trait Tracer {
    type Span: Span;
    fn build_with_context(&self, builder: SpanBuilder<'_>, parent_cx: &Context) -> Self::Span;
}
```

## Execution plan (one PR at a time)

### PR 1 — Baseline and success criteria

- [ ] Refresh span-start benchmarks (sample-on/off, builder/non-builder).
- [ ] Document baseline numbers and expected improvement areas.
- [ ] Define pass/fail criteria for regression checks.

### PR 2 — Add `LinkRef` type (additive)

- [ ] Add `LinkRef<'a>` in API crate.
- [ ] Add conversion path from `LinkRef<'a>` to owned internal link data.
- [ ] Add unit tests for conversion and limits behavior.

### PR 3 — Introduce borrow-first `SpanBuilder<'a>` (additive)

- [ ] Add lifetime-parameterized `SpanBuilder<'a>`.
- [ ] Add borrowed setters (`with_attributes(&[KeyValue])`, `with_links(&[LinkRef])`).
- [ ] Keep existing owned builder path in parallel for now.

### PR 4 — Wire SDK to defer ownership until decision

- [ ] Update SDK start path to sample on borrowed builder input.
- [ ] Clone/normalize/truncate only on `RecordOnly` / `RecordAndSample`.
- [ ] Ensure drop path avoids unnecessary owned conversion.
- [ ] Add tests for no-conversion-on-drop behavior.

### PR 5 — Remove pre-start events from builder path

- [ ] Deprecate/remove `with_events` from span creation flow.
- [ ] Keep event recording through post-start `add_event` APIs.
- [ ] Update tests/examples that use pre-start events.

### PR 6 — Move primary tracer signatures

- [ ] Switch `Tracer::build_with_context` and related paths to `SpanBuilder<'_>`.
- [ ] Update global tracer wrappers if signatures require adaptation.
- [ ] Keep temporary compatibility wrappers if needed for one cycle.

### PR 7 — Migrate in-repo callsites

- [ ] Update examples and benches to borrow-first builder APIs.
- [ ] Update SDK/internal usage to target the new path.
- [ ] Verify behavior parity.

### PR 8 — Cleanup and finalize

- [ ] Remove deprecated eager-ownership builder APIs.
- [ ] Finalize docs/migration notes/changelogs.
- [ ] Re-run benchmarks and publish before/after results.

## Definition of done

- [ ] Never-sample builder path shows measurable reduction in overhead.
- [ ] No semantic regression in sampling decisions.
- [ ] No regression in always-sample correctness.
- [ ] Public docs describe borrow-first start guidance clearly.

## Out of scope (this phase)

- Redesigning `KeyValue` / `Value` ownership model.
- Accepting non-`'static` borrowed string values directly in `KeyValue`.
- Full zero-copy attribute-value representation.
- Removing `Span::add_event` / `Span` events API in this phase.

## Future consideration (separate track)

- [ ] Define a staged deprecation/removal plan for `Span` events API (`add_event`, `add_event_with_timestamp`, `record_error` behavior), including migration guidance and compatibility strategy.

## Recommendation

Proceed with a performance-first redesign centered on **`SpanBuilder<'a>` borrow-first inputs + deferred ownership after sampling decision**.

This directly addresses #1109’s core concern and improves chances of shipping a stable tracing API with acceptable startup-path overhead.
