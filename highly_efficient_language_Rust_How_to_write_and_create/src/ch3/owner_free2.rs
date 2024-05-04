fn main() {
    // ブロック1
    {
        let s1 = String::from("真実はワインの中にある");
        let s3 = String::from("ブドウ畑と美人は手がかかる");
        // ブロック2
        {
            let s2 = s1;
            println!("{}", s2);
        }
        // s2, 及びs2が所有権を持つ値はここで破棄される
        println!("{}", s3);
    }
    // s3, 及びs3が所有権を持つ値はここで破棄される
}