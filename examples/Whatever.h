#ifndef WHATEVER_REAL_H
#define WHATEVER_REAL_H
#include "String.h"
#include "Enum.h"
#include "Function.h"
using namespace Data::String;
using namespace Data::Enum;
using namespace Data::Function;
namespace Folder {
namespace Subfolder {
namespace Whatever {
using Error = String;
namespace Result_Variants {
template <typename R>
struct Ok { R __0; };
template <typename E>
struct Err { E __0; };
}
template <typename R, typename E = Error>
using Result = Enum<Result_Variants::Ok, Result_Variants::Err>;
namespace MyData_Variants {
struct A { int __0; int __1; };
struct B { int __0; int __1; };
struct C { int __0; int __1; };
}
using MyData = Enum<MyData_Variants::A, MyData_Variants::B, MyData_Variants::C>;
namespace Whatever_Variants {
struct A { };
struct B { };
struct C { };
}
using Whatever = Enum<Whatever_Variants::A, Whatever_Variants::B, Whatever_Variants::C>;
constexpr MyData MyObject = MyData { MyData_Variants::A { 1, 2 } };
template <typename T>
using Predicate = Function<bool(T)>;
class ParentObject {
public:
ParentObject() = default;
~ParentObject() = default;
private:
virtual void ExampleProcedure() = 0;
};
template <typename T, typename U>
class MyGenericObject : public ParentObject{
public:
MyGenericObject() = default;
~MyGenericObject() = default;
private:
virtual void ExampleProcedure() override = 0;
int MyPrivateProcedure(Whatever whatever);
};
template <typename T, typename U = String>
class MyExtendsGenericObject {
public:
MyExtendsGenericObject() = default;
~MyExtendsGenericObject() = default;
};
struct MyStruct {
int a;
int b;
};
}
}
}
#endif // WHATEVER_REAL_H
