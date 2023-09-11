#include <iostream>

int main() {
    std::vector<int> v = {1, 2, 3};

    auto it = v.begin();
    auto end = v.end();

    for (; it != end; ++it) {
        v.push(5);
    }
}
