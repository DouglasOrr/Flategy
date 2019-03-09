#ifndef FLATEGY_HPP
#define FLATEGY_HPP

#include "flategy_data_generated.h"
#include <memory>

namespace flategy {

    // "Game" knows how to play out a Flategy game
    struct Game {
        virtual ~Game();
        virtual void tick(const flategy_data::TickInput& input) = 0;
        virtual flategy_data::Snapshot view(flatbuffers::FlatBufferBuilder& fbb) const = 0;
    };

    // Create a new game of Flategy
    Game* create_game(const flategy_data::CreateGameInput& options);

} // namespace flategy

#endif // FLATEGY_HPP
