#include <iostream>

#include "data_structure/fixed_list.h"
#include "constants/CardPools.h"
#include "game/Game.h"
#include "game/Map.h"
#include "game/Neow.h"
#include "combat/BattleContext.h"
#include "sim/ConsoleSimulator.h"
#include "sim/PrintHelpers.h"

#pragma clang diagnostic push
#pragma ide diagnostic ignored "EndlessLoop"
using namespace sts;

int main() {
    for (int seed = 1; seed <= 10000; seed++) {
        GameContext gameContext(CharacterClass::IRONCLAD, seed, 0);
        std::cout << gameContext << std::endl;
        gameContext.transitionToAct(2);
        std::cout << gameContext << std::endl;
        gameContext.transitionToAct(3);
        std::cout << gameContext << std::endl;

    }
}

#pragma clang diagnostic pop


