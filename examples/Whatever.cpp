#include "Whatever.h"
namespace Folder {
namespace Subfolder {
namespace Whatever {
void ParentObject::ExampleProcedure() {
println("Hi from parent!");
}
void MyGenericObject::ExampleProcedure() {
println("Hi from child!");
}
int MyGenericObject::MyPrivateProcedure(Whatever whatever) {
std::visit([this](auto&& whatever) {
using T = std::decay_t<decltype(whatever)>;
if constexpr (std::is_same_v<T, Whatever_Variants::A>) {
return 1;
}
if constexpr (std::is_same_v<T, Whatever_Variants::B>) {
return 2;
}
if constexpr (std::is_same_v<T, Whatever_Variants::C>) {
return 3;
}
}, whatever);
}
void MyProcedure(MyData a) {
std::visit([this](auto&& a) {
using T = std::decay_t<decltype(a)>;
if constexpr (std::is_same_v<T, MyData_Variants::A>) {
auto x = a.__0;
auto y = a.__1;
println("%d", x + y);
return;
}
if constexpr (std::is_same_v<T, MyData_Variants::B>) {
auto x = a.__0;
auto y = a.__1;
println("%d", x - y);
return;
}
if constexpr (std::is_same_v<T, MyData_Variants::C>) {
auto x = a.__0;
auto y = a.__1;
println("%d", x * y);
return;
}
return println("No match");
}, a);
}
}
}
}
