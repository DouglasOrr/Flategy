#ifndef FLATEGY_HPP
#define FLATEGY_HPP

#include <optional>
#include <vector>
#include "flategy_data_generated.h"

namespace flategy {

    extern const char* Docstring;

    // "Game" knows how to play out a Flategy game
    struct Game {
        virtual ~Game();
        virtual void tick(const flategy_data::TickInputT& input) = 0;
        virtual std::unique_ptr<flategy_data::SnapshotT> view() const = 0;
    };

    // Create a new game of Flategy
    Game* create_game(const flategy_data::CreateGameInputT& options);

} // namespace flategy

#endif // FLATEGY_HPP
