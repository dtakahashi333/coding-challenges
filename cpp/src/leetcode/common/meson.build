leetcode_common_sources = files(
    'list_node.cpp',
)

leetcode_common_lib = library(
    'leetcode_common',
    sources : leetcode_common_sources,
    include_directories : inc_dir,
)

leetcode_common_dep = declare_dependency(
    include_directories : inc_dir,
    link_with : leetcode_common_lib,
)
