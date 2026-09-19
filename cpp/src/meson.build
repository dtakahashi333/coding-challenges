inc = include_directories('../include')

leetcode_easy_lib = library(
    'leetcode_easy',
    sources : files(
        'leetcode/common/list_node.cpp',
        'leetcode/easy/has_cycle.cpp',
        'leetcode/easy/get_intersection_node.cpp',
    ),
    include_directories : inc,
)

leetcode_easy_dep = declare_dependency(
    include_directories : inc,
    link_with : leetcode_easy_lib,
)
