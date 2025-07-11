# Caution: Rust isn't OOP - it just looks like it, sometimes

Traits *feel* like Object-Oriented Programming (OOP) - but they aren't. Traits aren't classes (neither are structs), and there is NO inheritance. There are ways to fake it, but you shouldn't.

Favour composition over inheritance.

* Inheritance is "X is a Y" - so you can say "a Dog is an Animal".
* Composition is "X has a Y" - so you can say "a Dog has a Tail".

Developers coming from OOP languages often hit the wall of "this should be a class hierarchy" and try to force traits into that model. You *can* make it work, but it will be painful and you'll end up with a lot of boilerplate code.