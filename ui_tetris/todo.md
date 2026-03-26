- Keybindings should be HashMap<Key, TetrisCommand>, since that's the most frequent use
=> there should be a method to get a Hashmap<TetrisCommand, Vec<Key>> since that's needed for the input manager
=> the method set_keys is a little complicated ://// think about this

=> depending on the situation, use the hashmap or its inverse