// https://leetcode-cn.com/problems/satisfiability-of-equality-equations/comments/
// 所谓并查集，本质是上树啊
fn union(roots: &mut Vec<usize>, node1: usize, node2: usize) {
    let root1 = findroot(roots, node1);
    let root2 = findroot(roots, node2);
    roots[root1] = root2;
}

fn findroot(roots: &Vec<usize>, node: usize) -> usize {
    if roots[node] == 99 || roots[node] == node {
        return node;
    } else {
        return findroot(roots, roots[node]);
    }
}

pub fn equations_possible(equations: Vec<String>) -> bool {
    let mut roots = vec![99; 26]; // 99 as guard

    // 处理等号，形成连通域
    let base = b'a';
    for s in &equations {
        let bytes = s.as_bytes();
        if bytes[1] as char == '=' {
            let c1 = bytes[0];
            let c2 = bytes[3];
            let n1 = (c1 - base) as usize;
            let n2 = (c2 - base) as usize;
            union(&mut roots, n1, n2);
        }
    }
    
    // 处理不等号
    for s in &equations {
        let bytes = s.as_bytes();
        if bytes[1] as char == '!' {
            let c1 = bytes[0];
            let c2 = bytes[3];
            let n1 = (c1 - base) as usize;
            let n2 = (c2 - base) as usize;
            let r1 = findroot(&roots, n1);
            let r2 = findroot(&roots, n2);
            if r1 == r2 { return false; }
        }
    }
    return true;
}

fn main()
{
    let eqs = [
        "b==b".to_string(),
        "b==e".to_string(),
        "e==c".to_string(),
        "d!=e".to_string(),
    ].to_vec();
    let r = equations_possible(eqs);
    println!("{}", r);
}