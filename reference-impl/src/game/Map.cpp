//
// Created by gamerpuppy on 6/24/2021.
//

#include <cmath>
#include <cassert>
#include <iostream>

#include "game/Map.h"
#include "game/Random.h"

using namespace sts;

constexpr int MAP_HEIGHT = 15;
constexpr int MAP_WIDTH = 7;
constexpr int PATH_DENSITY = 6;

constexpr int MIN_ANCESTOR_GAP = 3;
constexpr int MAX_ANCESTOR_GAP = 5;

constexpr int ROW_END_NODE = MAP_WIDTH-1;

constexpr float SHOP_ROOM_CHANCE = 0.05F;
constexpr float REST_ROOM_CHANCE = 0.12F;
constexpr float TREASURE_ROOM_CHANCE = 0.0f;
constexpr float EVENT_ROOM_CHANCE = 0.22f;

constexpr float ELITE_ROOM_CHANCE_A0 = 0.08f;
constexpr float ELITE_ROOM_CHANCE_A1 = ELITE_ROOM_CHANCE_A0 * 1.6f;

void createPaths(Map &map, Random &mapRng);
void filterRedundantEdgesFromFirstRow(Map &map);
void assignRooms(Map &map, Random &mapRng, int ascensionLevel=0);
void assignBurningElite(Map &map, Random &mapRng);

Map::Map(std::uint64_t seed, int ascension, int act, bool assignBurningElite)
    : Map(Map::fromSeed(seed,ascension,act,assignBurningElite)) {}

MapNode &Map::getNode(int x, int y) {
    return nodes.at(y).at(x);
}

const MapNode &Map::getNode(int x, int y) const {
    return nodes.at(y).at(x);
}

void initNodes(Map &map) {
    for (int y = 0; y < MAP_HEIGHT; y++) {
        for (int x = 0; x < MAP_WIDTH; x++) {
            auto &node = map.nodes.at(y).at(x);
            node.x = x;
            node.y = y;
        }
    }
}

#include "base64.h"
#include <vector>
#include <fstream>
#include <iomanip>
#include <cstdint>

uint64_t to_big_endian(uint64_t value) {
    #if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
        return value;
    #elif __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
        return __builtin_bswap64(value);
    #else
        #error "Unknown endianness"
    #endif
}

void Map::writeExitData(std::ostream &os) const {
    std::vector<char> exitData;   
    for (int y = 0; y < MAP_HEIGHT - 1; y++) {
        for (int x = 0; x < MAP_WIDTH; x++) {
            auto &node = nodes.at(y).at(x);
            int left = x - 1;
            int straight = x;
            int right = x + 1;
            int edgesVal = 0;
            for (int i = 0; i < node.edgeCount; i++) {
                if (node.edges[i] == left) {
                    edgesVal |= 4;
                } else if (node.edges[i] == straight) {
                    edgesVal |= 2;
                } else if (node.edges[i] == right) {
                    edgesVal |= 1;
                }
            }
            exitData.push_back(edgesVal);
        }
    }
    std::vector<uint64_t> flattened;
    for (int i = 0; i < exitData.size(); i += 21) {
        uint64_t acc = 0;
        for (int j = 0; j < 21; j++) {
            if (i+j < exitData.size()) {
                acc <<= 3;
                acc |= exitData[i+j];
            }
        }
        flattened.push_back(to_big_endian(acc));
    }
    os << base64_encode(
        reinterpret_cast<const unsigned char*>(flattened.data()), flattened.size()*8
    ) << std::endl;
}

