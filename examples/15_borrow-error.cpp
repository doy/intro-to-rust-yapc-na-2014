#include <iostream>

int *foo(void)
{
    int x = 2;
    return &x;
}

int main(int argc, char *argv[])
{
    int *x = foo();
    std::cout << *x << std::endl;
}

