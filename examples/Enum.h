#ifndef ENUM_H
#define ENUM_H

#include <variant>

namespace Data {
namespace Enum {

template<typename... Variants>
using Enum = std::variant<Variants>;

} // namespace String
} // namespace Data


#endif // ENUM_H