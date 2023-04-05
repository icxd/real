#ifndef STRING_REAL_H
#define STRING_REAL_H
#include <Data/List.h>
using namespace Data::List;
namespace Data {
namespace String {
using Data::List::List;
class String {
public:
String(List<char> chars) : __chars(chars) {}
~String() = default;
private:
List<char> __chars;
};
}
}
#endif // STRING_REAL_H
