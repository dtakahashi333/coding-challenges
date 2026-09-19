// cpp/tests/leetcode/easy/test_two_sum.cpp

#include <gtest/gtest.h>

#include <algorithm>
#include <cassert>
#include <leetcode/easy/two_sum.hpp>
#include <vector>

using namespace std;

TEST(Test1, BasicAssertions) {
  vector<int> nums = {2, 7, 11, 15};
  int target = 9;
  Solution s;
  vector<int> actual = s.twoSum(nums, target);
  sort(actual.begin(), actual.end());
  EXPECT_EQ(actual, vector<int>({0, 1}));
}

TEST(Test2, BasicAssertions) {
  vector<int> nums = {3, 2, 4};
  int target = 6;
  Solution s;
  vector<int> actual = s.twoSum(nums, target);
  sort(actual.begin(), actual.end());
  EXPECT_EQ(actual, vector<int>({1, 2}));
}

TEST(Test3, BasicAssertions) {
  vector<int> nums = {3, 3};
  int target = 6;
  Solution s;
  vector<int> actual = s.twoSum(nums, target);
  sort(actual.begin(), actual.end());
  EXPECT_EQ(actual, vector<int>({0, 1}));
}