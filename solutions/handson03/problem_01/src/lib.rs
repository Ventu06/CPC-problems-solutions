pub fn holiday_planning(attr: &[Vec<u32>]) -> u32 {
    let d = attr[0].len();

    let mut dp: Vec<u32> = vec![0; d + 1];

    for a in attr {
        let mut dpn = dp.clone();
        let mut sum = 0;
        for i in 0..d {
            sum += a[i];
            for j in i + 1..d + 1 {
                dpn[j] = dpn[j].max(dp[j - i - 1] + sum);
            }
        }
        dp = dpn;
    }

    *dp.iter().last().unwrap()
}
