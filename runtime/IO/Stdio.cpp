#include <iostream>
#include "Stdio.h"
#include <Data/String.h>
using namespace Data::String;
namespace IO {
namespace Stdio {
using Data::String::String;
#include <iostream>
void println(String msg) {
std::cout << msg << "\n";
}
}
}
