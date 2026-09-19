leetcode_easy_sources = files(
    'has_cycle.cpp',
    'get_intersection_node.cpp',
    'two_sum.cpp',
)

leetcode_easy_lib = library(
    'leetcode_easy',
    sources : leetcode_easy_sources,
    include_directories : inc_dir,
)

leetcode_easy_dep = declare_dependency(
    include_directories : inc_dir,
    link_with : leetcode_easy_lib,
        dependencies: [
        leetcode_common_dep,
    ]
)
