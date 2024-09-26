pub fn vcopy_clone() {
    let s1 = String::from("i am string");
    let s2 = s1;
    //println!("this is ok: {}", s1);  s1移动到s2, 已被释放了 不能打印
    let s3 = s2.clone();
    println!("this is :{}", s3);
    println!("this is:{}", s2);
}

pub fn vownershipfunc(x:String) {
    println!("x:{}", x)
}

pub fn vreference(x: String) -> usize {
    let r1 = x;
    println!("r1:{}", r1);
    x.len()
}

pub fn vslice1(s: &String) -> usize {
    let bytes  = s.as_bytes();//将字符转换为字节数组
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  i;
        }
    }

    
    s.len()
}

pub fn vslice2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  &s[..i];
        }
    }
    &s[..]
}