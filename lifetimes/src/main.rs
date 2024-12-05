fn main() {
    let ans;
    let s1 = String::from("Small"); //Lifetime -> main function
    {
        let s2 = String::from("Longer"); //lifetime -> Only within {} this scope.
        //ans = longest(s1, s2); //valid as ownership being moved to the function.
        //ans = longest_with_reference(&s1, &s2); //This is invalid because lifetime of ans1 is the intersection of s1,s2 as the function is just borrowing and not owning the arguments; 
        ans = longest_with_lifetime(&s1, &s2); //gives better error as lifetimes are defined in the function signature.
    }
    println!("{}", ans);
}

fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        return a;
    }
    else {
        return b;
    }
}

fn longest_with_reference(a: &str, b: &str) -> &str { //lifetime is not specified => compilation error
    if a.len() > b.len() {
        return a;
    }
    else {
        return b;
    }
}

fn longest_with_lifetime<'a>(s1: &'a str, s2: &'a str) -> &'a str{ //lifetime specified with generics. => Better error
    if s1.len() > s2.len() {
        return s1;
    }
    else {
        return s2;
    }
}