#include "Whatever.h"

using Folder::Subfolder::Whatever::Error;
using Folder::Subfolder::Whatever::Result;
using Folder::Subfolder::Whatever::Result_Variants::Ok;

Result<int, Error> Main() {
    return Ok<int>{ 0 };
}

int main(int argc, char const *argv[]){
    return std::visit([](auto&& arg) {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, Ok<int>>) {
            return arg.__0;
        } else {
            return 1;
        }
    }, Main());
}
