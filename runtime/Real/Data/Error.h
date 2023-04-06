#ifndef REAL_ERROR_H
#define REAL_ERROR_H

#include <Data/String.h>

namespace Data {
namespace Error {
    
class Error {
public:
    static Error from_errno(int code) { return Error(code); }
    static Error from_string(String::String string) { return Error(string); }

    bool is_errno() const { return m_code != 0; }

    int code() const { return m_code; }
    String::String string() const { return m_string; }
protected:
    Error(int code)
        : m_code(code) {}
private:
    Error(String::String string)
        : m_string(string) {}

    int m_code { 0 };
    String::String m_string;
};

template<typename T>
class ErrorOr { };

} // namespace Error
} // namespace Data


#endif // REAL_ERROR_H