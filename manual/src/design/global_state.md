# Global State

Global state is --- according to many designs --- an anti-pattern. However, it's often necessary to have *some form* of global state. A source of truth, describing the current configuration, or state of the system.

## Really Avoid Global Mutable State

Avoid sharing state with a global mutable static variable, even if its nicely Mutex wrapped. This is a great way to create hidden coupling, suffer from accidental deadlocks, and make testing difficult.

## Better: Global Mutable State with Controlled Access

Wrapping Global Mutable State in an API, and locking it away in a module (with no public access to the underlying data structure) is a better approach. This allows you to control access, and refactor the underlying implementation without breaking users of the API.

You can still incur unintended coupling, but at least you won't have deadlocks.

## Even Better: Small Pieces of Global State with Clear Ownership and Controlled Access

Instead of a single global state, consider breaking it into smaller pieces - close to the code that uses it. The module that holds it is responsible for its lifecycle, and providing a clear API to access it. This also scales out better if you need to move to a distributed system later.

## Consider Actors

As you saw in the MUD example, actors can be a great way to manage state. You can often skip the synchronization altogether, and move to *sharing by communicating*.

There's often a tradeoff between the relative simplicity of actors, and the (low) latency of message passing. If your application is latency sensitive, consider using a hybrid approach - actors for high level state, and shared state for low level data.