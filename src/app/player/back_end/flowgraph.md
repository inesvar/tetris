# Flowgraph

```mermaid
graph LR
    %% objects
    Position[[Position]]
    Direction[Direction]
    RotationTranslation[[RotationTranslation]]
    RotationType[RotationType]
    ApplyRotationTranslation([ApplyRotationTranslation])
    TetrisGrid[[TetrisGrid]]
    Tetromino[[Tetromino]]
    UseTetromino([UseTetromino])

    %% modules
    spatial_primitives[spatial_primitives]
    rotation_translation[rotation_translation]
    moving_primitives[moving_primitives]
    tetromino[tetromino]

    subgraph spatial_primitives
        Direction ~~~ Position
    end
    

    subgraph rotation_translation
        RotationTranslation --o RotationType
    end

    RotationTranslation --o Position

    subgraph moving_primitives
        ApplyRotationTranslation
    end

    ApplyRotationTranslation ==> Position & Direction
    ApplyRotationTranslation -.-> RotationTranslation

    subgraph tetris_grid
        TetrisGrid
    end

    subgraph tetromino
        UseTetromino ==> Tetromino
    end

    Tetromino --o spatial_primitives
    Tetromino -.-> ApplyRotationTranslation & TetrisGrid
```

```mermaid
graph LR
    %% objects
    Position[[Position]]
    Direction[Direction]
    RotationTranslation[[RotationTranslation]]
    ApplyRotationTranslation([ApplyRotationTranslation])
    ZoomInAndOut([ZoomInAndOut])

    %% modules
    rotation_translation[rotation_translation]
    spatial_primitives[spatial_primitives]
    moving_primitives[moving_primitives]

    subgraph spatial_primitives
        Direction
        Position
    end

    subgraph rotation_translation
        RotationTranslation
    end

    subgraph moving_primitives
        ApplyRotationTranslation
        ZoomInAndOut
    end

    ApplyRotationTranslation -.-> RotationTranslation
    ApplyRotationTranslation ==> Direction
    ApplyRotationTranslation === ZoomInAndOut
    ZoomInAndOut ==> Position

    ZoomInAndOut ~~~ Direction
```