#include <gtest/gtest.h>
#include <cassert>
#include <vector>
#include "leet_code/common/list_node.hpp"
#include "leet_code/easy/has_cycle.hpp"

using namespace std;

TEST(Test1, BasicAssertions)
{
    int arr[] = {3, 2, 0, -4};
    vector<int> vec(arr, arr + 4);
    auto head = vec_to_cyclic_list(vec, 2);
    Solution s;
    EXPECT_EQ(s.hasCycle(head), true);
}

TEST(Test2, BasicAssertions)
{
    int arr[] = {1, 2};
    vector<int> vec(arr, arr + 2);
    auto head = vec_to_cyclic_list(vec, 0);
    Solution s;
    EXPECT_EQ(s.hasCycle(head), true);
}

TEST(Test3, BasicAssertions)
{
    int arr[] = {1};
    vector<int> vec(arr, arr + 1);
    auto head = vec_to_cyclic_list(vec, -1);
    Solution s;
    EXPECT_EQ(s.hasCycle(head), false);
}

TEST(Test4, BasicAssertions)
{
    int arr[] = {3, 2, 0, -4};
    vector<int> vec(arr, arr + 4);
    auto head = vec_to_cyclic_list(vec, 2);
    Solution s;
    EXPECT_EQ(s.hasCycle2(head), true);
}

TEST(Test5, BasicAssertions)
{
    int arr[] = {1, 2};
    vector<int> vec(arr, arr + 2);
    auto head = vec_to_cyclic_list(vec, 0);
    Solution s;
    EXPECT_EQ(s.hasCycle2(head), true);
}

TEST(Test6, BasicAssertions)
{
    int arr[] = {1};
    vector<int> vec(arr, arr + 1);
    auto head = vec_to_cyclic_list(vec, -1);
    Solution s;
    EXPECT_EQ(s.hasCycle2(head), false);
}

void test_has_cycle1()
{
    int arr[] = {3, 2, 0, -4};
    vector<int> vec(arr, arr + 4);
    auto head = vec_to_cyclic_list(vec, 2);
    Solution s;
    assert(s.hasCycle(head) == true);
}

void test_has_cycle2()
{
    int arr[] = {1, 2};
    vector<int> vec(arr, arr + 2);
    auto head = vec_to_cyclic_list(vec, 0);
    Solution s;
    assert(s.hasCycle(head) == true);
}

void test_has_cycle3()
{
    int arr[] = {1};
    vector<int> vec(arr, arr + 1);
    auto head = vec_to_cyclic_list(vec, -1);
    Solution s;
    assert(s.hasCycle(head) == false);
}

void test_has_cycle4()
{
    int arr[] = {3, 2, 0, -4};
    vector<int> vec(arr, arr + 4);
    auto head = vec_to_cyclic_list(vec, 2);
    Solution s;
    assert(s.hasCycle2(head) == true);
}

void test_has_cycle5()
{
    int arr[] = {1, 2};
    vector<int> vec(arr, arr + 2);
    auto head = vec_to_cyclic_list(vec, 0);
    Solution s;
    assert(s.hasCycle2(head) == true);
}

void test_has_cycle6()
{
    int arr[] = {1};
    vector<int> vec(arr, arr + 1);
    auto head = vec_to_cyclic_list(vec, -1);
    Solution s;
    assert(s.hasCycle2(head) == false);
}

int main()
{
    test_has_cycle1();
    test_has_cycle2();
    test_has_cycle3();
    test_has_cycle4();
    test_has_cycle5();
    test_has_cycle6();
    return 0;
}