
fn calc(value: u64) -> u64 {
    println!("In: {}", value);
    let mut ret: u64 = 0;

// 2
    ret += (value / 2) + 1;

// 5
    for z in 0..(value / 5) {
        ret +=  (value - (5 * (z + 1))) / 2  + 1;
        //println!("after 5.{}: {}", i, ret);
    }

// 10
    for y in 0..(value / 10) {
        ret +=  (value - (10 * (y + 1))) / 2  + 1;
        let new_val_y: u64 = value - (10 * (y + 1));
        for z in 0..(new_val_y / 5) {
            ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
        }
    }
    
// 20
    for x in 0..(value / 20) {
        ret += (value - (20 * (x + 1))) / 2  + 1;
        let new_val_x: u64 = value - (20 * (x + 1));
        // 10
        for y in 0..(new_val_x / 10) {
            ret += (new_val_x - (10 * (y + 1))) / 2  + 1;
            let new_val_y: u64 = new_val_x - (10 * (y + 1));
            for z in 0..(new_val_y / 5) {
                ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
            }
        }// 5
        for z in 0..(new_val_x / 5) {
            ret +=  (new_val_x - (5 * (z + 1))) / 2  + 1;
        }
    }

// 50
    for w in 0..(value / 50) {
        ret +=  (value - (50 * (w + 1))) / 2  + 1;
        let new_val_w: u64 = value - (50 * (w + 1));
        // 20
        for x in 0..(new_val_w / 20) {
            ret += (new_val_w - (20 * (x + 1))) / 2  + 1;
            let new_val_x: u64 = new_val_w - (20 * (x + 1));
            // 10
            for y in 0..(new_val_x / 10) {
                ret += (new_val_x - (10 * (y + 1))) / 2  + 1;
                let new_val_y: u64 = new_val_x - (10 * (y + 1));
                for z in 0..(new_val_y / 5) {
                    ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                }
            }// 5
            for z in 0..(new_val_x / 5) {
                ret +=  (new_val_x - (5 * (z + 1))) / 2  + 1;
            }
        }
        for y in 0..(new_val_w / 10) {
            ret += (new_val_w - (10 * (y + 1))) / 2  + 1;
            let new_val_y: u64 = new_val_w - (10 * (y + 1));
            for z in 0..(new_val_y / 5) {
                ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
            }
        }// 5
        for z in 0..(new_val_w / 5) {
            ret +=  (new_val_w - (5 * (z + 1))) / 2  + 1;
        }
    }

// 100
    for v in 0..(value / 100) {
        ret +=  (value - (100 * (v + 1))) / 2  + 1;
        let new_val_v: u64 = value - (100 * (v + 1));

        for w in 0..(new_val_v / 50) {
            ret +=  (new_val_v - (50 * (w + 1))) / 2  + 1;
            let new_val_w: u64 = new_val_v - (50 * (w + 1));
            // 20
            for x in 0..(new_val_w / 20) {
                ret += (new_val_w - (20 * (x + 1))) / 2  + 1;
                let new_val_x: u64 = new_val_w - (20 * (x + 1));
                // 10
                for y in 0..(new_val_x / 10) {
                    ret += (new_val_x - (10 * (y + 1))) / 2  + 1;
                    let new_val_y: u64 = new_val_x - (10 * (y + 1));
                    for z in 0..(new_val_y / 5) {
                        ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                    }
                }// 5
                for z in 0..(new_val_x / 5) {
                    ret +=  (new_val_x - (5 * (z + 1))) / 2  + 1;
                }
            }
            for y in 0..(new_val_w / 10) {
                ret += (new_val_w - (10 * (y + 1))) / 2  + 1;
                let new_val_y: u64 = new_val_w - (10 * (y + 1));
                for z in 0..(new_val_y / 5) {
                    ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                }
            }// 5
            for z in 0..(new_val_w / 5) {
                ret +=  (new_val_w - (5 * (z + 1))) / 2  + 1;
            }
        }
        for x in 0..(new_val_v / 20) {
            ret += (new_val_v - (20 * (x + 1))) / 2  + 1;
            let new_val_x: u64 = new_val_v - (20 * (x + 1));
            // 10
            for y in 0..(new_val_x / 10) {
                ret += (new_val_x - (10 * (y + 1))) / 2  + 1;
                let new_val_y: u64 = new_val_x - (10 * (y + 1));
                for z in 0..(new_val_y / 5) {
                    ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                }
            }// 5
            for z in 0..(new_val_x / 5) {
                ret +=  (new_val_x - (5 * (z + 1))) / 2  + 1;
            }
        }
        for y in 0..(new_val_v / 10) {
            ret += (new_val_v - (10 * (y + 1))) / 2  + 1;
            let new_val_y: u64 = new_val_v - (10 * (y + 1));
            for z in 0..(new_val_y / 5) {
                ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
            }
        }// 5
        for z in 0..(new_val_v / 5) {
            ret +=  (new_val_v - (5 * (z + 1))) / 2  + 1;
        }
    }

// 200
    for u in 0..(value / 200) {
        ret +=  (value - (200 * (u + 1))) / 2  + 1;
        let new_val_u: u64 = value - (200 * (u + 1));

        for v in 0..(new_val_u / 100) {
            ret +=  (new_val_u - (100 * (v + 1))) / 2  + 1;
            let new_val_v: u64 = new_val_u - (100 * (v + 1));
    
            for w in 0..(new_val_v / 50) {
                ret +=  (new_val_v - (50 * (w + 1))) / 2  + 1;
                let new_val_w: u64 = new_val_v - (50 * (w + 1));
                // 20
                for x in 0..(new_val_w / 20) {
                    ret += (new_val_w - (20 * (x + 1))) / 2  + 1;
                    let new_val_x: u64 = new_val_w - (20 * (x + 1));
                    // 10
                    for y in 0..(new_val_x / 10) {
                        ret += (new_val_x - (10 * (y + 1))) / 2  + 1;
                        let new_val_y: u64 = new_val_x - (10 * (y + 1));
                        for z in 0..(new_val_y / 5) {
                            ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                        }
                    }// 5
                    for z in 0..(new_val_x / 5) {
                        ret +=  (new_val_x - (5 * (z + 1))) / 2  + 1;
                    }
                }
                for y in 0..(new_val_w / 10) {
                    ret += (new_val_w - (10 * (y + 1))) / 2  + 1;
                    let new_val_y: u64 = new_val_w - (10 * (y + 1));
                    for z in 0..(new_val_y / 5) {
                        ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                    }
                }// 5
                for z in 0..(new_val_w / 5) {
                    ret +=  (new_val_w - (5 * (z + 1))) / 2  + 1;
                }
            }
            for x in 0..(new_val_v / 20) {
                ret += (new_val_v - (20 * (x + 1))) / 2  + 1;
                let new_val_x: u64 = new_val_v - (20 * (x + 1));
                // 10
                for y in 0..(new_val_x / 10) {
                    ret += (new_val_x - (10 * (y + 1))) / 2  + 1;
                    let new_val_y: u64 = new_val_x - (10 * (y + 1));
                    for z in 0..(new_val_y / 5) {
                        ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                    }
                }// 5
                for z in 0..(new_val_x / 5) {
                    ret +=  (new_val_x - (5 * (z + 1))) / 2  + 1;
                }
            }
            for y in 0..(new_val_v / 10) {
                ret += (new_val_v - (10 * (y + 1))) / 2  + 1;
                let new_val_y: u64 = new_val_v - (10 * (y + 1));
                for z in 0..(new_val_y / 5) {
                    ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                }
            }
            for z in 0..(new_val_v / 5) {
                ret +=  (new_val_v - (5 * (z + 1))) / 2  + 1;
            }
        }
        for w in 0..(new_val_u / 50) {
            ret +=  (new_val_u - (50 * (w + 1))) / 2  + 1;
            let new_val_w: u64 = new_val_u - (50 * (w + 1));
            // 20
            for x in 0..(new_val_w / 20) {
                ret += (new_val_w - (20 * (x + 1))) / 2  + 1;
                let new_val_x: u64 = new_val_w - (20 * (x + 1));
                // 10
                for y in 0..(new_val_x / 10) {
                    ret += (new_val_x - (10 * (y + 1))) / 2  + 1;
                    let new_val_y: u64 = new_val_x - (10 * (y + 1));
                    for z in 0..(new_val_y / 5) {
                        ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                    }
                }// 5
                for z in 0..(new_val_x / 5) {
                    ret +=  (new_val_x - (5 * (z + 1))) / 2  + 1;
                }
            }
            for y in 0..(new_val_w / 10) {
                ret += (new_val_w - (10 * (y + 1))) / 2  + 1;
                let new_val_y: u64 = new_val_w - (10 * (y + 1));
                for z in 0..(new_val_y / 5) {
                    ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                }
            }// 5
            for z in 0..(new_val_w / 5) {
                ret +=  (new_val_w - (5 * (z + 1))) / 2  + 1;
            }
        }
        for x in 0..(new_val_u / 20) {
            ret += (new_val_u - (20 * (x + 1))) / 2  + 1;
            let new_val_x: u64 = new_val_u - (20 * (x + 1));
            // 10
            for y in 0..(new_val_x / 10) {
                ret += (new_val_x - (10 * (y + 1))) / 2  + 1;
                let new_val_y: u64 = new_val_x - (10 * (y + 1));
                for z in 0..(new_val_y / 5) {
                    ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
                }
            }// 5
            for z in 0..(new_val_u / 5) {
                ret +=  (new_val_u - (5 * (z + 1))) / 2  + 1;
            }
        }
        for y in 0..(new_val_u / 10) {
            ret += (new_val_u - (10 * (y + 1))) / 2  + 1;
            let new_val_y: u64 = new_val_u - (10 * (y + 1));
            for z in 0..(new_val_y / 5) {
                ret +=  (new_val_y - (5 * (z + 1))) / 2  + 1;
            }
        }// 5
        for z in 0..(new_val_u / 5) {
            ret +=  (new_val_u - (5 * (z + 1))) / 2  + 1;
        }
    }

    ret 
}

pub fn main() {
    println!("Out: {}", calc(200));
}