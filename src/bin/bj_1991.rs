use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut tree = vec![(' ', ' '); n];

    for _ in 0..n {
        let (parent, left, right) = (
            iter.next().unwrap().as_bytes()[0],
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );
        tree[(parent - b'A') as usize] = (left, right);
    }

    let mut preorder = String::new();
    let mut inorder = String::new();
    let mut postorder = String::new();

    get_preorder(&tree, 'A', &mut preorder);
    get_inorder(&tree, 'A', &mut inorder);
    get_postorder(&tree, 'A', &mut postorder);

    println!("{}", preorder);
    println!("{}", inorder);
    println!("{}", postorder);
}

fn get_preorder(tree: &[(char, char)], root: char, preorder: &mut String) {
    if root == '.' {
        return;
    }
    preorder.push(root);
    get_preorder(tree, tree[(root as u8 - b'A') as usize].0, preorder);
    get_preorder(tree, tree[(root as u8 - b'A') as usize].1, preorder);
}

fn get_inorder(tree: &[(char, char)], root: char, inorder: &mut String) {
    if root == '.' {
        return;
    }
    get_inorder(tree, tree[(root as u8 - b'A') as usize].0, inorder);
    inorder.push(root);
    get_inorder(tree, tree[(root as u8 - b'A') as usize].1, inorder);
}

fn get_postorder(tree: &[(char, char)], root: char, postorder: &mut String) {
    if root == '.' {
        return;
    }
    get_postorder(tree, tree[(root as u8 - b'A') as usize].0, postorder);
    get_postorder(tree, tree[(root as u8 - b'A') as usize].1, postorder);
    postorder.push(root);
}