Map Map::fromSeed(std::uint64_t seed, int ascension, int act, bool setBurning) {
    Map map;
    auto offset = act == 1 ? 1 : act*(100*(act-1));
    Random mapRng(seed+offset, "mapRng");
    initNodes(map);
    createPaths(map, mapRng);
    filterRedundantEdgesFromFirstRow(map);
    assignRooms(map, mapRng, ascension);
    if (setBurning) {
        assignBurningElite(map, mapRng);
        map.burningEliteBuff = mapRng.random(0,3);
    }
    /*
    std::cout << "[" << std::endl;
    for (int y = 0; y < MAP_HEIGHT; y++) {
        std::cout << "    [" << std::endl;
        for (int x = 0; x < MAP_WIDTH; x++) {
            auto& node = map.nodes.at(y).at(x);
            if (node.room == Room::NONE) {
                std::cout << "        None, " << std::endl;
                continue;
            }
            std::cout << "        Some(";
            switch (node.room) {
                case Room::SHOP:
                    std::cout << "Room::Shop(";
                    break;
                case Room::REST:
                    std::cout << "Room::Campfire(";
                    break;
                case Room::EVENT:
                    std::cout << "Room::Event(";
                    break;
                case Room::ELITE:
                    if (x == map.burningEliteX && y == map.burningEliteY) {
                        std::cout << "Room::BurningElite" << (map.burningEliteBuff + 1) << "(";
                    } else {
                        std::cout << "Room::Elite(";
                    }
                    break;
                case Room::MONSTER:
                    std::cout << "Room::Monster(";
                    break;
                case Room::TREASURE:
                    std::cout << "Room::Treasure(";
                    break;
                case Room::BOSS:
                    std::cout << "Room::Boss(";
                    break;
                default:
                    break;
            }
            for (int i = 0; i < node.edgeCount; i++) {
                if (i > 0) {
                    std::cout << " | ";
                }
                if (node.edges[i] < x) {
                    std::cout << "Exit::Left";
                } else if (node.edges[i] == x) {
                    std::cout << "Exit::Straight";
                } else {
                    std::cout << "Exit::Right";
                }
            }
            std::cout << "))," << std::endl;
        }
        std::cout << "    ]," << std::endl;
    }
    std::cout << "];" << std::endl;
    */
    return map;
}

Map Map::act4Map() {
    Map map;
    initNodes(map);
    auto &restNode = map.getNode(3,0);
    auto &shopNode = map.getNode(3,1);
    auto &eliteNode = map.getNode(3,2);
    auto &bossNode = map.getNode(3,3);

    restNode.room = Room::REST;
    shopNode.room = Room::SHOP;
    eliteNode.room = Room::ELITE;
    bossNode.room = Room::BOSS;

    restNode.addEdge(3);
    shopNode.addEdge(3);
    eliteNode.addEdge(3);

    bossNode.addParent(3); // not really necessary to add parents
    eliteNode.addParent(3);
    shopNode.addParent(3);

    return map;
}

void Map::normalizeParents() {
    for (int row = 1; row < 15; ++row) {
        for (int col = 0; col < 7; ++col) {
            auto &node = getNode(col, row);
            bool found[7] = {false};
            for (int i = 0; i < node.parentCount; ++i) {
                found[node.parents[i]] = true;
            }
            node.parentCount = 0;
            for (int i = 0; i < 7; ++i) {
                if (found[i]) {
                    node.addParent(i);
                }
            }
        }
    }
}


static inline int randRange(Random &rng, int min, int max) {
    return rng.random(max - min) + min;
}

inline void insertEdge(MapNode &mapNode, int dstX, int idx) {
    for (int x = mapNode.edgeCount; x > idx; --x) {
        mapNode.edges[x] = mapNode.edges[x-1];
    }
    mapNode.edges[idx] = dstX;
    ++mapNode.edgeCount;
}

char MapNode::getRoomSymbol() const {
    return sts::getRoomSymbol(room);
}

void MapNode::addParent(int parent) {
    parents[parentCount++] = parent;
}

inline void MapNode::addEdge(int edge) {
    int cur = 0;
    while (true) {
        if (cur == edgeCount) {
            edges[cur] = edge;
            ++edgeCount;
            return;
        }

        if (edge == edges[cur]) {
            return;
        }

        if (edge < edges[cur]) {
            insertEdge(*this, edge, cur);
            return;
        }
        ++cur;
    }
}

inline int MapNode::getMaxEdge() const {
//    assert(edgeCount > 0);
    return edges.at(edgeCount-1);
}

inline int MapNode::getMinEdge() const {
//    assert(edgeCount > 0);
    return edges.at(0);
}

