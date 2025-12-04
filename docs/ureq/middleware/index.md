*[ureq](../index.md) / [middleware](index.md)*

---

# Module `middleware`

Chained interception to modify the request or response.

## Structs

### `MiddlewareNext<'a>`

```rust
struct MiddlewareNext<'a> {
    // [REDACTED: Private Fields]
}
```

Continuation of a [`Middleware`](#middleware) chain.

#### Implementations

- `fn handle(self: Self, request: http::Request<SendBody<'_>>) -> Result<http::Response<Body>, Error>`
  Continue the middleware chain.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Middleware`

```rust
trait Middleware: Send + Sync + 'static { ... }
```

Chained processing of request (and response).

# Middleware as `fn`

The middleware trait is implemented for all functions that have the signature

`Fn(Request, MiddlewareNext) -> Result<Response, Error>`

That means the easiest way to implement middleware is by providing a `fn`, like so

```
use ureq::{Body, SendBody};
use ureq::middleware::MiddlewareNext;
use ureq::http::{Request, Response};

fn my_middleware(req: Request<SendBody>, next: MiddlewareNext)
    -> Result<Response<Body>, ureq::Error> {

    // do middleware things to request

    // continue the middleware chain
    let res = next.handle(req)?;

    // do middleware things to response

    Ok(res)
}
```

# Adding headers

A common use case is to add headers to the outgoing request. Here an example of how.

```no_run
use ureq::{Body, SendBody, Agent, config::Config};
use ureq::middleware::MiddlewareNext;
use ureq::http::{Request, Response, header::HeaderValue};

# #[cfg(feature = "json")]
# {
fn my_middleware(mut req: Request<SendBody>, next: MiddlewareNext)
    -> Result<Response<Body>, ureq::Error> {

    req.headers_mut().insert("X-My-Header", HeaderValue::from_static("value_42"));

    // set my bespoke header and continue the chain
    next.handle(req)
}

let mut config = Config::builder()
    .middleware(my_middleware)
    .build();

let agent: Agent = config.into();

let result: serde_json::Value =
    agent.get("http://httpbin.org/headers").call()?.body_mut().read_json()?;

assert_eq!(&result["headers"]["X-My-Header"], "value_42");
# } Ok::<_, ureq::Error>(())
```

# State

To maintain state between middleware invocations, we need to do something more elaborate than
the simple `fn` and implement the `Middleware` trait directly.

## Example with mutex lock

In the `examples` directory there is an additional example `count-bytes.rs` which uses
a mutex lock like shown below.

```
use std::sync::{Arc, Mutex};

use ureq::{Body, SendBody};
use ureq::middleware::{Middleware, MiddlewareNext};
use ureq::http::{Request, Response};

struct MyState {
    // whatever is needed
}

struct MyMiddleware(Arc<Mutex<MyState>>);

impl Middleware for MyMiddleware {
    fn handle(&self, request: Request<SendBody>, next: MiddlewareNext)
        -> Result<Response<Body>, ureq::Error> {

        // These extra brackets ensures we release the Mutex lock before continuing the
        // chain. There could also be scenarios where we want to maintain the lock through
        // the invocation, which would block other requests from proceeding concurrently
        // through the middleware.
        {
            let mut state = self.0.lock().unwrap();
            // do stuff with state
        }

        // continue middleware chain
        next.handle(request)
    }
}
```

## Example with atomic

This example shows how we can increase a counter for each request going
through the agent.

```
use ureq::{Body, SendBody, Agent, config::Config};
use ureq::middleware::{Middleware, MiddlewareNext};
use ureq::http::{Request, Response};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

// Middleware that stores a counter state. This example uses an AtomicU64
// since the middleware is potentially shared by multiple threads running
// requests at the same time.
struct MyCounter(Arc<AtomicU64>);

impl Middleware for MyCounter {
    fn handle(&self, req: Request<SendBody>, next: MiddlewareNext)
        -> Result<Response<Body>, ureq::Error> {

        // increase the counter for each invocation
        self.0.fetch_add(1, Ordering::Relaxed);

        // continue the middleware chain
        next.handle(req)
    }
}

let shared_counter = Arc::new(AtomicU64::new(0));

let mut config = Config::builder()
    .middleware(MyCounter(shared_counter.clone()))
    .build();

let agent: Agent = config.into();

agent.get("http://httpbin.org/get").call()?;
agent.get("http://httpbin.org/get").call()?;

// Check we did indeed increase the counter twice.
assert_eq!(shared_counter.load(Ordering::Relaxed), 2);

# Ok::<_, ureq::Error>(())
```

#### Required Methods

- `fn handle(self: &Self, request: http::Request<SendBody<'_>>, next: MiddlewareNext<'_>) -> Result<http::Response<Body>, Error>`

  Handle of the middleware logic.

