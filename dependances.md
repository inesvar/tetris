# message d'erreur du linker pour -lxcb-shape

I think this fixed it
```
sudo apt-get install libxcb-shape0-dev libxcb-xfixes0-dev
```

# message d'erreur du linker pour -lSDL2

note: /usr/bin/ld: cannot find -lSDL2: No such file or directory

collect2: error: ld returned 1 exit status

```
sudo apt install libsdl2-dev
```