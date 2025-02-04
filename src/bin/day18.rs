use std::fmt::{Display, Formatter};
use uuid::Uuid;

macro_rules! is_of_var {
    ($val:ident, $var:path) => {
        match $val {
            $var { .. } => true,
            _ => false,
        }
    };
}

#[derive(Debug, Clone)]
struct List {
    items: Vec<Element>,
    uuid: Uuid,
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.items.len() == 2 {
            write!(f, "{},", self.items.first().unwrap());
            write!(f, "{}", self.items.last().unwrap());
        } else if self.items.len() == 1 {
            write!(f, "{}", self.items.first().unwrap());
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Element {
    Integer(u32),
    List(Box<List>),
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Integer(ref value) => {
                write!(f, "{}", value)
            }
            Element::List(ref list) => {
                write!(f, "[{}]", list)
            }
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        match (self, other) {
            (&Element::Integer(ref a), &Element::Integer(ref b)) => a == b,
            (&Element::List(ref a), &Element::List(ref b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        self.uuid == other.uuid
    }
}

impl List {
    fn new() -> Self {
        List {
            items: vec![],
            uuid: Uuid::new_v4(),
        }
    }

    fn new_with_values(left: u32, right: u32) -> Self {
        List {
            items: vec![Element::Integer(left), Element::Integer(right)],
            uuid: Uuid::new_v4(),
        }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn left(&self) -> &Element {
        &self.items[0]
    }

    fn right(&self) -> &Element {
        &self.items[1]
    }
    fn right_list(&self) -> Option<&List> {
        if self.items.len() == 1 {
            return None;
        }
        let e = &self.items[1];
        match e {
            Element::Integer(_) => None,
            Element::List(ref list) => Some(list),
        }
    }

    fn right_value(&self) -> Option<u32> {
        if self.items.len() == 1 {
            return None;
        }
        let e = &self.items[1];
        match e {
            Element::Integer(v) => Some(*v),
            _ => None,
        }
    }

    fn left_list(&self) -> Option<&List> {
        if self.items.len() == 1 {
            return None;
        }
        let e = &self.items[0];
        match e {
            Element::Integer(_) => None,
            Element::List(ref list) => Some(list),
        }
    }

    fn left_value(&self) -> Option<u32> {
        if self.items.len() == 1 {
            return None;
        }
        let e = &self.items[0];
        match e {
            Element::Integer(v) => Some(*v),
            _ => None,
        }
    }

    fn set_left_value(&mut self, value: u32) {
        let e = &mut self.items[0];
        match e {
            Element::Integer(ref mut v) => *v = value,
            Element::List(_) => self.items[0] = Element::Integer(value),
        }
    }

    fn set_right_value(&mut self, value: u32) {
        let e = &mut self.items[1];
        match e {
            Element::Integer(ref mut v) => *v = value,
            Element::List(_) => self.items[1] = Element::Integer(value),
        }
    }

    fn set_left_list(&mut self, value: List) {
        self.items[0] = Element::List(Box::new(value))
    }

    fn set_right_list(&mut self, value: List) {
        self.items[1] = Element::List(Box::new(value))
    }

    fn find(&self, uuid: Uuid) -> Option<&List> {
        if self.uuid == uuid {
            return Some(&self);
        }
        for element in self.items.iter() {
            match element {
                Element::List(ref list) => {
                    if let Some(l) = list.find(uuid) {
                        return Some(l);
                    }
                }
                _ => {}
            }
        }
        None
    }
    fn find_mut(&mut self, uuid: Uuid) -> Option<&mut List> {
        if self.uuid == uuid {
            return Some(self);
        }
        for element in self.items.iter_mut() {
            match element {
                Element::List(ref mut list) => {
                    if let Some(l) = list.find_mut(uuid) {
                        return Some(l);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn parent(&self, uuid: Uuid) -> Option<&List> {
        for element in self.items.iter() {
            match element {
                Element::List(ref list) => {
                    if list.uuid == uuid {
                        return Some(&self);
                    }
                    if let Some(l) = list.parent(uuid) {
                        return Some(l);
                    }
                }
                _ => {}
            }
        }
        None
    }

    // fn replace(&mut self, uuid: Uuid, list: &List) {
    //     let binding = self.find_mut(uuid);
    //     let mut p = binding.unwrap();
    //     println!("replace {} with {}", p, list);
    //     p.items.clear();
    //     p.items.append(&mut list.items.clone());
    // }

    fn bottom_right(&self) -> &Element {
        let left = self.items.last().unwrap();
        match left {
            Element::List(ref list) => list.bottom_right(),
            Element::Integer(ref element) => left,
        }
    }

    fn is_list_pair(&self) -> bool {
        if self.items.len() == 1 {
            return false;
        }
        let l = self.items.get(0).unwrap();
        let r = self.items.get(1).unwrap();

        !is_of_var!(l, Element::Integer) && !is_of_var!(r, Element::Integer)
    }

    fn is_integer_pair(&self) -> bool {
        if self.items.len() == 1 {
            return false;
        }
        let l = self.items.get(0).unwrap();
        let r = self.items.get(1).unwrap();

        is_of_var!(l, Element::Integer) && is_of_var!(r, Element::Integer)
    }

    fn is_integer_left(&self) -> bool {
        if self.items.len() == 1 {
            return false;
        }
        let l = self.items.get(0).unwrap();
        let r = self.items.get(1).unwrap();

        is_of_var!(l, Element::Integer) && !is_of_var!(r, Element::Integer)
    }

    fn is_integer_right(&self) -> bool {
        if self.items.len() == 1 {
            return false;
        }
        let l = self.items.get(0).unwrap();
        let r = self.items.get(1).unwrap();

        !is_of_var!(l, Element::Integer) && is_of_var!(r, Element::Integer)
    }

    fn is_list_right(&self) -> bool {
        if self.items.len() == 1 {
            return false;
        }
        let r = self.items.get(1).unwrap();

        is_of_var!(r, Element::List)
    }

    fn is_list_left(&self) -> bool {
        if self.items.len() == 1 {
            return false;
        }
        let l = self.items.get(0).unwrap();

        is_of_var!(l, Element::List)
    }

    fn get_first_pair(&self, level: usize) -> Option<Uuid> {
        for element in self.items.iter() {
            match element {
                Element::List(ref list) => {
                    if list.is_integer_pair() && level == 4 {
                        return Some(list.uuid);
                    }
                    if let Some(pair) = list.get_first_pair(level + 1) {
                        return Some(pair);
                    }
                }
                _ => {}
            }
        }
        None
    }

    // fn replace_pair(&mut self, replace: &List, path: &Vec<usize>, level: usize) {
    //     if level == path.len() {
    //         println!("{:?} -> {:?}", self, replace);
    //         self.items.clear();
    //         self.items.append(&mut replace.items.clone());
    //         return;
    //     }
    //
    //     let path_i = path[level];
    //     let mut i = 0;
    //     for element in self.items.iter_mut() {
    //         match element {
    //             Element::List(ref mut list) => {
    //                 if i == path_i {
    //                     list.replace_pair(replace, path, level + 1);
    //                 } else {
    //                 }
    //             }
    //             _ => {}
    //         }
    //         i += 1;
    //     }
    // }

    fn find_first_left_integer(&self, root: &List, last: &List) -> Option<Uuid> {
        // println!("find_first_left_integer {}", self);
        if self.is_integer_left() {
            return Some(self.uuid);
        }

        if self.is_list_pair() {
            // down search
            if let Some(l) = self.left_list() {
                if l.uuid != last.uuid {
                    if let Some(res) = l.find_first_right_integer_down(root) {
                        return Some(res);
                    }
                }
            }
        }
        if let Some(parent) = root.parent(self.uuid) {
            if let Some(r) = parent.right_list() {
                if r.uuid == self.uuid {
                    if let Some(res) = parent.find_first_left_integer(root, &self) {
                        return Some(res);
                    }
                }
            }
            if let Some(l) = parent.left_list() {
                if l.uuid == self.uuid {
                    if let Some(res) = parent.find_first_left_integer(root, &self) {
                        return Some(res);
                    }
                }
            }
        }
        None
    }

    fn find_first_left_integer_down(&self, root: &List) -> Option<Uuid> {
        if self.is_integer_left() || self.is_integer_pair() {
            return Some(self.uuid);
        }
        if let Some(l) = self.left_list() {
            if let Some(res) = l.find_first_left_integer_down(root) {
                return Some(res);
            }
        }
        None
    }

    fn find_first_right_integer_down(&self, root: &List) -> Option<Uuid> {
        if self.is_integer_right() || self.is_integer_pair() {
            return Some(self.uuid);
        }
        if let Some(r) = self.right_list() {
            if let Some(res) = r.find_first_right_integer_down(root) {
                return Some(res);
            }
        }
        None
    }

    fn find_first_right_integer(&self, root: &List, last: &List) -> Option<Uuid> {
        // println!("find_first_right_integer {}", self);
        if self.is_integer_right() {
            return Some(self.uuid);
        }
        if self.is_list_pair() {
            // down search
            if let Some(r) = self.right_list() {
                if r.uuid != last.uuid {
                    if let Some(res) = r.find_first_left_integer_down(root) {
                        return Some(res);
                    }
                }
            }
        }
        if let Some(parent) = root.parent(self.uuid) {
            if let Some(r) = parent.right_list() {
                if r.uuid == self.uuid {
                    if let Some(res) = parent.find_first_right_integer(root, &self) {
                        return Some(res);
                    }
                }
            }
            if let Some(l) = parent.left_list() {
                if l.uuid == self.uuid {
                    if let Some(res) = parent.find_first_right_integer(root, &self) {
                        return Some(res);
                    }
                }
            }
        }
        None
    }

    fn explode(&mut self, pair: &List) {
        let mut left_value = pair.left_value().unwrap();
        let mut right_value = pair.right_value().unwrap();

        let left_uuid = pair.find_first_left_integer(&self, pair);
        let right_uuid = pair.find_first_right_integer(&self, pair);

        if left_uuid.is_some() {
            let left = self.find_mut(left_uuid.unwrap()).unwrap();
            // println!("left {}", left);

            if left.is_integer_left() {
                let left_add_value = left.left_value().unwrap();
                left.set_left_value(left_value + left_add_value);
            }
            if left.is_integer_right() || left.is_integer_pair() {
                let left_add_value = left.right_value().unwrap();
                left.set_right_value(left_value + left_add_value);
            }

            // println!("left {}", left);
        }

        if right_uuid.is_some() {
            let right = self.find_mut(right_uuid.unwrap()).unwrap();
            // println!("right {}", right);
            if right.is_integer_right() {
                let right_add_value = right.right_value().unwrap();
                right.set_right_value(right_value + right_add_value);
            }
            if right.is_integer_left() || right.is_integer_pair() {
                let right_add_value = right.left_value().unwrap();
                right.set_left_value(right_value + right_add_value);
            }

            // println!("right {}", right);
        }

        let p = self.parent(pair.uuid).unwrap();

        let mut parent = self.find_mut(p.uuid).unwrap();
        match parent.left() {
            Element::List(ref list) => {
                if list.uuid == pair.uuid {
                    parent.set_left_value(0);
                }
            }
            _ => {}
        }
        match parent.right() {
            Element::List(ref list) => {
                if list.uuid == pair.uuid {
                    parent.set_right_value(0);
                }
            }
            _ => {}
        }
    }

    fn can_split(&self) -> bool {
        // if self.is_integer_right() || self.is_integer_pair() {
        //     if self.right_value().unwrap() >= 10 {
        //         return true;
        //     }
        // }
        // if self.is_integer_left() || self.is_integer_pair() {
        //     if self.left_value().unwrap() >= 10 {
        //         return true;
        //     }
        // }
        for element in self.items.iter() {
            match element {
                Element::List(ref list) => {
                    if list.can_split() {
                        return true;
                    }
                }
                Element::Integer(ref value) => {
                    if *value >= 10 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        // if self.is_integer_left() || self.is_integer_pair() {
        //     if self.left_value().unwrap() >= 10 {
        //         //
        //         let split_value = self.left_value().unwrap();
        //         println!("split left {:?}", split_value);
        //         let left = split_value / 2;
        //         let right = split_value - left;
        //         let s = List::new_with_values(left, right);
        //         self.set_left_list(s);
        //         return true;
        //     }
        // }
        // if self.is_integer_right() || self.is_integer_pair() {
        //     if self.right_value().unwrap() >= 10 {
        //         //
        //         let split_value = self.right_value().unwrap();
        //         println!("split right {:?}", split_value);
        //         let left = split_value / 2;
        //         let right = split_value - left;
        //         let s = List::new_with_values(left, right);
        //         self.set_right_list(s);
        //         return true;
        //     }
        // }
        let mut i = 0;
        for element in self.items.iter_mut() {
            match element {
                Element::List(ref mut list) => {
                    if list.split() {
                        return true;
                    }
                }
                Element::Integer(ref value) => {
                    let split_value = *value;
                    if split_value >= 10 {
                        println!("split left {:?}", value);
                        let left = split_value / 2;
                        let right = split_value - left;
                        let s = List::new_with_values(left, right);
                        if i == 0 {
                            self.set_left_list(s);
                        } else if i == 1 {
                            self.set_right_list(s);
                        }
                        return true;
                    }
                }
            }
            i += 1;
        }

        false
    }

    fn calc_magnitude(&self, sum: u32) -> u32 {
        let mut sum = sum;
        for element in self.items.iter() {
            match element {
                Element::List(ref list) => {
                    if list.is_integer_pair() {
                        sum += list.left_value().unwrap() * 3 + list.right_value().unwrap() * 2
                    } else if list.is_integer_left() {
                        sum += list.left_value().unwrap() * 3 + list.calc_magnitude(sum) * 2
                    } else if list.is_integer_right() {
                        sum += list.calc_magnitude(sum) * 3 + list.right_value().unwrap() * 2
                    } else if list.is_list_pair() {
                        sum += list.calc_magnitude(sum)
                    }
                }
                _ => {}
            }
        }
        sum
    }
}

fn add(root: &List, list2: &List) -> Element {
    let mut l = Element::List(Box::new(List::new()));
    match l {
        Element::List(ref mut l) => {
            l.set_left_list(root.clone());
            l.set_right_list(list2.clone())
        }
        _ => {}
    }
    l
}

fn parse_term(line: &String, i: usize, current: &mut List) -> usize {
    let mut i = i;
    let mut current = current;

    while i < line.len() {
        let c = line.chars().nth(i).unwrap();
        if c == '[' {
            i += 1;
            let mut l = List::new();
            i = parse_term(line, i, &mut l);
            current.items.push(Element::List(Box::new(l)));
        } else if c == ']' {
            i += 1;
            return i;
        } else if c == ',' {
            i += 1;
        } else {
            let c = line.chars().nth(i).unwrap();
            if c.is_digit(10) {
                let int_value = Element::Integer(c.to_digit(10).unwrap());
                current.items.push(int_value);
                i += 1;
            }
        }
    }
    i
}

fn part1(lines: &[String]) -> usize {
    let mut root: Option<List> = None;

    for line in lines.iter() {
        let mut term = List::new();
        parse_term(line, 0, &mut term);
        let term_item = term.items.first().unwrap();
        match term_item {
            Element::List(ref list) => {
                if root.is_none() {
                    let mut root_list = List::new();
                    root_list.items.push(Element::List(list.clone()));
                    root = Some(root_list);
                } else {
                    let r = root.as_ref().unwrap();
                    if r.items.len() == 1 {
                        let root_item = r.items.first().unwrap();
                        match root_item {
                            Element::List(ref old_list) => {
                                let mut root_list = List::new();
                                root_list.items.push(Element::List(old_list.clone()));
                                root_list.items.push(Element::List(list.clone()));

                                let mut root_list2 = List::new();
                                root_list2
                                    .items
                                    .push(Element::List(Box::new(root_list.clone())));
                                root = Some(root_list2);
                            }
                            _ => {}
                        }
                    } else {
                        let mut root_list = List::new();
                        root_list.items.push(Element::List(Box::new(r.clone())));
                        root_list.items.push(Element::List(list.clone()));
                        root = Some(root_list);
                    }
                }
            }
            _ => {}
        }
        // println!("{:?}", root);

        let mut r = root.as_mut().unwrap();
        println!("{}", r);

        // for element in l.items.iter() {
        //     match element {
        //         Element::List(ref list) => {
        //             println!("{}", list);
        //             copy = Some(list.clone());
        //         }
        //         _ => {}
        //     }
        // }
        //
        // if root.is_none() {
        //     root = Element::List(Box::new(l))
        // } else {
        //     let mut l = List::new();
        //     l.items.push(root_list.clone());
        //     l.items.push(l.clone());
        //     root = Element::List(Box::new(l))
        // }
        // let mut r = root.as_mut().unwrap();
        // println!("{}", r);

        loop {
            let mut can_explode = true;
            let mut can_split = true;

            if let Some(uuid) = r.get_first_pair(0) {
                // let l = r.find(uuid).unwrap();
                // println!("{:?}", l);
                // let mut l_new = l.clone();
                let pair = r.find(uuid).unwrap().clone();
                println!("explode {}", pair);
                r.explode(&pair);
                // println!("{:?}", l_new);
                // let parent = r.parent(l.uuid).unwrap();
                // r.replace(parent.uuid, &l_new);

                // let parent = root.as_ref().unwrap().parent(l.uuid).unwrap();
                // parent.replace_pair(&pair, );
                println!("after explode {}", r);
                continue;
            } else {
                can_explode = false;
            }
            if r.can_split() {
                r.split();
                println!("after split {}", r);
                continue;
            } else {
                can_split = false;
            }

            println!();

            if !can_split && !can_explode {
                break;
            }
        }
        println!("{}", r.calc_magnitude(0));
    }
    0usize
}

fn part2(lines: &[String]) -> usize {
    0usize
}

fn main() {
    // let lines = read_lines_as_vec("input/input_day18.txt").unwrap();
    let lines = vec![
        "[[1,2],[[3,4],5]]", // "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                             // "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                             // "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                             // "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                             // "[7,[5,[[3,8],[1,4]]]]",
                             // "[[2,[2,2]],[8,[8,1]]]",
                             // "[2,9]",
                             // "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                             // "[[[5,[7,4]],7],1]",
                             // "[[[[4,2],2],6],[8,7]]",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    // [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

// #[cfg(test)]
// mod tests {
//     use crate::{part1, part2};
//
//     #[test]
//     fn it_works() {
//         let lines = vec![
//             /*"[[6,[5,[4,[3,2]]]],1]",
//             "[7,[6,[5,[4,[3,2]]]]]",
//             "[[[[[9,8],1],2],3],4]",
//             "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
//             "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",*/
//             "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"
//         ];
//         assert_eq!(part1(&lines));
//         assert_eq!(part2(&lines));
//     }
// }
