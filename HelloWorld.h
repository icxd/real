#ifndef HELLOWORLD_REAL_H
#define HELLOWORLD_REAL_H
#include <Prelude.h>
#include <Data/Map.h>
namespace HelloWorld {
using namespace Prelude;
using namespace Data::Map;
using Data::Map::Map;
List<List<int>> Test();
Optional<int> ListTest();
int StringTest();
Map<String, int> MapTest();
}
#endif // HELLOWORLD_REAL_H
