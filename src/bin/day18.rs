use itertools::Itertools;
use regex::Regex;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use aoc_2021::read_lines_as_vec;

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
        if self.items.len() == 1 {
            match write!(f, "{}", self.items.first().unwrap()) {
                Ok(_) => {}
                Err(_) => {}
            };
        } else {
            let mut i = 0;
            for element in self.items.iter() {
                if i == 0 {
                    match write!(f, "{},", element) {
                        Ok(_) => {}
                        Err(_) => {}
                    };
                } else {
                    match write!(f, "{}", element) {
                        Ok(_) => {}
                        Err(_) => {}
                    };
                }
                i += 1;
            }
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

    // must always use uuid to get a mutable reference
    // to change the tree in a separate step to avoid
    // any borrow issues
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

    // yes we dont need a parent ref/pointer/whatever
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

    fn find_first_left_integer(&self, root: &List, last: &List) -> Option<Uuid> {
        if self.is_integer_left() {
            return Some(self.uuid);
        }

        if self.is_list_pair() {
            // down search the other side
            // BUT we must search right down to get the closest to us
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
        if self.is_integer_right() {
            return Some(self.uuid);
        }
        if self.is_list_pair() {
            // down search the other side
            // BUT we must search left down to get the closest
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
        let left_value = pair.left_value().unwrap();
        let right_value = pair.right_value().unwrap();

        let left_uuid = pair.find_first_left_integer(&self, pair);
        let right_uuid = pair.find_first_right_integer(&self, pair);

        if left_uuid.is_some() {
            let left = self.find_mut(left_uuid.unwrap()).unwrap();

            if left.is_integer_left() {
                let left_add_value = left.left_value().unwrap();
                left.set_left_value(left_value + left_add_value);
            }
            if left.is_integer_right() || left.is_integer_pair() {
                let left_add_value = left.right_value().unwrap();
                left.set_right_value(left_value + left_add_value);
            }
        }

        if right_uuid.is_some() {
            let right = self.find_mut(right_uuid.unwrap()).unwrap();

            if right.is_integer_right() {
                let right_add_value = right.right_value().unwrap();
                right.set_right_value(right_value + right_add_value);
            }
            if right.is_integer_left() || right.is_integer_pair() {
                let right_add_value = right.left_value().unwrap();
                right.set_left_value(right_value + right_add_value);
            }
        }

        let p = self.parent(pair.uuid).unwrap();
        let parent = self.find_mut(p.uuid).unwrap();

        // replace pair with 0 in parent
        if let Some(l) = parent.left_list() {
            if l.uuid == pair.uuid {
                parent.set_left_value(0);
            }
        }
        if let Some(r) = parent.right_list() {
            if r.uuid == pair.uuid {
                parent.set_right_value(0);
            }
        }
    }

    fn can_split(&self) -> bool {
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

    fn calc_magnitude(&self) -> u32 {
        // root always is single item list
        if self.items.len() == 1 {
            let item = self.items.first().unwrap();
            match item {
                Element::List(ref list) => list.calc_magnitude(),
                _ => 0,
            }
        } else {
            let left = match self.left() {
                Element::Integer(ref value) => *value,
                Element::List(ref list) => list.calc_magnitude(),
            };
            let right = match self.right() {
                Element::Integer(ref value) => *value,
                Element::List(ref list) => list.calc_magnitude(),
            };
            left * 3 + right * 2
        }
    }
}

fn parse_term(line: &String, i: usize, current: &mut List) -> usize {
    let mut i = i;
    let current = current;

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
                let reg = Regex::new(r"\d+").unwrap();
                let int_value_str = reg.find(line.get(i..).unwrap()).unwrap().as_str();
                let int_value = Element::Integer(int_value_str.parse().unwrap());
                current.items.push(int_value);
                i += 1;
            }
        }
    }
    i
}
fn reduce(r: &mut List) {
    loop {
        if let Some(uuid) = r.get_first_pair(0) {
            let pair = r.find(uuid).unwrap().clone();
            r.explode(&pair);
            continue;
        } else {
            if r.can_split() {
                r.split();
                continue;
            }
        }
        break;
    }
}

fn calc_magnitude(left: &Element, right: &Element) -> u32 {
    let mut root_list = List::new();
    match left {
        Element::List(ref l) => {
            root_list.items.push(Element::List(l.clone()));
        }
        _ => {}
    };

    match right {
        Element::List(ref r) => {
            root_list.items.push(Element::List(r.clone()));
        }
        _ => {}
    };

    let mut root_list2 = List::new();
    root_list2
        .items
        .push(Element::List(Box::new(root_list.clone())));
    let mut root = Some(root_list2);
    let r = root.as_mut().unwrap();
    reduce(r);
    r.calc_magnitude()
}

fn part1(lines: &[String]) -> u32 {
    let mut root: Option<List> = None;

    for line in lines.iter() {
        let mut term = List::new();
        parse_term(line, 0, &mut term);
        let term_item = term.items.first().unwrap();

        if root.is_none() {
            let mut root_list = List::new();
            match term_item {
                Element::List(ref l) => {
                    root_list.items.push(Element::List(l.clone()));
                }
                _ => {}
            };
            root = Some(root_list);
            continue;
        }
        let mut r = root.as_mut().unwrap();
        let mut root_list = List::new();
        match r.items.first().unwrap() {
            Element::List(ref l) => {
                root_list.items.push(Element::List(l.clone()));
            }
            _ => {}
        };

        match term_item {
            Element::List(ref r) => {
                root_list.items.push(Element::List(r.clone()));
            }
            _ => {}
        };

        let mut root_list2 = List::new();
        root_list2
            .items
            .push(Element::List(Box::new(root_list.clone())));

        root = Some(root_list2);

        let r = root.as_mut().unwrap();
        reduce(r);
    }
    let r = root.as_ref().unwrap();
    r.calc_magnitude()
}

fn part2(lines: &[String]) -> u32 {
    // 4616
    let mut terms = vec![];

    for line in lines.iter() {
        let mut term = List::new();
        parse_term(line, 0, &mut term);
        let term_item = term.items.first().unwrap();
        terms.push(term_item.clone());
    }

    let mut max_magnitude = 0;

    for pair in terms.iter().combinations(2) {
        let left = pair.first().unwrap();
        let right = pair.last().unwrap();

        let magnitude = calc_magnitude(left, right);
        if magnitude > max_magnitude {
            max_magnitude = magnitude
        }

        let magnitude = calc_magnitude(right, left);
        if magnitude > max_magnitude {
            max_magnitude = magnitude
        }
    }

    max_magnitude
}

fn main() {
    // [[[[7,7],[7,7]],[[0,8],[9,9]]],[[[6,6],[6,7]],[2,1]]] = 3359

    let lines = read_lines_as_vec("input/input_day18.txt").unwrap();

    // let lines = vec![
    //     "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
    //     "[[[5,[2,8]],4],[5,[[9,9],0]]]",
    //     "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
    //     "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
    //     "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
    //     "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
    //     "[[[[5,4],[7,7]],8],[[8,3],8]]",
    //     "[[9,3],[[9,9],[6,[4,9]]]]",
    //     "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
    //     "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect::<Vec<_>>();

    // [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] - 3488
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        assert_eq!(part1(&lines), 4140);

        let lines = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        assert_eq!(part2(&lines), 3993);
    }
}
