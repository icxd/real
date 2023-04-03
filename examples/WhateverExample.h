#ifndef FOLDER_SUBFOLDER_WHATEVER
#define FOLDER_SUBFOLDER_WHATEVER

#include <string>
#include <variant>
#include <functional>

class String : public std::string {
public:
    String() : std::string() { }
    String(const char* str) : std::string(str) { }
    String(const std::string& str) : std::string(str) { }
    String(const String& str) : std::string(str) { }
    String(std::string&& str) : std::string(str) { }
    String(String&& str) : std::string(str) { }
    String& operator=(const char* str) { std::string::operator=(str); return *this; }
    String& operator=(const std::string& str) { std::string::operator=(str); return *this; }
    String& operator=(const String& str) { std::string::operator=(str); return *this; }
    String& operator=(std::string&& str) { std::string::operator=(str); return *this; }
    String& operator=(String&& str) { std::string::operator=(str); return *this; }
    String& operator+=(const char* str) { std::string::operator+=(str); return *this; }
    String& operator+=(const std::string& str) { std::string::operator+=(str); return *this; }
    String& operator+=(const String& str) { std::string::operator+=(str); return *this; }
    String& operator+=(std::string&& str) { std::string::operator+=(str); return *this; }
    String& operator+=(String&& str) { std::string::operator+=(str); return *this; }
};

template<typename... Variants>
using Variant = std::variant<Variants...>;

void println(String fmt, ...);

namespace Folder {
namespace Subfolder {
namespace MyData_Variants {
    struct A { int __0; int __1; };
    struct B { int __0; int __1; };
    struct C { int __0; int __1; };
}
/*internal*/ using MyData = Variant<MyData_Variants::A, MyData_Variants::B, MyData_Variants::C>;

/*internal*/ class ParentObject {
public:
    ParentObject() = default;
    ~ParentObject() = default;
    virtual void ExampleProcedure() = 0;
};
namespace Whatever {

using Error = String;

namespace Result_Variants {
    template<typename R>
    struct Ok { R __0; };
    template<typename E = Error>
    struct Err { E __1; };
}
template<typename R, typename E = Error>
using Result = Variant<Result_Variants::Ok<R>, Result_Variants::Err<E>>;

namespace Whatever_Variants {
    struct A { };
    struct B { };
    struct C { };
}
using Whatever = Variant<Whatever_Variants::A, Whatever_Variants::B, Whatever_Variants::C>;

/*public*/ static const MyData MyObject = MyData { MyData_Variants::A { 1, 2 } };

template<typename T>
/*private*/ using Predicate = std::function<bool(T)>;

template<typename T, typename U>
class MyGenericObject : public ParentObject {
public:
    MyGenericObject() = default;
    ~MyGenericObject() = default;
    virtual void ExampleProcedure() override = 0;
private:
    int MyPrivateProcedure(Whatever whatever) = 0;
};

template<typename T = String, typename U = int>
class MyExtendsGenericObject;

struct MyStruct {
    int a;
    int b;
};

}
}
}

#endif // FOLDER_SUBFOLDER_WHATEVER