#ifndef STRING_REAL_H
#define STRING_REAL_H
namespace Data {
namespace String {
class String {
public:
String(const char* str) : __str(str) {}
~String() = default;
const char* String::c_str();
private:
const char* __str;
};
}
}
#endif // STRING_REAL_H
