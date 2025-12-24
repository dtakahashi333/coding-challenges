#include <cassert>
#include <vector>
#include "leet_code/common/list_node.hpp"
#include "leet_code/easy/has_cycle.hpp"

using namespace std;

void test1()
{
    int arr[] = {3, 2, 0, -4};
    vector<int> vec(arr, arr + 4);
    auto head = vec_to_cyclic_list(vec, 2);
    Solution s;
    assert(s.hasCycle(head) == true);
}

void test2()
{
    int arr[] = {1, 2};
    vector<int> vec(arr, arr + 2);
    auto head = vec_to_cyclic_list(vec, 0);
    Solution s;
    assert(s.hasCycle(head) == true);
}

void test3()
{
    int arr[] = {1};
    vector<int> vec(arr, arr + 1);
    auto head = vec_to_cyclic_list(vec, -1);
    Solution s;
    assert(s.hasCycle(head) == false);
}

void test4()
{
    int arr[] = {3, 2, 0, -4};
    vector<int> vec(arr, arr + 4);
    auto head = vec_to_cyclic_list(vec, 2);
    Solution s;
    assert(s.hasCycle2(head) == true);
}

void test5()
{
    int arr[] = {1, 2};
    vector<int> vec(arr, arr + 2);
    auto head = vec_to_cyclic_list(vec, 0);
    Solution s;
    assert(s.hasCycle2(head) == true);
}

void test6()
{
    int arr[] = {1};
    vector<int> vec(arr, arr + 1);
    auto head = vec_to_cyclic_list(vec, -1);
    Solution s;
    assert(s.hasCycle2(head) == false);
}

int main()
{
    test1();
    test2();
    test3();
    return 0;
}