inline int MapNode::getMaxXParent() const {
    int maxParent = parents[0];
    for (int i = 1; i < parentCount; i++) {
        if (parents[i] > maxParent) {
            maxParent = parents[i];
        }
    }
    return maxParent;
}

inline int MapNode::getMinXParent() const {
    int minParent = parents[0];
    for (int i = 1; i < parentCount; i++) {
        if (parents[i] < minParent) {
            minParent = parents[i];
        }
    }
    return minParent;
}

void removeEdge(MapNode &node, int idx) {
    for (int i = idx; i < node.edgeCount-1; i++) {
        std::swap(node.edges[i], node.edges[i+1]);
    }
    --node.edgeCount;
}

void removeParentAtIdx(MapNode &node, int parentIdx) {
    for (int i = parentIdx; i < node.parentCount-1; ++i) {
        node.parents[i] = node.parents[i+1];
    }
    --node.parentCount;
}

void removeParent(MapNode &node, int parent) {
    for (int i = node.parentCount-1; i >= 0; --i) {
        if (node.parents[i] == parent) {
            removeParentAtIdx(node, i);
        }
    }
}

void filterRedundantEdgesFromFirstRow(Map &map) {
    bool nodesVisited[7] = {false};
    for (int srcX = 0; srcX < 7; ++srcX) {
        auto &node = map.getNode(srcX, 0);
        for (int i = node.edgeCount-1; i >= 0 ; --i) {
            int destX = node.edges[i];
            if (nodesVisited[destX]) {
                removeParent(map.getNode(destX, 1), srcX);
                removeEdge(node, i);
            } else {
                nodesVisited[destX] = true;
            }
        }
    }
}

inline int getCommonAncestor(const Map &map, int x1, int x2, int y) {
    if (y < 0) {
        return -1;
    }

    int l_node;
    int r_node;
    if (x1 < y) {
        l_node = x1;
        r_node = x2;
    } else {
#ifdef DEBUG        
        std::cout << "x1: " << x1 << ", x2: " << x2 << ", y: " << y << std::endl;
#endif
        l_node = x2;
        r_node = x1;
    }

    if (map.getNode(l_node, y).parentCount == 0 || map.getNode(r_node, y).parentCount == 0) {
        return -1;
    }

    int leftX = map.getNode(l_node, y).getMaxXParent();
    if (leftX == map.getNode(r_node, y).getMinXParent()) {
        return leftX;
    }
    return -1;
}

inline int choosePathParentLoopRandomizer(const Map &map, Random &rng, int curX, int curY, int newX) {
    const MapNode &newEdgeDest = map.getNode(newX, curY + 1);
#ifdef DEBUG
    std::cout << "Investigating destination node: " << (curY + 1 ) << " " << newX << " (" << newEdgeDest.parentCount << "): ";
    for (int i = 0; i < newEdgeDest.parentCount; i++) {
        std::cout << newEdgeDest.parents.at(i) << " ";
    }
    std::cout << std::endl;
    bool cycle_detected = false;
#endif

    for (int i = 0; i < newEdgeDest.parentCount; i++) {
        int parentX = newEdgeDest.parents.at(i);
        if (curX == parentX) {
            continue;
        }
        if (getCommonAncestor(map, parentX, curX, curY) == -1) {
            continue;
        }
#ifdef DEBUG
        cycle_detected = true;
#endif
        //std::cout << "Cycle detected, iteration " << i << " curX: " << curX << " newX: " << newX << " parentX: " << parentX << std::endl;
        if (newX > curX) {
            //std::cout << "newX > curX so sampling " << (curX - 1) << ", " << curX << std::endl;
            newX = curX + randRange(rng, -1, 0);
            if (newX < 0) {
                newX = curX;
            }
        } else if (newX == curX) {
            //std::cout << "newX == curX so sampling " << (curX - 1) << ", " << curX << ", " << (curX + 1) << std::endl;
            newX = curX + randRange(rng, -1, 1);
            if (newX > ROW_END_NODE) {
                newX = curX - 1;
            } else if (newX < 0) {
                newX = curX + 1;
            }
        } else {
            //std::cout << "newX < curX so sampling " << curX  << ", " << (curX + 1) << std::endl;
            newX = curX + randRange(rng, 0, 1);
            if (newX > ROW_END_NODE) {
                newX = curX;
            }
        }
        //std::cout << "Cycle iteration " << i << " remedy: " << newX << ", which is " << (newX - curX) << std::endl;
    }
#ifdef DEBUG    
    if (cycle_detected) {
        std::cout << "Cycle final remedy: " << newX << std::endl;
    }
#endif

    return newX;
}

