### Elite Pathfinder
Hello and welcome to my first major coding project.
I started developing this app as a cross-platform alternative to EliteObservatory, since running it on linux was rather finnicky, since it is built using Winforms.


#### Quickstart
For now, the project compiles using cargo, but uses meson to install its settings locally to the dev folder
To do this run:
```
    meson setup build
    meson compile -C build
```
And then you can
```
    cargo build
    // Or just "cargo run"
```

There is a lot of work to make this a finished application, but for now there is:
- Journal folder detection (on linux and windows)
- Journal parsing
- A non-functional UI
- Console logging of the available bio species when they are found on a planet
- Copious amounts of missing journal events, exobio logic and many missing enum entries.

Next few things I want to implement:
- [ ] Database for holding the parsed events from previous sessions
- [ ] Complete exobio parsing
- [ ] Functional UI for the exobio entries
- [ ] Functional UI for Jumping into a system & keeping track of where you are in the current one

#### Contributing
IF you dare to contribute, please don't submit anything AI generated related to the actual design of the app and such.
If you think you need to use it to do something hella repetitive, (like I did for some of the exobio entries, it's just the same thing with different values a lot of the times) that's ok

If you don't particularly know how to code, but still want to contribute when you find a missing enumeration entry, or an event from the log that hasn't been properly implemented, please post an issue with the exact error message, and more importantly, your journal entry that caused the error! Otherwise I can't really do anything with it.
