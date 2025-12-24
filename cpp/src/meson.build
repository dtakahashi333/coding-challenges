inc = include_directories('.')

leet_code_easy_lib = library(
    'leet_code',
    sources : files(
        'leet_code/common/list_node.cpp',
        'leet_code/easy/has_cycle.cpp',
    ),
    include_directories : inc,
)

leet_code_easy_dep = declare_dependency(
    include_directories : inc,
    link_with : leet_code_easy_lib,
)
