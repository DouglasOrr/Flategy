#ifndef FLATEGY_HPP
#define FLATEGY_HPP

#include <optional>
#include <vector>

namespace flategy {

    extern const char* Docstring;

    // ---------- Data ----------

    typedef unsigned unit_id;
    typedef unsigned owner_id;

    struct Position {
        float x;
        float y;
    };

    enum CommandType {
        Stop,
        Move,
        Attack,
        Spawn,
        IdleDoNothing,
        IdleDontMove,
        IdleAutoAttack
    };

    enum UnitType {
        Block,
        Territory,
        HQ,
        Turret,
        Melee,
        Ranged,
        Fast
    };

    struct Command {
        CommandType type;
        union {
            unit_id attack_target;
            Position move_destination;
            struct {
                UnitType type;
                Position position;
            } spawn;
        };
    };

    struct Unit {
        // Logical
        unit_id id;
        UnitType type;
        owner_id owner;
        Command primary_command;
        Command secondary_command;
        // Physical
        Position position;
        float radius;
        float orientation;
        float health;
    };

    struct Snapshot {
        std::vector<Unit> units;
    };

    struct TickData {
        std::vector<std::tuple<unit_id, Command>> commands;
    };

    struct GameData {
        unsigned num_players;
        unsigned map_seed;
    };

    // ---------- Behaviour ----------

    // "Game" knows how to play out a Flategy game
    class Game {
        virtual ~Game();
        virtual void tick(const TickData& options) = 0;
        virtual Snapshot view() const = 0;
    };

    // Create a new game of Flategy
    Game* create_game(const GameData& options);

} // namespace flategy

#endif // FLATEGY_HPP
