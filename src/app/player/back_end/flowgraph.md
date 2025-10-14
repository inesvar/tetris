# Flowgraph

```mermaid
graph LR
    %% objects
    Position[[Position]]
    Block[[Block]]
    Direction[Direction]
    RotationTranslation[[RotationTranslation]]
    RotationType[RotationType]
    ApplyRotationTranslation([ApplyRotationTranslation])
    Tetromino[[Tetromino]]
    UseTetromino([UseTetromino])

    %% modules
    spatial_primitives[spatial_primitives]
    rotation_translation[rotation_translation]
    moving_primitives[moving_primitives]
    tetromino[tetromino]

    subgraph spatial_primitives
        Block --o Position
        Direction
    end

    subgraph rotation_translation
        RotationTranslation --o RotationType
    end

    RotationTranslation --o Position

    subgraph moving_primitives
        ApplyRotationTranslation
    end

    ApplyRotationTranslation === spatial_primitives
    ApplyRotationTranslation -.-> RotationTranslation

    subgraph tetromino
        UseTetromino === Tetromino
    end

    Tetromino --o spatial_primitives
    Tetromino -.-> ApplyRotationTranslation
```

```mermaid
graph LR
    %% objects
    Position[[Position]]
    Block[[Block]]
    Direction[Direction]
    RotationTranslation[[RotationTranslation]]
    ApplyRotationTranslation([ApplyRotationTranslation])
    ZoomInAndOut([ZoomInAndOut])

    %% modules
    rotation_translation[rotation_translation]
    spatial_primitives[spatial_primitives]
    moving_primitives[moving_primitives]

    subgraph spatial_primitives
        Block --o Position
        Direction
    end

    subgraph rotation_translation
        RotationTranslation
    end

    subgraph moving_primitives
        ApplyRotationTranslation
        ZoomInAndOut
    end

    ZoomInAndOut === Position
    ApplyRotationTranslation === Position & Direction
    ApplyRotationTranslation -.-> RotationTranslation
    ApplyRotationTranslation -.-> ZoomInAndOut

    ZoomInAndOut ~~~ Block & Direction & RotationTranslation
    Block ~~~ Direction
```