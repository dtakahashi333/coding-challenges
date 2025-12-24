// cpp/src/leet_code/common/list_node.hpp

#pragma once

#include <vector>

using namespace std;

// Definition for singly-linked list.
struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

ListNode *vec_to_cyclic_list(const vector<int> &v, int pos = -1);
ListNode *vec_to_list(const std::vector<int> &v);
void attach_at(ListNode *headB, ListNode *headA, int pos);