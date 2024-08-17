# Spotibye

##### Binary Crate

In the binary crate (src/main.rs) there will exist a CLI program that allows a user to interact with the spotbiye API. There will be a few commands and an auth flow that allows a user to firstly, login, and then interact programmatically with their spotify account.

This would include functionality like:

Removing an entire artists discography from their liked songs.
Adding an entire artists discography from their liked songs.

Later on...
Inputting details on where they are about to listen i.e.
Driving, with partner, feeling happy, wanting to sing, in the shower, friends over for dinner, wanting chill music, wnating to listen to new music etc.

Then, through some clever modelling, songs will be recommended with the odd random song (maybe that is at varying degrees of fit with what the model understands for the context), and the user will have the option to like or dislike a song for the context.

##### Library Crate

In the library crate (src/lib.rs), there will be the actual spotibye API that delivers the functionality described above.

The further library crates/modules (src/in_the_moment/mod.rs) will modularize the functinoality used in the main spotibye library. Certain modules like the auth flow will be made private since it isn't part of spotibye, just something spotibye needs to acheive its goals.
