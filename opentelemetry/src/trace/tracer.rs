use crate::{
    trace::{Event, Link, Span, SpanId, SpanKind, Status, TraceContextExt, TraceId, TraceState},
    Context, KeyValue,
};
use std::borrow::Cow;
use std::time::SystemTime;

/// The interface for constructing [`Span`]s.
///
/// ## In Synchronous Code
///
/// Spans can be created and nested manually:
///
/// ```
/// use opentelemetry::{global, trace::{Span, Tracer, TraceContextExt}, Context};
///
/// let tracer = global::tracer("my-component");
///
/// let parent = tracer.start("foo");
/// let parent_cx = Context::current_with_span(parent);
/// let mut child = tracer.start_with_context("bar", &parent_cx);
///
/// // ...
///
/// child.end(); // explicitly end
/// drop(parent_cx) // or implicitly end on drop
/// ```
///
/// Spans can also use the current thread's [`Context`] to track which span is active:
///
/// ```
/// use opentelemetry::{global, trace::{SpanKind, Tracer}};
///
/// let tracer = global::tracer("my-component");
///
/// // Create simple spans with `in_span`
/// tracer.in_span("foo", |_foo_cx| {
///     // parent span is active
///     tracer.in_span("bar", |_bar_cx| {
///         // child span is now the active span and associated with the parent span
///     });
///     // child has ended, parent now the active span again
/// });
/// // parent has ended, no active spans
/// ```
///
/// Spans can also be marked as active, and the resulting guard allows for
/// greater control over when the span is no longer considered active.
///
/// ```
/// use opentelemetry::{global, trace::{Span, Tracer, mark_span_as_active}};
/// let tracer = global::tracer("my-component");
///
/// let parent_span = tracer.start("foo");
/// let parent_active = mark_span_as_active(parent_span);
///
/// {
///     let child = tracer.start("bar");
///     let _child_active = mark_span_as_active(child);
///
///     // do work in the context of the child span...
///
///     // exiting the scope drops the guard, child is no longer active
/// }
/// // Parent is active span again
///
/// // Parent can be dropped manually, or allowed to go out of scope as well.
/// drop(parent_active);
///
/// // no active span
/// ```
///
/// ## In Asynchronous Code
///
/// If you are instrumenting code that make use of [`std::future::Future`] or
/// async/await, be sure to use the [`FutureExt`] trait. This is needed because
/// the following example _will not_ work:
///
/// ```no_run
/// # use opentelemetry::{global, trace::{Tracer, mark_span_as_active}};
/// # let tracer = global::tracer("foo");
/// # let span = tracer.start("foo-span");
/// async {
///     // Does not work
///     let _g = mark_span_as_active(span);
///     // ...
/// };
/// ```
///
/// The context guard `_g` will not exit until the future generated by the
/// `async` block is complete. Since futures can be entered and exited
/// _multiple_ times without them completing, the span remains active for as
/// long as the future exists, rather than only when it is polled, leading to
/// very confusing and incorrect output.
///
/// In order to trace asynchronous code, the [`Future::with_context`] combinator
/// can be used:
///
/// ```
/// # async fn run() -> Result<(), ()> {
/// use opentelemetry::{trace::FutureExt, Context};
/// let cx = Context::current();
///
/// let my_future = async {
///     // ...
/// };
///
/// my_future
///     .with_context(cx)
///     .await;
/// # Ok(())
/// # }
/// ```
///
/// [`Future::with_context`] attaches a context to the future, ensuring that the
/// context's lifetime is as long as the future's.
///
/// [`FutureExt`]: crate::trace::FutureExt
/// [`Future::with_context`]: crate::trace::FutureExt::with_context()
/// [`Context`]: crate::Context
pub trait Tracer {
    /// The [`Span`] type used by this tracer.
    type Span: Span;

