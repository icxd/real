#ifndef REAL_OPTIONAL_H
#define REAL_OPTIONAL_H

namespace Data {
namespace Optional {

template<typename T>
class Optional {
public:
    Optional() = default;
    ~Optional() = default;

    Optional(T value)
        : m_value(value)
        , m_has_value(true)
    {
    }

    Optional(const Optional<T>& other)
        : m_value(other.m_value)
        , m_has_value(other.m_has_value)
    {
    }

    Optional<T>& operator=(const Optional<T>& other) {
        m_value = other.m_value;
        m_has_value = other.m_has_value;
        return *this;
    }

    bool has_value() const {
        return m_has_value;
    }

    T value() const {
        return m_value;
    }

    T value_or(T default_value) const {
        if (m_has_value) {
            return m_value;
        }
        return default_value;
    }

private:
    T m_value;
    bool m_has_value { false };
};

} // namespace Optional
} // namespace Data

#endif // REAL_OPTIONAL_H