// cpp/tests/leetcode/easy/test_get_intersection_node.cpp

#include <gtest/gtest.h>

#include <cassert>
#include <leetcode/common/list_node.hpp>
#include <leetcode/easy/get_intersection_node.hpp>
#include <vector>

using namespace std;

TEST(Test1, BasicAssertions) {
  vector<int> vecA = {4, 1, 8, 4, 5};
  auto* headA = vec_to_list(vecA);
  vector<int> vecB = {5, 6, 1};
  auto* headB = vec_to_list(vecB);

  // intersect at node with value 8 (index 2 in A)
  attach_at(headB, headA, 2);

  Solution s;
  auto* inter = s.getIntersectionNode(headA, headB);
  EXPECT_NE(inter, nullptr);
  EXPECT_EQ(inter->val, 8);
}

TEST(Test2, BasicAssertions) {
  vector<int> vecA = {1, 9, 1, 2, 4};
  auto* headA = vec_to_list(vecA);
  vector<int> vecB = {3};
  auto* headB = vec_to_list(vecB);

  // intersect at node with value 8 (index 3 in A)
  attach_at(headB, headA, 3);

  Solution s;
  auto* inter = s.getIntersectionNode(headA, headB);
  EXPECT_NE(inter, nullptr);
  EXPECT_EQ(inter->val, 2);
}

TEST(Test3, BasicAssertions) {
  vector<int> vecA = {2, 6, 4};
  auto* headA = vec_to_list(vecA);
  vector<int> vecB = {1, 5};
  auto* headB = vec_to_list(vecB);
  Solution s;
  auto* inter = s.getIntersectionNode(headA, headB);
  EXPECT_EQ(inter, nullptr);
}

TEST(Test4, BasicAssertions) {
  vector<int> vecA = {2, 2, 4, 5, 4};
  auto* headA = vec_to_list(vecA);
  vector<int> vecB = {};
  auto* headB = vec_to_list(vecB);

  // intersect at node with value 8 (index 3 in A)
  attach_at(headB, headA, 1);

  Solution s;
  auto* inter = s.getIntersectionNode(headA, headB);
  EXPECT_EQ(inter, nullptr);
}

TEST(Test5, BasicAssertions) {
  vector<int> vecA = {4, 1, 8, 4, 5};
  auto* headA = vec_to_list(vecA);
  vector<int> vecB = {5, 6, 1};
  auto* headB = vec_to_list(vecB);

  // intersect at node with value 8 (index 2 in A)
  attach_at(headB, headA, 2);

  Solution s;
  auto* inter = s.getIntersectionNode2(headA, headB);
  EXPECT_NE(inter, nullptr);
  EXPECT_EQ(inter->val, 8);
}

TEST(Test6, BasicAssertions) {
  vector<int> vecA = {1, 9, 1, 2, 4};
  auto* headA = vec_to_list(vecA);
  vector<int> vecB = {3};
  auto* headB = vec_to_list(vecB);

  // intersect at node with value 8 (index 3 in A)
  attach_at(headB, headA, 3);

  Solution s;
  auto* inter = s.getIntersectionNode2(headA, headB);
  EXPECT_NE(inter, nullptr);
  EXPECT_EQ(inter->val, 2);
}

TEST(Test7, BasicAssertions) {
  vector<int> vecA = {2, 6, 4};
  auto* headA = vec_to_list(vecA);
  vector<int> vecB = {1, 5};
  auto* headB = vec_to_list(vecB);
  Solution s;
  auto* inter = s.getIntersectionNode2(headA, headB);
  EXPECT_EQ(inter, nullptr);
}

TEST(Test8, BasicAssertions) {
  vector<int> vecA = {2, 2, 4, 5, 4};
  auto* headA = vec_to_list(vecA);
  vector<int> vecB = {};
  auto* headB = vec_to_list(vecB);

  // intersect at node with value 8 (index 3 in A)
  attach_at(headB, headA, 1);

  Solution s;
  auto* inter = s.getIntersectionNode2(headA, headB);
  EXPECT_EQ(inter, nullptr);
}

TEST(Test9, BasicAssertions) {
  vector<int> vecA = {2, 6, 4};
  auto* headA = vec_to_list(vecA);
  vector<int> vecB = {1, 5, 3};
  auto* headB = vec_to_list(vecB);
  Solution s;
  auto* inter = s.getIntersectionNode2(headA, headB);
  EXPECT_EQ(inter, nullptr);
}
