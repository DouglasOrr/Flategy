# RTS Bot

Goal: a playable RTS game with an interesting bot behaviour.

## Getting started

Install Docker & run the following:

    ./run -r
    ./scripts/notebook

## The game

Flategy is played on a 2D world, the objective is to destroy the opponent's base.

**World**

 - 2D, integer coordinates
 - finite, circular field of view from each unit

**Map**

 - rectangular, arbitrary width/height
 - contains walls - rectangular movement/attack blockers
 - contains objectives - zones that can be captured to provide resource income
 - contains N circular "bases"

**Base**

 - continually spawns units around the base, based on resource income
 - player control: set spawning unit type
 - finite but slowly regenerating health

**Unit**

 - can move around the map, attack enemies, capture territory
 - player control: set command = STOP|MOVE(x,y)|ATTACK(unit)
 - types: agile|ranged|strong
 - finite health, regenerates near to base
