// cpp/src/leet_code/easy/get_intersection_node.hpp

// 160. Intersection of Two Linked Lists
// https://leetcode.com/problems/intersection-of-two-linked-lists/description/

#pragma once

#include "../common/list_node.hpp"

class Solution
{
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB);
    ListNode *getIntersectionNode2(ListNode *headA, ListNode *headB);
};