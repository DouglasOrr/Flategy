#include <catch.hpp>
#include "../flategy.hpp"

TEST_CASE("The docstring is nonempty", "[docstring]") {
    REQUIRE(!std::string(flategy::Docstring).empty());
}
