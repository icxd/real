#ifndef TEST_REAL_H
#define TEST_REAL_H
namespace Test {
using Float = int;
template <typename T = int, typename U = int>
T Add(T t, U u);
template <typename T = Float, typename U = Float>
T Add(T t, U u);
template <typename T = int, typename U = Float>
U Add(T t, U u);
template <typename T = Float, typename U = int>
T Add(T t, U u);
}
#endif // TEST_REAL_H
