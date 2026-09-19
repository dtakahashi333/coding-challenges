// cpp/src/leetcode/common/has_cycle.cpp

#include <leetcode/easy/has_cycle.hpp>
#include <iostream>
#include <unordered_set>

using namespace std;

bool Solution::hasCycle(ListNode *head)
{
    auto *node = head;
    unordered_set<ListNode *> visited;
    while (node != NULL)
    {
        if (visited.contains(node))
        {
            return true;
        }
        visited.insert(node);
        node = node->next;
    }
    return false;
}

bool Solution::hasCycle2(ListNode *head)
{
    auto *fast = head;
    auto *slow = head;
    while (fast != NULL && fast->next != NULL)
    {
        fast = fast->next->next;
        slow = slow->next;
        if (fast == slow)
        {
            return true;
        }
    }
    return false;
}