    /// Starts a new [`Span`].
    ///
    /// By default the currently active `Span` is set as the new `Span`'s parent.
    ///
    /// Each span has zero or one parent span and zero or more child spans, which
    /// represent causally related operations. A tree of related spans comprises a
    /// trace. A span is said to be a root span if it does not have a parent. Each
    /// trace includes a single root span, which is the shared ancestor of all other
    /// spans in the trace.
    fn start<T>(&self, name: T) -> Self::Span
    where
        T: Into<Cow<'static, str>>,
    {
        Context::map_current(|cx| self.start_with_context(name, cx))
    }

    /// Starts a new [`Span`] with a given context.
    ///
    /// If this context contains a span, the newly created span will be a child of
    /// that span.
    ///
    /// Each span has zero or one parent span and zero or more child spans, which
    /// represent causally related operations. A tree of related spans comprises a
    /// trace. A span is said to be a root span if it does not have a parent. Each
    /// trace includes a single root span, which is the shared ancestor of all other
    /// spans in the trace.
    fn start_with_context<T>(&self, name: T, parent_cx: &Context) -> Self::Span
    where
        T: Into<Cow<'static, str>>,
    {
        self.build_with_context(SpanBuilder::from_name(name), parent_cx)
    }

    /// Start a [`Span`] from a [`SpanBuilder`].
    fn build(&self, builder: SpanBuilder) -> Self::Span {
        Context::map_current(|cx| self.build_with_context(builder, cx))
    }

    /// Start a span from a [`SpanBuilder`] with a parent context.
    fn build_with_context(&self, builder: SpanBuilder, parent_cx: &Context) -> Self::Span;

    /// Start a new span and execute the given closure with reference to the context
    /// in which the span is active.
    ///
    /// This method starts a new span and sets it as the active span for the given
    /// function. It then executes the body. It ends the span before returning the
    /// execution result.
    ///
    /// # Examples
    ///
    /// ```
    /// use opentelemetry::{global, trace::{Span, Tracer, get_active_span}, KeyValue};
    ///
    /// fn my_function() {
    ///     // start an active span in one function
    ///     global::tracer("my-component").in_span("span-name", |_cx| {
    ///         // anything happening in functions we call can still access the active span...
    ///         my_other_function();
    ///     })
    /// }
    ///
    /// fn my_other_function() {
    ///     // call methods on the current span from
    ///     get_active_span(|span| {
    ///         span.add_event("An event!".to_string(), vec![KeyValue::new("happened", true)]);
    ///     })
    /// }
    /// ```
    fn in_span<T, F, N>(&self, name: N, f: F) -> T
    where
        F: FnOnce(Context) -> T,
        N: Into<Cow<'static, str>>,
        Self::Span: Send + Sync + 'static,
    {
        let span = self.start(name);
        let cx = Context::current_with_span(span);
        let _guard = cx.clone().attach();
        f(cx)
    }
}

/// `SpanBuilder` allows span attributes to be configured before the span
/// has started.
///
/// ```
/// use opentelemetry::{
///     global,
///     trace::{TracerProvider, SpanBuilder, SpanKind, Tracer},
/// };
///
/// let tracer = global::tracer("example-tracer");
///
/// // The builder can be used to create a span directly with the tracer
/// let _span = tracer.build(SpanBuilder {
///     name: "example-span-name".into(),
///     span_kind: Some(SpanKind::Server),
///     ..Default::default()
/// });
///
/// // Or used with builder pattern
/// let _span = SpanBuilder::from_name("example-span-name")
///     .with_kind(SpanKind::Server)
///     .start(&tracer);
/// ```
#[derive(Clone, Debug, Default)]
pub struct SpanBuilder {
    /// Trace id, useful for integrations with external tracing systems.
    pub trace_id: Option<TraceId>,

    /// Span id, useful for integrations with external tracing systems.
    pub span_id: Option<SpanId>,

    /// Span kind
    pub span_kind: Option<SpanKind>,

    /// Span name
    pub name: Cow<'static, str>,

    /// Span start time
    pub start_time: Option<SystemTime>,

    /// Span end time
    pub end_time: Option<SystemTime>,

