#include "flategy.hpp"

using namespace flategy;

namespace {

    struct Impl : Game {
        void tick(const TickData& options);
        Snapshot view() const;
    };

} // namespace (anonymous)

namespace flategy {

    Game::~Game() { }

    Game* create_game(const GameData&) {
        return 0; // TODO
    }

} // namespace flategy