inline int choosePathAdjustNewX(const Map &map, int curX, int curY, int newEdgeX) {
    if (curX != 0) {
        auto right_node = map.getNode(curX - 1, curY);
        if (right_node.edgeCount > 0) {
            int left_edge_of_right_node = right_node.getMaxEdge();
            if (left_edge_of_right_node > newEdgeX) {
                newEdgeX = left_edge_of_right_node;
            }
        }
    }

    if (curX < ROW_END_NODE) {
        auto right_node = map.getNode(curX + 1, curY);
        if (right_node.edgeCount > 0) {
            int left_edge_of_right_node = right_node.getMinEdge();
            if (left_edge_of_right_node < newEdgeX) {
                newEdgeX = left_edge_of_right_node;
            }
        }
    }
    return newEdgeX;
}


int chooseNewPath(Map &map, Random &rng, int curX, int curY) {
    MapNode &currentNode = map.getNode(curX, curY);

    int min;
    int max;
    if (curX == 0) {
        min = 0;
        max = 1;
    } else if (curX == ROW_END_NODE) {
        min = -1;
        max = 0;
    } else {
        min = -1;
        max = 1;
    }

    int r = randRange(rng, min, max);
    //std::cout << "curY: " << curY << ", curX: " << curX << ", r: " << r << " from min=" << min << ", max=" << max << std::endl;
    int newEdgeX = curX + r;
    //std::cout << "First proposed exit from node at " << curY << " " << curX << " is " << (newEdgeX - curX) << std::endl;
    newEdgeX = choosePathParentLoopRandomizer(map, rng, curX, curY, newEdgeX);
    //std::cout << "Next proposed exit from node at " << curY << " " << curX << " is " << (newEdgeX - curX) << std::endl;
    newEdgeX = choosePathAdjustNewX(map, curX, curY, newEdgeX);
    //std::cout << "Final proposed exit from node at " << curY << " " << curX << " is " << (newEdgeX - curX) << std::endl;

    return newEdgeX;
}

void createPathsIteration(Map &map, Random &rng, int startX) {
    int curX = startX;
    for (int curY = 0; curY < MAP_HEIGHT-1; ++curY) {
        int newX = chooseNewPath(map, rng, curX, curY);
        //std::cout << "curY: " << curY << " curX: " << curX << " newX: " << newX << std::endl;
        map.getNode(curX, curY).addEdge(newX);
        map.getNode(newX, curY+1).addParent(curX);
        //std::cout << map.toString() << std::endl;
        curX = newX;
    }
    map.getNode(curX, 14).addEdge(3);
}

void createPaths(Map &map, Random &mapRng) {
    int firstStartX = randRange(mapRng, 0, MAP_WIDTH - 1);
    //std::cout << "First Start X: " << firstStartX << std::endl;
    createPathsIteration(map, mapRng, firstStartX);
    //std::cout << "RNG counter after first path: " << mapRng.counter1 << std::endl;
    //std::cout << map.toString() << std::endl;

    for(int i = 1; i < PATH_DENSITY; ++i) {
        int startX = randRange(mapRng, 0, MAP_WIDTH - 1);

        while(startX == firstStartX && i == 1) {
            startX = randRange(mapRng, 0, MAP_WIDTH - 1);
        }

        createPathsIteration(map, mapRng, startX);
        //std::cout << "RNG counter after iteration " << (i+1) << ": " << mapRng.counter1 << std::endl;
        //std::cout << map.toString() << std::endl;
    }
}

std::string paddingGenerator(int length) {
    std::string ret;
    for(int i = 0; i < length; ++i) {
        ret += ' ';
    }
    return ret;
}

