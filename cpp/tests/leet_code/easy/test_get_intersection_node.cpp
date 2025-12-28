#include <cassert>
#include <vector>
#include "leet_code/common/list_node.hpp"
#include "leet_code/easy/get_intersection_node.hpp"

using namespace std;

void test_get_intersection_node1()
{
    vector<int> vecA = {4, 1, 8, 4, 5};
    auto headA = vec_to_list(vecA);
    vector<int> vecB = {5, 6, 1};
    auto headB = vec_to_list(vecB);

    // intersect at node with value 8 (index 2 in A)
    attach_at(headB, headA, 2);

    Solution s;
    auto inter = s.getIntersectionNode(headA, headB);
    assert(inter != NULL);
    assert(inter->val == 8);
}

void test_get_intersection_node2()
{
    vector<int> vecA = {1, 9, 1, 2, 4};
    auto headA = vec_to_list(vecA);
    vector<int> vecB = {3};
    auto headB = vec_to_list(vecB);

    // intersect at node with value 8 (index 3 in A)
    attach_at(headB, headA, 3);

    Solution s;
    auto inter = s.getIntersectionNode(headA, headB);
    assert(inter != NULL);
    assert(inter->val == 2);
}

void test_get_intersection_node3()
{
    vector<int> vecA = {2, 6, 4};
    auto headA = vec_to_list(vecA);
    vector<int> vecB = {1, 5};
    auto headB = vec_to_list(vecB);
    Solution s;
    auto inter = s.getIntersectionNode(headA, headB);
    assert(inter == NULL);
}

void test_get_intersection_node4()
{
    vector<int> vecA = {2, 2, 4, 5, 4};
    auto headA = vec_to_list(vecA);
    vector<int> vecB = {};
    auto headB = vec_to_list(vecB);

    // intersect at node with value 8 (index 3 in A)
    attach_at(headB, headA, 1);

    Solution s;
    auto inter = s.getIntersectionNode(headA, headB);
    assert(inter == NULL);
}

void test_get_intersection_node5()
{
    vector<int> vecA = {4, 1, 8, 4, 5};
    auto headA = vec_to_list(vecA);
    vector<int> vecB = {5, 6, 1};
    auto headB = vec_to_list(vecB);

    // intersect at node with value 8 (index 2 in A)
    attach_at(headB, headA, 2);

    Solution s;
    auto inter = s.getIntersectionNode2(headA, headB);
    assert(inter != NULL);
    assert(inter->val == 8);
}

void test_get_intersection_node6()
{
    vector<int> vecA = {1, 9, 1, 2, 4};
    auto headA = vec_to_list(vecA);
    vector<int> vecB = {3};
    auto headB = vec_to_list(vecB);

    // intersect at node with value 8 (index 3 in A)
    attach_at(headB, headA, 3);

    Solution s;
    auto inter = s.getIntersectionNode2(headA, headB);
    assert(inter != NULL);
    assert(inter->val == 2);
}

void test_get_intersection_node7()
{
    vector<int> vecA = {2, 6, 4};
    auto headA = vec_to_list(vecA);
    vector<int> vecB = {1, 5};
    auto headB = vec_to_list(vecB);
    Solution s;
    auto inter = s.getIntersectionNode2(headA, headB);
    assert(inter == NULL);
}

void test_get_intersection_node8()
{
    vector<int> vecA = {2, 2, 4, 5, 4};
    auto headA = vec_to_list(vecA);
    vector<int> vecB = {};
    auto headB = vec_to_list(vecB);

    // intersect at node with value 8 (index 3 in A)
    attach_at(headB, headA, 1);

    Solution s;
    auto inter = s.getIntersectionNode2(headA, headB);
    assert(inter == NULL);
}

void test_get_intersection_node9()
{
    vector<int> vecA = {2, 6, 4};
    auto headA = vec_to_list(vecA);
    vector<int> vecB = {1, 5, 3};
    auto headB = vec_to_list(vecB);
    Solution s;
    auto inter = s.getIntersectionNode2(headA, headB);
    assert(inter == NULL);
}

int main()
{
    test_get_intersection_node1();
    test_get_intersection_node2();
    test_get_intersection_node3();
    test_get_intersection_node4();
    test_get_intersection_node5();
    test_get_intersection_node6();
    test_get_intersection_node7();
    test_get_intersection_node8();
    test_get_intersection_node9();
    return 0;
}