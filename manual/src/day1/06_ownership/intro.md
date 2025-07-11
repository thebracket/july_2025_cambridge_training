# Ownership

Raise your hand if you, or a friend, have ever said: "I'd love to use Rust, but the borrow checker won't let me DO anything!"

You may also have experienced the stages of Rust Grief:

1. *Denial*: "Rust solves a lot of bugs I don't write!" (which maybe true)
2. *Anger*: "The BORROW CHECKER IS IMPOSSIBLE! This works in (other language!)"
3. *Bargaining*: "I'll just use `clone()` everywhere until it compiles" (NOTE: There's nothing wrong with this while you get going!)
4. *Depression*: "I see why this is here, but it's really slowing me down."
5. *Acceptance*: "Okay, I see how this works now. Wait, now I'm basically writing Rust when I open a file in Python or C++?"

It's true, and it's a bit of a learning curve. Rust actually rewards you for keeping it simple, and won't let you get away with a lot of things that would be easy to do in other languages. Rust also gives you a *lot* of power and safety in return.