std::string Map::toString(bool showRoomSymbols) const {
    std::string str;

    int lastRow = 14;
    int left_padding_size = 5;


    bool hitNonEmptyRow = false;

    for(int y = 14; y >= 0; y--) {

        if (!hitNonEmptyRow) {
            bool empty = true;
            for (int x = 0; x < 7; ++x) {
                if (getNode(x,y).parentCount > 0) {
                    empty = false;
                    break;
                }
            }
            if (empty) {
                continue;
            } else {
                hitNonEmptyRow = true;
            }
        }

        str.append("\n");
        //str.append("\n ").append(paddingGenerator(left_padding_size));
        for (int x = 0; x < 7; ++x) {
            auto node = getNode(x, y);
            std::string right = " ";
            std::string mid = " ";
            std::string node_symbol = " ";

            for (int i = 0; i < node.edgeCount; ++i) {
                const int edge = node.edges[i];
                if (edge < x) {
                    node_symbol = "\\";
                }

                if (edge == x) {
                    mid = "|";
                }

                if (edge > x) {
                    right = "/";
                }
            }
            str.append(node_symbol).append(mid).append(right);
        }
        str.append("\n");
        //str.append("\n").append(std::to_string(y)).append(" ");
        //str.append(paddingGenerator(left_padding_size - (int)std::to_string(y).length()));

        for (int x = 0; x < 7; ++x) {
            auto node = getNode(x, y);
            std::string node_symbol = " ";

            if (y == lastRow) {
                for (auto &lower_node : nodes.at(y - 1)) {
                    for (int i = 0; i < lower_node.edgeCount; i++) {
                        if (lower_node.edges[i] == x) {
                            node_symbol = showRoomSymbols ? node.getRoomSymbol() : '*';
                        }
                    }
                }
            } else {
                if (node.edgeCount > 0 || node.room == Room::BOSS) {
                    node_symbol = showRoomSymbols ? node.getRoomSymbol() : '*';
                }
                if (node.x == burningEliteX && node.y == burningEliteY) {
                    switch (burningEliteBuff) {
                        case 0:
                            node_symbol = '1';
                            break;
                        case 1:
                            node_symbol = '2';
                            break;
                        case 2:
                            node_symbol = '3';
                            break;
                        case 3:
                            node_symbol = '4';
                            break;
                        default:
                            node_symbol = 'e';
                    }
                }
            }
            str.append(" ").append(node_symbol).append(" ");
        }
    }

    return str;
}

struct RoomCounts {
    float total = 0;
    int unassigned = 0;
};

RoomCounts getRoomCountsAndAssignFixed(Map &map) {
    const int monsterRow = 0;
    const int treasureRow = 8;

    const int restRow = static_cast<int>(MAP_HEIGHT-1);
    const int restRowBug = static_cast<int>(MAP_HEIGHT-2);

    RoomCounts counts;
    for (int row = 0; row < MAP_HEIGHT; ++row) {

        for (auto &node : map.nodes.at(row)) {
            if (node.edgeCount <= 0) {
                continue;
            }

            switch (row) {
                case monsterRow:
                    node.room = Room::MONSTER;
                    ++counts.total;
                    break;

                case treasureRow:
                    node.room = Room::TREASURE;
                    ++counts.total;
                    break;

                case restRow:
                    node.room = Room::REST;
                    ++counts.total;
                    break;

                case restRowBug:
                    ++counts.unassigned;
                    break;

                default:
                    ++counts.unassigned;
                    ++counts.total;
            }
        }
    }

    return counts;
}

void fillRoomArray(Room *arr, RoomCounts counts, float eliteRoomChance) {

    int shopCount = static_cast<int>(std::round(counts.total * SHOP_ROOM_CHANCE));
    int restCount = static_cast<int>(std::round(counts.total * REST_ROOM_CHANCE));
    int treasureCount = static_cast<int>(std::round(counts.total * TREASURE_ROOM_CHANCE));
    int eliteCount = static_cast<int>(std::round(counts.total * eliteRoomChance));
    int eventCount = static_cast<int>(std::round(counts.total * EVENT_ROOM_CHANCE));

    int i = 0;
    int end = shopCount;
    for (; i < shopCount; ++i) {
        arr[i] = Room::SHOP;
    }

    end += restCount;
    for (; i < end; ++i) {
        arr[i] = Room::REST;
    }

    end += treasureCount;
    for (; i < end; ++i) {
        arr[i] = Room::TREASURE;
    }


    end += eliteCount;
    for (; i < end; ++i) {
        arr[i] = Room::ELITE;
    }

    end += eventCount;
    for (; i < end; ++i) {
        arr[i] = Room::EVENT;
    }

    assert(i < counts.total); // this means that a really weird map was generated?>? is this possible?
    for (; i < counts.unassigned; ++i) {
        arr[i] = Room::MONSTER;
    }

}

