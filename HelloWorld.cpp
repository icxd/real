#include "HelloWorld.h"
#include <Data/String.h>
#include <Data/List.h>
#include <IO/Stdio.h>
using namespace Data::String;
using namespace Data::List;
using namespace IO::Stdio;
namespace HelloWorld {
using Data::String::String;
using Data::List::List;
using IO::Stdio::println;
void Main(List<String> args) {
println("Hello, World!");
}
}
