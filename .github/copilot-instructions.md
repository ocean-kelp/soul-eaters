Project:

soul-eaters: a 3D open world action game multiplayer with level up system based up on souls you collect from enemies you kill and other activities. One shapes one's soul based on the people we stay with, the things we do, the experiences we have.

Tech stack:

- rust
- bevy game engine

Folder structure:

assets: literally assets, static stuff

src: code

- ui: code for the user interface, like menus and HUDs. Divided in features->ssubfeatures, for example start_screen is a feature.

feature/subfeature structure:

- mod.rs: main module file for the feature/subfeature, a plugin must be defined there, it uses stuff from all other files in the feature folder as "helpers" to build the plugin piece by piece.
- systems.rs: contains the systems (functions) that run every frame or on certain events.
- components.rs: contains the components (data structures) that are attached to entities.
- resources.rs: contains resources (global data structures) used by the feature/subfeature.
- layout.rs: only for ui features/subfeatures, contains code that builds the layout of the ui elements.
- states.rs: only for features/subfeatures that define a state, contains the definition of the state enum and maybe some helper functions.
- helpers.rs: any other helper functions that don't fit in the other files.
- events.rs: contains event definitions used by the feature/subfeature.

each of those files are optional, only include each of them if needed.

Register a father plugin per main module, for example the UI is a main module, so register a plugin that registers all UI features/subfeatures plugins inside it. Then register that main module plugin in the game main.rs file.