struct RoomConstructorData {
    Room *rooms;
    int roomCount;
    int offset = 0;

    std::uint64_t rowData = 0;
    std::uint64_t prevRowData;

    std::uint64_t siblingMasks[MAP_WIDTH] = {0};
    std::uint64_t nextSiblingMasks[MAP_WIDTH] = {0};

    std::uint64_t parentMasks[MAP_WIDTH] = {0};
    std::uint64_t nextParentMasks[MAP_WIDTH] = {0};

    constexpr static std::uint64_t masks[] {
            0x0101010101010101ULL,
            0x0202020202020202ULL,
            0x0404040404040404ULL,
            0x0808080808080808ULL,
            0x1010101010101010ULL,
            0x2020202020202020ULL,
            0x4040404040404040ULL
    };


    RoomConstructorData(Room *rooms, int roomCount) : rooms(rooms), roomCount(roomCount) {}

    void setData(const MapNode &node) {
        if (node.edgeCount == 1) {
            for (int i = 0; i < node.edgeCount; i++) {
                nextParentMasks[node.edges[i]] |= 0xFFULL << (node.x*8);
            }

        } else {
            std::uint64_t siblingMask = 0;
            for (int i = 0; i < node.edgeCount; i++) {
                int edge = node.edges[i];
                siblingMask |= 0xFFULL << (node.edges[i]*8U);
                nextSiblingMasks[edge] |= siblingMask;
                nextParentMasks[edge] |= 0xFFULL << (node.x*8U);
            }
        }
    }

    void setCurDataOnly(const MapNode &node) {
        rowData |= 1ULL << ((unsigned int)node.room + node.x*8U);
    }

    void setNextDataOnly(const MapNode &node) {
        if (node.edgeCount == 1) {
            for (int i = 0; i < node.edgeCount; i++) {
                nextParentMasks[node.edges[i]] |= 0xFFULL << (node.x*8U);
            }

        } else {
            std::uint64_t siblingMask = 0;
            for (int i = 0; i < node.edgeCount; i++) {
                int edge = node.edges[i];
                siblingMask |= 0xFFULL << (node.edges[i]*8U);
                nextSiblingMasks[edge] |= siblingMask;
                nextParentMasks[edge] |= 0xFFULL << (node.x*8U);
            }
        }
    }

    void removeElement(int idx) {
        for (int i = idx; i > offset; --i) {
            rooms[i] = rooms[i-1];
        }
        ++offset;
    }

    void nextRow() {
        prevRowData = rowData;
        rowData = 0;

        for (int i = 0; i < MAP_WIDTH; i++) {
            siblingMasks[i] = nextSiblingMasks[i];
            nextSiblingMasks[i] = 0;

            parentMasks[i] = nextParentMasks[i];
            nextParentMasks[i] = 0;
        }
    }

};

bool doesSiblingMatch(const RoomConstructorData &data, int nodeX, Room roomToBeSet) {
    return data.rowData & data.siblingMasks[nodeX] & RoomConstructorData::masks[(int)roomToBeSet];
}

bool doesParentMatch(const RoomConstructorData &data, int nodeX, Room roomToBeSet) {
    return data.prevRowData & data.parentMasks[nodeX] & RoomConstructorData::masks[(int)roomToBeSet];
}

