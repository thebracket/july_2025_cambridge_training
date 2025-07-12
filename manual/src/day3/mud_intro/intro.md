# Workshop: Async Multi-User Dungeon

Over the last couple of days, we've gradually built up our MUD. 

On day 1, you wrote authentication (and CLI app to manage it), rooms handling - and built a simple "walk around the dungeon" client.

On day 2, you wrote a synchronous TCP server that can handle multiple clients using threads, and added the beginnings of state management. It's usable, but there's a lot wrong with it!

So today, we're going to make an *async* version of the MUD server, and a *threaded* version of the client. We'll put some effort into making the server, client and protocol cleaner and easier to extend/maintain.