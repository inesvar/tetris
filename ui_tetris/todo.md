- Depending on the situation, use the hashmap or its inverse

1. created KeyLookup : OK
2. using KeyLookup in KeyboardInput : OK


Keybindings probably shouldn't even be stored in KeyboardInput/Player

- There should be a Keybindings per player _only when in the settings_

- When leaving the settings, update_key_look_up should be called, passing the Keybindings as arguments