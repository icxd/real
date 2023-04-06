#ifndef REAL_LIST_H
#define REAL_LIST_H

namespace Data {
namespace List {

template<typename T>
class List {
public:
    List() = default;
    ~List() = default;

    static List<T> from_array(T array[], int length) {
        List<T> list;
        for (int i = 0; i < length; i++) {
            list.append(array[i]);
        }
        return list;
    }


    void append(T value) {
        if (m_size == m_capacity) {
            m_capacity *= 2;
            T* new_data = new T[m_capacity];
            for (int i = 0; i < m_size; i++) {
                new_data[i] = m_data[i];
            }
            delete[] m_data;
            m_data = new_data;
        }
        m_data[m_size] = value;
        m_size++;
    }

    T& operator[](int index) {
        return m_data[index];
    }
    
    int length() {
        return m_size;
    }

private:
    T* m_data { nullptr };
    int m_size { 0 };
    int m_capacity { 8 };
};

} // namespace List
} // namespace Data

#endif // REAL_LIST_H