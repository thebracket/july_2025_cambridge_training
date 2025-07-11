# Serialization with Serde

`Serde` (pronounced "sir-dee") is a framework for serializing and deserializing Rust data structures efficiently and generically. It is the most popular serialization library in the Rust ecosystem. It's one of DTolnay's *many* contributions to the Rust ecosystem, and is used by many other libraries. It's relatively easy to use, quite fast, and is format agnostic.

> I once pronounced it "surd" at RustConf. I was very politely corrected by a LOT of people!

The final project for this workshop will be a TCP client-server that plays a bit like an old Multi-User Dungeon (MUD). You'll be using serialization/deserialization extensively. We'll start off with a quick "how to use Serde", and then start on a couple of parts of the project.
