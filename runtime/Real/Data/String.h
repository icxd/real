#ifndef REAL_STRING_H
#define REAL_STRING_H

namespace Data {
namespace String {
    
class String {
public:
    String() = default;
    ~String() = default;

    static String from_cstr(const char* cstr) { return String(cstr); }

    int length() const {
        int length = 0;
        while (m_cstr[length] != '\0') {
            length++;
        }
        return length;
    }

    const char* cstr() const { return m_cstr; }
private:
    String(const char* cstr)
        : m_cstr(cstr) {}
    const char* m_cstr;
};

} // namespace String
} // namespace Data


#endif // REAL_STRING_H