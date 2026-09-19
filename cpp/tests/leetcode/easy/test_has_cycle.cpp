// cpp/tests/leetcode/easy/test_has_cycle.cpp

#include <gtest/gtest.h>

#include <cassert>
#include <leetcode/common/list_node.hpp>
#include <leetcode/easy/has_cycle.hpp>
#include <vector>

using namespace std;

TEST(Test1, BasicAssertions) {
  int arr[] = {3, 2, 0, -4};
  vector<int> vec(arr, arr + 4);
  auto* head = vec_to_cyclic_list(vec, 2);
  Solution s;
  EXPECT_EQ(s.hasCycle(head), true);
}

TEST(Test2, BasicAssertions) {
  int arr[] = {1, 2};
  vector<int> vec(arr, arr + 2);
  auto* head = vec_to_cyclic_list(vec, 0);
  Solution s;
  EXPECT_EQ(s.hasCycle(head), true);
}

TEST(Test3, BasicAssertions) {
  int arr[] = {1};
  vector<int> vec(arr, arr + 1);
  auto* head = vec_to_cyclic_list(vec, -1);
  Solution s;
  EXPECT_EQ(s.hasCycle(head), false);
}

TEST(Test4, BasicAssertions) {
  int arr[] = {3, 2, 0, -4};
  vector<int> vec(arr, arr + 4);
  auto* head = vec_to_cyclic_list(vec, 2);
  Solution s;
  EXPECT_EQ(s.hasCycle2(head), true);
}

TEST(Test5, BasicAssertions) {
  int arr[] = {1, 2};
  vector<int> vec(arr, arr + 2);
  auto* head = vec_to_cyclic_list(vec, 0);
  Solution s;
  EXPECT_EQ(s.hasCycle2(head), true);
}

TEST(Test6, BasicAssertions) {
  int arr[] = {1};
  vector<int> vec(arr, arr + 1);
  auto* head = vec_to_cyclic_list(vec, -1);
  Solution s;
  EXPECT_EQ(s.hasCycle2(head), false);
}
