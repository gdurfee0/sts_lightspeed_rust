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
    /*
    for (int seed = 1; seed <= 10000; seed++) {
        GameContext gameContext(CharacterClass::IRONCLAD, seed, 0);
        std::cout << gameContext << std::endl;
        gameContext.transitionToAct(2);
        std::cout << gameContext << std::endl;
        gameContext.transitionToAct(3);
        std::cout << gameContext << std::endl;

    }
    */
    uint64_t seed = 3;
    GameContext gameContext(CharacterClass::IRONCLAD, seed, 0);
    std::cout << "[" << std::endl;
    std::cout << "    // Act 1" << std::endl;
    for (int i = 3; i < 15; i++) {
        gameContext.floorNum = i;
        gameContext.curMapNodeY = i;
        //std::cout << "floor: " << i << " m " << gameContext.monsterChance << " s " << gameContext.shopChance << " t " << gameContext.treasureChance << std::endl;
        Room room = gameContext.getEventRoomOutcomeHelper(false);
        std::cout << "    (" << std::endl;
        switch (room) {
            case Room::MONSTER:
                std::cout << "        Room::Monster," << std::endl;
                std::cout << "        None" << std::endl;
                break;
            case Room::EVENT:
                std::cout << "        Room::Event," << std::endl;
                {
                    Event event = gameContext.generateEvent(gameContext.eventRng);
                    std::cout << "        Some(Event::" << eventRustEnums[(int)event] << ")" << std::endl;
                }
                break;
            case Room::SHOP:
                std::cout << "        Room::Shop," << std::endl;
                std::cout << "        None" << std::endl;
                break;
            case Room::TREASURE:
                std::cout << "        Room::Treasure," << std::endl;
                std::cout << "        None" << std::endl;
                break;
            default:
                std::cout << "        Room::INVALID," << std::endl;
                std::cout << "        None" << std::endl;
                break;
        }
        std::cout << "    )," << std::endl;
    }
    //gameContext.obtainRelic(RelicId::BLACK_STAR);
    gameContext.transitionToAct(2);
    std::cout << "    // Act 2" << std::endl;
    for (int i = 20; i < 32; i++) {
        gameContext.floorNum = i;
        gameContext.curMapNodeY = i - 17;
        //std::cout << "floor: " << i << " m " << gameContext.monsterChance << " s " << gameContext.shopChance << " t " << gameContext.treasureChance << std::endl;
        Room room = gameContext.getEventRoomOutcomeHelper(false);
        std::cout << "    (" << std::endl;
        switch (room) {
            case Room::MONSTER:
                std::cout << "        Room::Monster," << std::endl;
                std::cout << "        None" << std::endl;
                break;
            case Room::EVENT:
                std::cout << "        Room::Event," << std::endl;
                {
                    Event event = gameContext.generateEvent(gameContext.eventRng);
                    std::cout << "        Some(Event::" << eventRustEnums[(int)event] << ")" << std::endl;
                }
                break;
            case Room::SHOP:
                std::cout << "        Room::Shop," << std::endl;
                std::cout << "        None" << std::endl;
                break;
            case Room::TREASURE:
                std::cout << "        Room::Treasure," << std::endl;
                std::cout << "        None" << std::endl;
                break;
            default:
                std::cout << "        Room::INVALID," << std::endl;
                std::cout << "        None" << std::endl;
                break;
        }
        std::cout << "    )," << std::endl;
    }
    //gameContext.obtainRelic(RelicId::ASTROLABE);
    gameContext.transitionToAct(3);
    gameContext.speedrunPace = true;
    std::cout << "    // Act 3" << std::endl;
    for (int i = 35; i < 47; i++) {
        gameContext.floorNum = i;
        gameContext.curMapNodeY = i - 32;
        //std::cout << "floor: " << i << " m " << gameContext.monsterChance << " s " << gameContext.shopChance << " t " << gameContext.treasureChance << std::endl;
        Room room = gameContext.getEventRoomOutcomeHelper(false);
        std::cout << "    (" << std::endl;
        switch (room) {
            case Room::MONSTER:
                std::cout << "        Room::Monster," << std::endl;
                std::cout << "        None" << std::endl;
                break;
            case Room::EVENT:
                std::cout << "        Room::Event," << std::endl;
                {
                    Event event = gameContext.generateEvent(gameContext.eventRng);
                    std::cout << "        Some(Event::" << eventRustEnums[(int)event] << ")" << std::endl;
                }
                break;
            case Room::SHOP:
                std::cout << "        Room::Shop," << std::endl;
                std::cout << "        None" << std::endl;
                break;
            case Room::TREASURE:
                std::cout << "        Room::Treasure," << std::endl;
                std::cout << "        None" << std::endl;
                break;
            default:
                std::cout << "        Room::INVALID," << std::endl;
                std::cout << "        None" << std::endl;
                break;
        }
        std::cout << "    )," << std::endl;
    }
    std::cout << "]" << std::endl;
    exit(0);

}

#pragma clang diagnostic pop


