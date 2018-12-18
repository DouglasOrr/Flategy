#include "flategy.hpp"

using namespace flategy;

namespace {

    struct Impl : Game {
        void tick(const flategy_data::TickInputT& options);
        std::unique_ptr<flategy_data::SnapshotT> view() const;
    };

} // namespace (anonymous)

namespace flategy {

    const char* Docstring = "Core native logic for the RTS game Flategy";

    Game::~Game() { }

    Game* create_game(const flategy_data::CreateGameInputT&) {
        return 0; // TODO
    }

} // namespace flategy
