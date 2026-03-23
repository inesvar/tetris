- keyboard has 2 separate functionalities :
    - mapping multiple keys to a unique boolean "key" ("go right" can be pressed or released, but actually there are multiple "go right" keys)
    - mapping this hardware "key" to a logical boolean ("go right" and "go left" can be pressed at the same time, only one of them will be considered)

    While the 1st was can be handled by PressedKeys, PressedKeys didn't previously have the responsibility of handling keybindings (which HW key
    maps to which SW event).