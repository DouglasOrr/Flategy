#include <catch.hpp>
#include "../flategy.hpp"

namespace flategy {

    TEST_CASE("Can create a game", "[create_game]") {
        std::unique_ptr<Game> game(create_game(flategy_data::CreateGameInputT()));
        REQUIRE(game);
    }

} // namespace flategy
