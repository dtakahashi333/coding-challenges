// cpp/include/leetcode/common/has_cycle.hpp

// 141. Linked List Cycle
// https://leetcode.com/problems/linked-list-cycle/description/

#pragma once

#include <leetcode/common/list_node.hpp>

class Solution {
 public:
  bool hasCycle(ListNode* head);
  bool hasCycle2(ListNode* head);
};