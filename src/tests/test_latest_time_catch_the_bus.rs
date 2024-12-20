/**
给你一个下标从 0 开始长度为 n 的整数数组 buses ，其中 buses[i] 表示第 i 辆公交车的出发时间。同时给你一个下标从 0 开始长度为 m 的整数数组 passengers ，
其中 passengers[j] 表示第 j 位乘客的到达时间。所有公交车出发的时间互不相同，所有乘客到达的时间也互不相同。
给你一个整数 capacity ，表示每辆公交车 最多 能容纳的乘客数目。
每位乘客都会搭乘下一辆有座位的公交车。如果你在 y 时刻到达，公交在 x 时刻出发，满足 y <= x  且公交没有满，那么你可以搭乘这一辆公交。最早 到达的乘客优先上车。
返回你可以搭乘公交车的最晚到达公交站时间。你 不能 跟别的乘客同时刻到达。
注意：数组 buses 和 passengers 不一定是有序的。
 **/
#[test]
pub fn test_latest_time_catch_the_bus() {
    let buses = vec![20, 30, 10];
    let passengers = vec![19, 13, 26, 4, 25, 11, 21];
    let res = latest_time_catch_the_bus(buses, passengers, 2);
    println!("result: {}", res)
}

fn latest_time_catch_the_bus(mut buses: Vec<i32>, mut passengers: Vec<i32>, capacity: i32) -> i32 {
    buses.sort_unstable();
    passengers.sort_unstable();

    // 模拟乘客上车
    let mut j = 0;
    let mut c = 0;

    for &bus in &buses {
        c = capacity;
        while c > 0 && j < passengers.len() && passengers[j] <= bus {
            j += 1;
            c -= 1;
        }
    }

    j -= 1;
    let mut res = if c > 0 { *buses.last().unwrap() } else {  passengers[j] };
    while j < passengers.len() && res == passengers[j] {
        res -= 1;
        j -= 1;
    }
    res
}
