#include "WhateverExample.h"

namespace Folder {
namespace Subfolder {

/*internal*/ void ParentObject::ExampleProcedure() {
    println("Hi from parent!");
}

namespace Whatever {

template<typename T, typename U>
void MyGenericObject::ExampleProcedure() {
    println("Hi from child!");
}

template<typename T, typename U>
int MyGenericObject::MyPrivateProcedure(Whatever whatever) {
    std::visit([this](auto&& arg) {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, Whatever_Variants::A>) {
            return 1;
        } else if constexpr (std::is_same_v<T, Whatever_Variants::B>) {
            return 2;
        } else if constexpr (std::is_same_v<T, Whatever_Variants::C>) {
            return 3;
        } else {
            throw std::runtime_error("No match");
        }
    }, whatever);
}

/*public*/ void MyProcedure(MyData a) {
    std::visit([](auto&& arg) {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, MyData_Variants::A>) {
            auto x = arg.__0; auto y = arg.__1;
            println("%d", x + y);
        } else if constexpr (std::is_same_v<T, MyData_Variants::B>) {
            auto x = arg.__0; auto y = arg.__1;
            println("%d", x - y);
        } else if constexpr (std::is_same_v<T, MyData_Variants::C>) {
            auto x = arg.__0; auto y = arg.__1;
            println("%d", x * y);
        } else {
            println("No match");
        }
    }, a);
}

}
}
}