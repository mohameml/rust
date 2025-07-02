#include <iostream>
#include <thread>

int x = 0;

void f1() { x += 1; }
void f2() { x += 1; }

int main() {
    std::thread t1(f1);
    std::thread t2(f2);

    t1.join();
    t2.join();

    std::cout << "x = " << x << std::endl;
}
