#include "flategy.hpp"

using namespace flategy;

namespace {

    struct Impl : Game {
        void tick(const flategy_data::TickInput&);
        flatbuffers::Offset<Snapshot> view(flatbuffers::FlatBufferBuilder&) const;
    };

    void Impl::tick(const flategy_data::TickInput&) {
    }

    flatbuffers::Offset<Snapshot> view(flatbuffers::FlatBufferBuilder&) const {
        return flatbuffers::Offset<Snapshot>(0);
    }

} // namespace (anonymous)

namespace flategy {

    Game::~Game() { }

    Game* create_game(const flategy_data::CreateGameInput&) {
        return new Impl;
    }

} // namespace flategy
