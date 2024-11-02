# Two Sum Solution

This project implements a solution to the "Two Sum" problem, where the objective is to find indices of two numbers in an array that add up to a specific target.

## Problem Description (Technical Specification)

Given an array of integers `nums` and an integer `target`, return the indices of the two numbers such that they add up to `target`.

- Each input will have exactly one solution.
- You may not use the same element twice.
- The answer can be returned in any order.

### Examples

#### Example 1
- **Input**: `nums = [2, 7, 11, 15]`, `target = 9`
- **Output**: `[0, 1]`
- **Explanation**: `nums[0] + nums[1] = 9`, so the function returns `[0, 1]`.

#### Example 2
- **Input**: `nums = [3, 2, 4]`, `target = 6`
- **Output**: `[1, 2]`
- **Explanation**: `nums[1] + nums[2] = 6`, so the function returns `[1, 2]`.

#### Example 3
- **Input**: `nums = [3, 3]`, `target = 6`
- **Output**: `[0, 1]`
- **Explanation**: `nums[0] + nums[1] = 6`, so the function returns `[0, 1]`.

### Constraints
- `2 <= nums.length <= 10^4`
- `-10^9 <= nums[i] <= 10^9`
- `-10^9 <= target <= 10^9`
- Only one valid answer exists.

## Setup and Usage

This project requires Rust to compile and run the code.

### Prerequisites
- **Rust**: Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already.

### Running the Project

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/two-sum-rust.git
   cd two-sum-rust

2.Build and run the project using Cargo:
   ```bash
cargo run
