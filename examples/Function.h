#ifndef FUNCTION_H
#define FUNCTION_H

#include <functional>

namespace Data {
namespace Function {

template<typename T, typename... Args>
using Function = std::function<T(Args...)>;

} // namespace String
} // namespace Data


#endif // FUNCTION_H