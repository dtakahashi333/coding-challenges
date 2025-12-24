// cpp/src/leet_code/common/list_node.cpp

#include "list_node.hpp"

ListNode *vec_to_cyclic_list(const vector<int> &v, int pos)
{
    if (v.empty())
        return NULL;
    auto head = new ListNode(v[0]);
    auto cur = head;
    for (auto it = v.begin() + 1; it != v.end(); it++)
    {
        cur->next = new ListNode(*it);
        cur = cur->next;
    }
    if (pos >= 0)
    {
        auto node = head;
        for (int i = 0; i < pos; i++)
            node = node->next;
        cur->next = node;
    }
    return head;
}

ListNode *vec_to_list(const std::vector<int> &v)
{
    if (v.empty())
        return NULL;

    ListNode *head = new ListNode(v[0]);
    ListNode *cur = head;

    for (size_t i = 1; i < v.size(); i++)
    {
        cur->next = new ListNode(v[i]);
        cur = cur->next;
    }
    return head;
}

void attach_at(ListNode *headB, ListNode *headA, int pos)
{
    ListNode *curA = headA;
    for (int i = 0; i < pos; i++)
        curA = curA->next;

    if (!headB)
        headB = curA;
    else
    {
        ListNode *curB = headB;
        while (curB->next)
            curB = curB->next;
        curB->next = curA;
    }
}