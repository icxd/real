#ifndef REAL_MAP_H
#define REAL_MAP_H

#include <Data/List.h>

namespace Data {
namespace Map {

template<typename K, typename V>
class Map {
public:
    Map() = default;
    ~Map() = default;

    Map(const Map<K, V>& other)
        : m_keys(other.m_keys)
        , m_values(other.m_values)
    {
    }

    static Map<K, V> from_list(List::List<K> keys, List::List<V> values) {
        Map<K, V> map;
        for (int i = 0; i < keys.size(); i++) {
            map.insert(keys[i], values[i]);
        }
        return map;
    }

    Map<K, V>& operator=(const Map<K, V>& other) {
        m_keys = other.m_keys;
        m_values = other.m_values;
        return *this;
    }

    void insert(K key, V value) {
        m_keys.push_back(key);
        m_values.push_back(value);
    }

    V get(K key) const {
        for (int i = 0; i < m_keys.size(); i++) {
            if (m_keys[i] == key) {
                return m_values[i];
            }
        }
        return V();
    }

    bool contains(K key) const {
        for (int i = 0; i < m_keys.size(); i++) {
            if (m_keys[i] == key) {
                return true;
            }
        }
        return false;
    }

    void remove(K key) {
        for (int i = 0; i < m_keys.size(); i++) {
            if (m_keys[i] == key) {
                m_keys.remove(i);
                m_values.remove(i);
                return;
            }
        }
    }

    int size() const {
        return m_keys.size();
    }

private:
    List::List<K> m_keys;
    List::List<V> m_values;
};

} // namespace Map
} // namespace Data

#endif // REAL_MAP_H