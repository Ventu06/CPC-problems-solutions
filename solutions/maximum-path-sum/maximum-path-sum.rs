// https://practice.geeksforgeeks.org/problems/maximum-path-sum/1

use std::io;

struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn my_solve(t: Node) -> (i32, Option<i32>) {

    match (t.left, t.right) {
        (None, None) => (t.data, None),
        (Some(t_l), None) => {
            let (x_l, y_l) = my_solve(*t_l);
            (t.data + x_l, y_l)
        },
        (None, Some(t_r)) => {
            let (x_r, y_r) = my_solve(*t_r);
            (t.data + x_r, y_r)
        },
        (Some(t_l), Some(t_r)) => {
            let (x_l, y_l) = my_solve(*t_l);
            let (x_r, y_r) = my_solve(*t_r);
            let mut y = x_l + x_r + t.data;
            if let Some(y_l) = y_l {
                y = y.max(y_l);
            }
            if let Some(y_r) = y_r {
                y = y.max(y_r);
            }
            (t.data + x_l.max(x_r), Some(y))
        },
    }

}

fn solve(t: Node) -> i32 {
    match my_solve(t).1 {
        Some(x) => x,
        None => 0
    }
}

fn main() {

    let t_1 = Node { data: 3
            , left: Some(Box::new(Node { data: 4
                , left: Some(Box::new(Node { data: -10
                    , left: None
                    , right: None
                }))
                , right: Some(Box::new(Node { data: 4
                    , left: None
                    , right: None
                }))
            }))
            , right: Some(Box::new(Node { data: 5
                , left: None
                , right: None
            }))
        };

    let t_2 = Node { data: -15
            , left: Some(Box::new(Node { data: 5
                , left: Some(Box::new(Node { data: -8
                    , left: Some(Box::new(Node { data: 2
                        , left: None
                        , right: None
                    }))
                    , right: Some(Box::new(Node { data: -3
                        , left: None
                        , right: None
                    }))
                }))
                , right: Some(Box::new(Node { data: 1
                    , left: None
                    , right: None
                }))
            }))
            , right: Some(Box::new(Node { data: 6
                , left: Some(Box::new(Node { data: 3
                    , left: None
                    , right: None
                }))
                , right: Some(Box::new(Node { data: 9
                    , left: None
                    , right: Some(Box::new(Node { data: 0
                        , left: Some(Box::new(Node { data: 4
                            , left: None
                            , right: None
                        }))
                        , right: Some(Box::new(Node { data: -1
                            , left: Some(Box::new(Node { data: 10
                                , left: None
                                , right: None
                            }))
                            , right: None
                        }))
                    }))
                }))
            }))
        };

    println!("{}", solve(t_1));
    println!("{}", solve(t_2));

}
