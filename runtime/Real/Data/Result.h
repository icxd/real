#ifndef REAL_RESULT_H
#define REAL_RESULT_H

namespace Data {
namespace Result {

template<typename T, typename E>
class Result {
    using ResultType = T;
    using ErrorType = E;
public:
    Result() = default;
    ~Result() = default;

    Result(ResultType value)
        : m_value(value)
        , m_has_value(true)
    {
    }

    Result(const Result<ResultType, ErrorType>& other)
        : m_value(other.m_value)
        , m_has_value(other.m_has_value)
    {
    }

    Result<ResultType, ErrorType>& operator=(const Result<ResultType, ErrorType>& other) {
        m_value = other.m_value;
        m_has_value = other.m_has_value;
        return *this;
    }

    bool is_ok() const {
        return m_has_value;
    }

    bool is_err() const {
        return !m_has_value;
    }

    ResultType ok() const {
        return m_value;
    }

    ErrorType err() const {
        return m_error;
    }

    ResultType ok_or(ResultType default_value) const {
        if (m_has_value) {
            return m_value;
        }
        return default_value;
    }

    ErrorType err_or(ErrorType default_value) const {
        if (!m_has_value) {
            return m_error;
        }
        return default_value;
    }

private:
    ResultType m_value;
    ErrorType m_error;
    bool m_has_value { false };
};

} // namespace Result
} // namespace Data

#endif // REAL_RESULT_H