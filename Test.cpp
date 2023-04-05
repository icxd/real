#include "Test.h"
namespace Test {
template <typename T = int, typename U = int>
T Add(T t, U u) {
return t + u;
}
template <typename T = Float, typename U = Float>
T Add(T t, U u) {
return t + u;
}
template <typename T = int, typename U = Float>
U Add(T t, U u) {
return t + u;
}
template <typename T = Float, typename U = int>
T Add(T t, U u) {
return t + u;
}
}
