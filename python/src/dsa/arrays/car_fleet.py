#!/usr/bin/env python3

# python/src/dsa/arrays/car_fleet.py

# 853. Car Fleet
# https://leetcode.com/problems/car-fleet/description/

from typing import List


class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        num_of_cars = len(position)
        cars = []
        for i in range(num_of_cars):
            arrival_time = (target - position[i]) / speed[i]
            cars.append((position[i], arrival_time))  # (position, arrival_time)

        cars.sort(key=lambda x: x[0], reverse=True)

        num_of_fleets = 1
        current_fleet_time = cars[0][1]
        for i in range(1, num_of_cars, 1):
            if cars[i][1] > current_fleet_time:
                num_of_fleets += 1
                current_fleet_time = cars[i][1]

        return num_of_fleets