void assignRoomToNode(Map &map, MapNode &node, RoomConstructorData &data) {
    bool triedAssignRoom[5] = { false };

    for (int i = data.offset; i < data.roomCount; i++) {
        Room room = data.rooms[i];

        if (triedAssignRoom[(int)room]) {
            continue;
        }
        triedAssignRoom[(int)room] = true;

        switch (room) {
            case Room::SHOP:
                break;

            case Room::ELITE:
                if (node.y <= 4) {
                    continue;
                }
                break;

            case Room::REST:
                if (node.y <= 4) {
                    continue;
                }
                if (node.y >= 13) {
                    continue;
                }
                break;

            case Room::EVENT:
                if (doesSiblingMatch(data, node.x, room)) {
                    continue;
                } else {
                    node.room = Room::EVENT;
                    data.rowData |= 1ULL << ((unsigned int)Room::EVENT + node.x*8U);
                    data.removeElement(i);
                    return;
                }

            case Room::MONSTER:
                if (doesSiblingMatch(data, node.x, room)) {
                    continue;
                }
                node.room = Room::MONSTER;
                data.rowData |= 1ULL << ((unsigned int)Room::MONSTER + node.x*8U);
                data.removeElement(i);
                return;

            default:
                break;
        }

        bool canSet = !doesParentMatch(data, node.x, room)
                      && !doesSiblingMatch(data, node.x, room);
        if (canSet) {
            node.room = room;
            data.rowData |= 1ULL << ((unsigned int)node.room + node.x*8U);
            data.removeElement(i);
            return;
        }
    }

    node.room = sts::Room::MONSTER;
}

void assignRoomsRow(Map &map, RoomConstructorData &data, int row) {
    for (auto &node : map.nodes.at(row)) {
        if (node.edgeCount <= 0) {
            continue;
        }

        if (row == 0 || row == 8) {
            data.setNextDataOnly(node);
        } else if (row == 7 || row == 13) {
            assignRoomToNode(map, node, data);
            data.setCurDataOnly(node);
        } else {
            assignRoomToNode(map, node, data);
            data.setData(node);
        }
    }
    data.nextRow();
}

// idea: remove rooms basically in reverse, shifting the head of the array forward instead
void assignRoomsToNodes(Map &map, Room *rooms, int roomsSize) {
    RoomConstructorData data(rooms, roomsSize);
    for (int row = 0; row < MAP_HEIGHT-1; ++row) {
        assignRoomsRow(map, data, row);
    }
}

void assignRooms(Map &map, Random &rng, int ascensionLevel) {
    RoomCounts counts = getRoomCountsAndAssignFixed(map);

    Room rooms[counts.unassigned];
    //std::cout << "Room counts: " << counts.total << " " << counts.unassigned << std::endl;
    fillRoomArray(rooms, counts, ascensionLevel > 0 ? ELITE_ROOM_CHANCE_A1 : ELITE_ROOM_CHANCE_A0);
    //std::cout << "Initial room array:  ";
    //for (int i = 0; i < counts.unassigned; i++) {
    //    std::cout << getRoomSymbol(rooms[i]);
    //}
    //std::cout << std::endl;

    for (int i=counts.unassigned; i>1; i--) {
        std::swap(rooms[i-1], rooms[rng.nextInt(i)]);
    }
    //std::cout << "Permuted room array: ";
    //for (int i = 0; i < counts.unassigned; i++) {
    //    std::cout << getRoomSymbol(rooms[i]);
    //}
    //std::cout << std::endl;

    assignRoomsToNodes(map, rooms, counts.unassigned);
}

struct IntTuple {
    int x;
    int y;

    IntTuple() = default;
    IntTuple(int x, int y) : x(x), y(y) {}
};

void assignBurningElite(Map &map, Random &mapRng) {
    int eliteRoomCount = 0;
    std::array<IntTuple,14> eliteRooms{};

    for (int row = 0; row < 15; ++row) {
        for (int col = 0; col < 7; ++col) {
            if (map.getNode(col, row).room == sts::Room::ELITE) {
                eliteRooms[eliteRoomCount++] = IntTuple(col,row);
            }
        }
    }

    // if number of elite rooms is 1 it will crash the base game too.
    int idx = mapRng.random(eliteRoomCount-1);
    map.burningEliteX = eliteRooms.at(idx).x;
    map.burningEliteY = eliteRooms.at(idx).y;
}

