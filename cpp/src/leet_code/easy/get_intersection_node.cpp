// cpp/src/leet_code/easy/get_intersection_node.cpp

#include "get_intersection_node.hpp"
#include <iostream>
#include <vector>

using namespace std;

ListNode *Solution::getIntersectionNode(ListNode *headA, ListNode *headB)
{
    vector<ListNode *> stackA, stackB;
    auto *nodeA = headA;
    auto *nodeB = headB;
    while (nodeA != NULL)
    {
        stackA.push_back(nodeA);
        nodeA = nodeA->next;
    }
    while (nodeB != NULL)
    {
        stackB.push_back(nodeB);
        nodeB = nodeB->next;
    }
    ListNode *intersection = NULL;
    while (!stackA.empty() && !stackB.empty())
    {
        if (stackA.back() != stackB.back())
            break;
        // preserve a previous node.
        intersection = stackA.back();
        stackA.pop_back();
        stackB.pop_back();
    }
    return intersection;
}

// O(m+n) time and O(1) space solution
ListNode *Solution::getIntersectionNode2(ListNode *headA, ListNode *headB)
{
    ListNode *pA = headA;
    ListNode *pB = headB;

    while (pA != pB)
    {
        pA = pA ? pA->next : headB;
        pB = pB ? pB->next : headA;
    }
    return pA;
}
