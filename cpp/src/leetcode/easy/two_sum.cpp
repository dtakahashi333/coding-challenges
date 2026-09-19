// cpp/src/leetcode/easy/two_sum.cpp

#include <leetcode/easy/two_sum.hpp>
#include <map>

vector<int> Solution::twoSum(vector<int>& nums, int target) {
  map<int, int> hash_map;

  for (int i = 0; i < static_cast<int>(nums.size()); i++) {
    if (hash_map.contains(nums[i])) {
      return {hash_map[nums[i]], i};
    }
    hash_map[target - nums[i]] = i;
  }

  return {};
}
