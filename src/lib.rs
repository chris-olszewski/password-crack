#![feature(test)]
extern crate test;

use std::rc::Rc;

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil
}

#[derive(Clone)]
pub struct IStack<T> {
    head: Rc<List<T>>,
    pub size: usize
}

use List::{Cons, Nil};

impl<T: Copy + Eq> IStack<T> {
    fn new () -> IStack<T>{
        IStack {
            head: Rc::new(Nil),
            size: 0
        }
    }

    pub fn push (&self, ele: T) -> IStack<T> {
        let head = Rc::new(Cons(ele, Rc::clone(&self.head)));
        let size = self.size + 1;
        // clone
        IStack {
            head,
            size
        }
    }

    pub fn pop (&self) -> (Option<&T>, IStack<T>) {
        if let Cons(ref ele, ref rest) = *self.head {
            let new_stack = IStack {
                head: Rc::clone(rest),
                size: self.size - 1
            };
            (Some(ele), new_stack)
        } else {
            // nothing to remove so stack stays the same?
            let new_stack = IStack {
                head: Rc::clone(&self.head),
                size: 0
            };
            (None, new_stack)
        }
    }

    pub fn peek (&self) -> Option<&T> {
        let (x, _) = self.pop();
        x
    }

    pub fn iter (&self) -> StackIter<T> {
        // really just get another ref to the current
        StackIter {
            inner: self.clone()
        }
    }
}

pub struct StackIter<T> {
    inner: IStack<T>
}

impl<T: Copy + Eq> Iterator for StackIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let (x, rest) = self.inner.pop();
        let y = x.copied();
        self.inner = rest;
        y
    }
}

// start project specific stuff
// again want to build from "rules"
// rule: state -> list of new states

pub struct CostRule<T> {
    inner: Vec<Vec<(T, usize)>>,
    cost: usize

}

impl<T: Eq + Copy> CostRule<T> {
    pub fn new(choices: Vec<Vec<(T, usize)>>, cost: usize) -> Self {
        CostRule {
            inner: choices,
            cost,
        }
    }

    pub fn call(&self, stack: &IStack<(T, usize)>) -> Vec<IStack<(T, usize)>> {
        // probably make this safer
        if self.inner.len() <= stack.size {
            return vec![]
        }
        let choices = &self.inner[stack.size];
        let score = stack.peek().map_or(0, |x| x.1);
        let mut res = Vec::new();
        for (c, cost) in choices {
            let new_score = score + cost;
            if  new_score < self.cost {
                let new_stack = stack.push((*c, new_score));
                res.push(new_stack);
            }
        }
        res
    }
}

pub fn builds_stacks<T: Copy + Eq>(rule: CostRule<T>) -> Vec<IStack<(T, usize)>> {
    let mut layer = vec![IStack::new()];
    loop {
        let mut next_layer = Vec::new();
        for stack in &layer {
            next_layer.append(&mut rule.call(stack));
        }
        if next_layer.is_empty() {
            break
        } else {
            layer = next_layer;
        }
    }
    layer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let stack = IStack::new().push(3).push(2).push(1);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_rule() {
        let chars = vec![
            vec![('a', 4), ('A', 1)],
            vec![('b', 2), ('B', 4)],
        ];
        let rule = CostRule::new(chars, 4);
        let res = rule.call(&IStack::new());

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].size, 1);
        assert_eq!(res[0].peek().copied(), Some(('A', 1)));
    }

    #[test]
    fn test_build_stacks() {
        let chars = vec![
            vec![('a', 4), ('A', 1)],
            vec![('b', 2), ('B', 2)],
        ];
        let rule = CostRule::new(chars, 4);
        let res = builds_stacks(rule);

        assert_eq!(res.len(), 2);
        assert_eq!(res[0].peek().copied(), Some(('b', 3)));
    }
}
