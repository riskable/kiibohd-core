# kll-core

kll-core is the KLL (Kiibohd Layout Language) funcitonal state machine implementation.
It is designed to be paired with the kll-compiler crate to process the generate state machine.

The main use-case for kll-core is embedded environments (no_std); however, it does work in standard environments.
kll-core uses externally defined datastructures to build the state-machine so the functionality can be manipulated without having to recompile kll-core.
This is especially important for embedded devices so firmware can be updated without having to change state configuration.


## Usage

TODO

