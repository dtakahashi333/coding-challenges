// cpp/src/leet_code/common/list_node.cpp

#include "list_node.hpp"

ListNode *vec_to_cyclic_list(const vector<int> &v, int pos)
{
    if (v.empty())
        return NULL;
    auto dummy = new ListNode(0);
    auto current = dummy;
    auto it = v.begin();
    for (; it != v.end(); it++)
    {
        current->next = new ListNode(*it);
        current = current->next;
    }
    auto head = dummy->next;
    delete dummy;
    if (pos >= 0)
    {
        auto node = head;
        for (int i = 0; i < pos; i++)
        {
            node = node->next;
        }
        current->next = node;
    }
    return head;
}