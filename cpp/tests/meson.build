# 1. Fetch GTest dependency (falls back to subproject if not on system)
gtest_dep = dependency(
    'gtest_main',
    fallback : ['gtest', 'gtest_main_dep']
)

# the below is the same as without dependencies:
# executable(
#   'test_has_cycle',
#   'leetcode/easy/test_has_cycle.cpp',
#   link_with: leetcode_easy_lib,
#   include_directories: inc,
#   dependencies: gtest_dep,
# )
test_has_cycle_exe = executable(
  'test_has_cycle',
  ['leetcode/easy/test_has_cycle.cpp',],
  dependencies : [
    gtest_dep,
    leetcode_easy_dep,
  ],
)

test_get_intersection_node_exe = executable(
  'test_get_intersection_node',
  ['leetcode/easy/test_get_intersection_node.cpp',],
  dependencies : [
    gtest_dep,
    leetcode_easy_dep,
  ],
)

test('Linked List Cycle tests', test_has_cycle_exe)
test('Intersection of Two Linked Lists tests', test_get_intersection_node_exe)