    /// Span attributes that are provided at the span creation time.
    /// More attributes can be added afterwards.
    /// Providing duplicate keys will result in multiple attributes
    /// with the same key, as there is no de-duplication performed.
    pub attributes: Option<Vec<KeyValue>>,

    /// Span events
    pub events: Option<Vec<Event>>,

    /// Span Links
    pub links: Option<Vec<Link>>,

    /// Span status
    pub status: Status,

    /// Sampling result
    pub sampling_result: Option<SamplingResult>,
}

/// SpanBuilder methods
impl SpanBuilder {
    /// Create a new span builder from a span name
    pub fn from_name<T: Into<Cow<'static, str>>>(name: T) -> Self {
        SpanBuilder {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Specify trace id to use if no parent context exists
    pub fn with_trace_id(self, trace_id: TraceId) -> Self {
        SpanBuilder {
            trace_id: Some(trace_id),
            ..self
        }
    }

    /// Assign span id
    pub fn with_span_id(self, span_id: SpanId) -> Self {
        SpanBuilder {
            span_id: Some(span_id),
            ..self
        }
    }

    /// Assign span kind
    pub fn with_kind(self, span_kind: SpanKind) -> Self {
        SpanBuilder {
            span_kind: Some(span_kind),
            ..self
        }
    }

    /// Assign span start time
    pub fn with_start_time<T: Into<SystemTime>>(self, start_time: T) -> Self {
        SpanBuilder {
            start_time: Some(start_time.into()),
            ..self
        }
    }

    /// Assign span end time
    pub fn with_end_time<T: Into<SystemTime>>(self, end_time: T) -> Self {
        SpanBuilder {
            end_time: Some(end_time.into()),
            ..self
        }
    }

    /// Assign span attributes from an iterable.
    /// Providing duplicate keys will result in multiple attributes
    /// with the same key, as there is no de-duplication performed.    
    pub fn with_attributes<I>(self, attributes: I) -> Self
    where
        I: IntoIterator<Item = KeyValue>,
    {
        SpanBuilder {
            attributes: Some(attributes.into_iter().collect()),
            ..self
        }
    }

    /// Assign events
    pub fn with_events(self, events: Vec<Event>) -> Self {
        SpanBuilder {
            events: Some(events),
            ..self
        }
    }

    /// Assign links
    pub fn with_links(self, mut links: Vec<Link>) -> Self {
        links.retain(|l| l.span_context.is_valid());
        SpanBuilder {
            links: Some(links),
            ..self
        }
    }

    /// Assign status code
    pub fn with_status(self, status: Status) -> Self {
        SpanBuilder { status, ..self }
    }

    /// Assign sampling result
    pub fn with_sampling_result(self, sampling_result: SamplingResult) -> Self {
        SpanBuilder {
            sampling_result: Some(sampling_result),
            ..self
        }
    }

    /// Builds a span with the given tracer from this configuration.
    pub fn start<T: Tracer>(self, tracer: &T) -> T::Span {
        Context::map_current(|cx| tracer.build_with_context(self, cx))
    }

    /// Builds a span with the given tracer from this configuration and parent.
    pub fn start_with_context<T: Tracer>(self, tracer: &T, parent_cx: &Context) -> T::Span {
        tracer.build_with_context(self, parent_cx)
    }
}

/// The result of sampling logic for a given span.
#[derive(Clone, Debug, PartialEq)]
pub struct SamplingResult {
    /// The decision about whether or not to sample.
    pub decision: SamplingDecision,

    /// Extra attributes to be added to the span by the sampler
    pub attributes: Vec<KeyValue>,

    /// Trace state from parent context, may be modified by samplers.
    pub trace_state: TraceState,
}

/// Decision about whether or not to sample
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SamplingDecision {
    /// Span will not be recorded and all events and attributes will be dropped.
    Drop,

    /// Span data wil be recorded, but not exported.
    RecordOnly,

    /// Span data will be recorded and exported.
    RecordAndSample,
}
