# stores directory

This directory contains all the stores used in the client application.
Stores are a way of storing the state of the application and client side data.

A state can be something like a JWT token, user profile, or any other data that the client side needs to store in memory.

Stores should only be `.js` files where other js and svelte files can import varables and functions to read and update defined states.

States purposely are used to be accessable by different components, pages and files without having to recalculate or regather the same data.