# High-Level Game Logic Diagram

This diagram provides an overview of the core flow and interactions within the Rust deck-building game.

```mermaid
graph TD
    A[Start Simulator] --> B[Initialize Components]
    B --> C[Run Neow Phase]
    C --> D[Generate Map]
    D --> E[Begin Game Loop]
    E -->|Advance Map| F[Room Simulation]
    F -->|Encounter Room| G[Combat Simulation]
    F -->|Event Room| H[Event Simulation]
    F -->|Rest Room| I[Rest or Upgrade]
    F -->|Treasure Room| J[Treasure Rewards]
    G -->|Player Defeated| M[Game Over]
    G -->|Defeat All Enemies| K[Choose Rewards]
    K --> L[Continue Loop]
    H --> L[Continue Loop]
    I --> L[Continue Loop]
    J --> L[Continue Loop]
    L -->|Proceed to Next Floor| E
    M -->|End Simulation| N[End of Game]
```

## Sequence Diagram: Client Code Interaction

```mermaid
sequenceDiagram
    participant Client
    participant StsSimulator
    participant Player
    participant CombatSimulator

    Client->>StsSimulator: Send game start signal
    StsSimulator->>Player: Initialize player state
    StsSimulator->>Client: Send map data
    loop Game Loop
        Client->>StsSimulator: Send player action
        StsSimulator->>Player: Process player action
        alt Room is Combat
            StsSimulator->>CombatSimulator: Start combat encounter
            CombatSimulator->>Player: Resolve player actions
            Player<<-->>Client: Resolve player actions
            Player->>CombatSimulator: Resolve player actions
            CombatSimulator->>StsSimulator: Return combat results
        end
        StsSimulator->>Client: Send updated game state
    end
    StsSimulator->>Client: Send game over message
```

## Key Components:

1. **Start Simulator**: Initialize the simulator with seed, character, and communication channels.
2. **Initialize Components**: Set up RNGs, player state, and other required systems.
3. **Run Neow Phase**: Handle the initial Neow blessing and penalties.
4. **Generate Map**: Create a randomized map for navigation.
5. **Game Loop**: Iterate through floors and rooms until the game ends.
6. **Room Simulation**: Simulate various room types:
   - **Combat Simulation**: Run battles and resolve rewards.
   - **Event Simulation**: Handle special events.
   - **Rest Room**: Allow player to heal or upgrade.
   - **Treasure Room**: Grant treasure rewards.
7. **Choose Rewards**: Let the player choose rewards after combat.
8. **Game Over**: Handle the end of the game based on player health.

This structure ensures modularity and clarity, making the game logic extensible for additional features like new room types or mechanics.

---

## Implementation

The following sections provide the Rust implementation details for each component.