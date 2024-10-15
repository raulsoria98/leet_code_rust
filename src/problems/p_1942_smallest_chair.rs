#![allow(dead_code)]

/*

There is a party where n friends numbered from 0 to n - 1 are attending. There is an infinite number of chairs in this
party that are numbered from 0 to infinity. When a friend arrives at the party, they sit on the unoccupied chair with
the smallest number.

    For example, if chairs 0, 1, and 5 are occupied when a friend comes, they will sit on chair number 2.

When a friend leaves the party, their chair becomes unoccupied at the moment they leave. If another friend arrives at
that same moment, they can sit in that chair.

You are given a 0-indexed 2D integer array times where times[i] = [arrivali, leavingi], indicating the arrival and
leaving times of the i friend respectively, and an integer targetFriend. All arrival times are distinct.

Return the chair number that the friend numbered targetFriend will sit on.

*/

pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let mut order: Vec<usize> = (0..times.len()).collect();

    order.sort_by_key(|&i| times[i][0]);

    let mut empty_seats: BinaryHeap<Reverse<i32>> = (0..times.len() as i32).map(Reverse).collect();
    let mut taken_seats: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new(); // (leaving, seat)

    for (friend, arrival, leaving) in order
        .into_iter()
        .map(|friend| (friend, times[friend][0], times[friend][1]))
    {
        while let Some(&Reverse((lv, seat))) = taken_seats.peek() {
            if lv <= arrival {
                empty_seats.push(Reverse(seat));
                taken_seats.pop();
            } else {
                break;
            }
        }

        let seat = empty_seats.pop().unwrap().0;

        if friend as i32 == target_friend {
            return seat;
        }

        taken_seats.push(Reverse((leaving, seat)));
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let times = vec![vec![1, 4], vec![2, 3], vec![4, 6]];
        let target_friend = 1;
        assert_eq!(smallest_chair(times, target_friend), 1);
    }

    #[test]
    fn test_example_2() {
        let times = vec![vec![3, 10], vec![1, 5], vec![2, 6]];
        let target_friend = 0;
        assert_eq!(smallest_chair(times, target_friend), 2);
    }

    #[test]
    fn test_single_friend() {
        let times = vec![vec![5, 10]];
        let target_friend = 0;
        assert_eq!(smallest_chair(times, target_friend), 0);
    }

    #[test]
    fn test_friends_arriving_same_time() {
        let times = vec![vec![1, 5], vec![1, 6], vec![1, 7]];
        let target_friend = 2;
        assert_eq!(smallest_chair(times, target_friend), 2);
    }

    #[test]
    fn test_friends_leaving_and_reusing_chair() {
        let times = vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7]];
        let target_friend = 3;
        assert_eq!(smallest_chair(times, target_friend), 1);
    }

    #[test]
    fn test_large_input() {
        let times = (0..1000)
            .map(|i| vec![i as i32, (i as i32) + 10])
            .collect::<Vec<_>>();
        let target_friend = 999;
        assert_eq!(smallest_chair(times, target_friend), 9);
    }

    #[test]
    fn test_multiple_friends_reusing_same_chair() {
        let times = vec![vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9]];
        let target_friend = 4;
        assert_eq!(smallest_chair(times, target_friend), 0);
    }

    #[test]
    fn test_all_friends_leave_before_others_arrive() {
        let times = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];
        let target_friend = 4;
        assert_eq!(smallest_chair(times, target_friend), 0);
    }
}
