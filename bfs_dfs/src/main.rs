// bfs 示例：找出二维数组中最大面积的岛屿
fn bfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    if i < 0 || i >= m || j < 0 || j >= n || grid[i as usize][j as usize] == 0 {
        return 0;
    }
    grid[i as usize][j as usize] = 0;
    return 1
        + bfs(grid, i - 1, j)  // 上
        + bfs(grid, i + 1, j) // 下
        + bfs(grid, i, j - 1) // 左
        + bfs(grid, i, j + 1); // 右
}

// dfs 示例：所有可能的路径
fn dfs(graph: &Vec<Vec<i32>>, stk: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, i: usize) {
    if i == graph.len() - 1 {
        ans.push(stk.to_vec());
        return;
    }
    for &n in &graph[i] {
        stk.push(n);
        dfs(graph, stk, ans, n as usize);
        stk.pop();
    }
}

fn main() {
    // bfs
    let mut res = 0;
    let mut grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            res = res.max(bfs(&mut grid, i as i32, j as i32));
        }
    }
    println!("最大面积: {}", res);

    // dfs
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    let mut ans = vec![];
    dfs(&graph, &mut vec![0], &mut ans, 0);
    println!("路径：{:?}", ans);
}
