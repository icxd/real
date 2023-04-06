#include "HelloWorld.h"
#include <Prelude.h>
#include <Data/Map.h>
namespace HelloWorld {
using namespace Prelude;
using namespace Data::Map;
using Data::Map::Map;
List<List<int>> Test() {
return List<List<int>>::from_array(new List<int>[2]{ List<int>::from_array(new int[2]{ 1, 2 }, 2), List<int>::from_array(new int[2]{ 3, 4 }, 2) }, 2);
}
Optional<int> ListTest() {
return List<int>::from_array(new int[4]{ 1, 2, 3, 4 }, 4).length();
}
int StringTest() {
return String::from_cstr("Hello World").length();
}
Map<String, int> MapTest() {
return Map<String, int>::from_list(List<String>::from_array(new String[2]{ String::from_cstr("a"), String::from_cstr("b") }, 2), List<int>::from_array(new int[2]{ 1, 2 }, 2));
}
}
