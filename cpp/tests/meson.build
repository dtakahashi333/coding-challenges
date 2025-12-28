# 1. Fetch GTest dependency (falls back to subproject if not on system)
gtest_dep = dependency(
    'gtest_main',
    fallback : ['gtest', 'gtest_main_dep']
)

test_has_cycle_exe = executable(
  'test_has_cycle',
  ['leet_code/easy/test_has_cycle.cpp',],
  dependencies : [
    gtest_dep,
    leet_code_easy_dep,
  ],
)

test_get_intersection_node_exe = executable(
  'test_get_intersection_node',
  ['leet_code/easy/test_get_intersection_node.cpp',],
  dependencies : [
    gtest_dep,
    leet_code_easy_dep,
  ],
)

test('Linked List Cycle tests', test_has_cycle_exe)
test('Intersection of Two Linked Lists tests', test_get_intersection_node_exe)
