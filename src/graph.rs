use crate::graph::Distance::Dist;
use std::cell::RefCell;
use std::cmp::{min, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter, Write};
use std::hash::Hash;
use std::ops::{Add, Deref};
use std::rc::Rc;
use std::slice::Iter;

type Predecessor<T: Eq + Hash + Copy> = Option<T>;

#[derive(Clone, Copy, Eq, PartialEq)]
struct VertAttributes<T: Eq + Hash + Copy> {
    id: T,
    predecessor: Predecessor<T>,
    distance: Distance,
}

impl<T: Eq + Hash + Copy> VertAttributes<T> {
    fn new_inf(vert: T) -> VertAttributes<T> {
        VertAttributes {
            id: vert,
            predecessor: Predecessor::None,
            distance: Distance::Inf,
        }
    }

    fn new_zero(vert: T) -> VertAttributes<T> {
        VertAttributes {
            id: vert,
            predecessor: Predecessor::None,
            distance: Distance::Dist(0),
        }
    }

    fn new_dist(vert: T, distance: Distance) -> VertAttributes<T> {
        VertAttributes {
            id: vert,
            predecessor: Predecessor::None,
            distance,
        }
    }
}

impl<T: Eq + Hash + Copy> PartialOrd<Self> for VertAttributes<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(self.cmp(other))
    }
}

impl<T: Eq + Hash + Copy> Ord for VertAttributes<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub struct PositionTrackingMinHeap<T: Eq + Hash + Copy>(Vec<VertAttributes<T>>, HashMap<T, usize>);

impl<T: Eq + Hash + Copy> PositionTrackingMinHeap<T> {
    pub fn new() -> PositionTrackingMinHeap<T> {
        PositionTrackingMinHeap(Vec::new(), HashMap::new())
    }

    fn _parent(i: usize) -> usize {
        i >> 1
    }

    fn _left(i: usize) -> usize {
        i << 1
    }
    fn _right(i: usize) -> usize {
        (i << 1) + 1
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn _swap(&mut self, first: usize, second: usize) {
        let tmp: VertAttributes<T> = (*self.0.get(first).unwrap()).clone();
        self.1.insert(tmp.id, second);
        let snd = self.0.get(second).unwrap();
        self.1.insert(snd.id, first);
        *self.0.get_mut(first).unwrap() = (*snd).clone();
        *self.0.get_mut(second).unwrap() = tmp;
    }

    pub fn get(&self, index: usize) -> Option<&VertAttributes<T>> {
        self.0.get(index)
    }

    pub fn min_heapify(&mut self, index: usize) {
        let left = PositionTrackingMinHeap::<T>::_left(index);
        let right = PositionTrackingMinHeap::<T>::_right(index);
        let mut smallest = left;
        let vec_len = self.0.len();
        if left < vec_len && self.0.get(left).unwrap() < self.0.get(index).unwrap() {
            smallest = left;
        } else {
            smallest = index;
        }
        if right < vec_len && self.0.get(right).unwrap() < self.0.get(smallest).unwrap() {
            smallest = right;
        }
        if smallest != index {
            self._swap(smallest, index);
            self.min_heapify(smallest);
        }
    }

    pub fn extract_min(&mut self) -> VertAttributes<T> {
        let len = self.0.len();
        let last_id = self.0.get(len - 1).unwrap().id;
        let min = self.0.swap_remove(0);
        self.1.insert(last_id, 0);
        self.1.remove(&min.id);
        self.min_heapify(0);
        min
    }

    pub fn insert(&mut self, value: VertAttributes<T>) {
        self.0.push(value.clone());
        self.1.insert(value.id, self.0.len() - 1);
        self.heap_decrease_key(self.0.len() - 1, value);
    }

    pub fn heap_decrease_key(&mut self, index: usize, value: VertAttributes<T>) {
        let current = self.0.get(index).unwrap();
        if value > *current {
            panic!("New key is greater than current")
        }
        *self.0.get_mut(index).unwrap() = value;
        let mut idx = index;
        while idx > 0
            && self
                .0
                .get(PositionTrackingMinHeap::<T>::_parent(idx))
                .unwrap()
                > self.0.get(idx).unwrap()
        {
            let parent = PositionTrackingMinHeap::<T>::_parent(idx);
            self._swap(idx, parent);
            idx = parent;
        }
    }

    pub fn vert_index(&self, vert: T) -> Option<usize> {
        self.1.get(&vert).map(|a| a.clone())
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Edge {
    value: Option<u32>,
    from: u32,
    to: u32,
}

impl Edge {
    pub fn new_weighted(value: u32, from: u32, to: u32) -> Edge {
        Edge {
            value: Option::Some(value),
            from,
            to,
        }
    }

    pub fn new(from: u32, to: u32) -> Edge {
        Edge {
            value: Option::None,
            from,
            to,
        }
    }

    pub fn reverse(&self) -> Edge {
        Edge {
            value: self.value,
            from: self.to,
            to: self.from,
        }
    }

    pub fn compare_values(&self, other: &Edge) -> Ordering {
        self.value.cmp(&other.value)
    }

    pub fn same_ignore_direction(&self, other: &Edge) -> bool {
        let mut result = self.eq(other);
        if !result {
            if self.to == other.from && self.from == other.to {
                result = self.value.eq(&other.value);
            } else {
                result = false;
            }
        }
        result
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = self.value {
            let str = format!("{{value: {}, from: {}, to: {}}}", v, self.from, self.to);
            f.write_str(str.as_str())
        } else {
            let str = format!("{{from: {}, to: {}}}", self.from, self.to);
            f.write_str(str.as_str())
        }
    }
}

pub enum Edges {
    Empty,
    Edges(Vec<Edge>),
}

impl Edges {
    pub fn new() -> Edges {
        Edges::Empty
    }

    pub fn from_edge(edge: Edge) -> Edges {
        let mut v = Vec::new();
        v.push(edge);
        Edges::Edges(v)
    }

    pub fn reverse(&mut self) {
        match self {
            Edges::Empty => {}
            Edges::Edges(e) => {
                e.reverse();
            }
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        match self {
            Edges::Empty => {
                let mut edges: Vec<Edge> = Vec::new();
                edges.push(edge);
                *self = Edges::Edges(edges);
            }
            Edges::Edges(v) => {
                v.push(edge);
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Edges::Empty => 0,
            Edges::Edges(e) => e.len(),
        }
    }

    pub fn iter(&self) -> Iter<'_, Edge> {
        match self {
            Edges::Empty => [].iter(),
            Edges::Edges(edges) => edges.iter(),
        }
    }
}

impl Display for Edges {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Edges::Empty => f.write_str("[]"),
            Edges::Edges(e) => {
                f.write_str("[")?;
                let len = e.len();
                for x in 0..len {
                    let value = e.get(x).unwrap();
                    if x == 0 {
                        f.write_str(format!("{}", value).as_str())?;
                    } else {
                        f.write_str(format!(",{}", value).as_str())?;
                    }
                }
                f.write_str("]")
            }
        }
    }
}

pub struct Graph {
    adj: HashMap<u32, Edges>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            adj: HashMap::new(),
        }
    }

    pub fn add_adj(&mut self, edge: Edge) {
        if let Some(v) = self.adj.get_mut(&edge.from) {
            v.add_edge(edge.clone());
        } else {
            let mut l = Vec::new();
            let key = edge.from;
            l.push(edge.clone());
            self.adj.insert(key, Edges::Edges(l));
        }
        if self.adj.get(&edge.to).is_none() {
            let key = edge.to;
            self.adj.insert(key, Edges::Empty);
        }
    }

    pub fn add_adj_both(&mut self, edge: Edge) {
        if let Some(v) = self.adj.get_mut(&edge.from) {
            v.add_edge(edge.clone());
        } else {
            self.adj.insert(edge.from, Edges::from_edge(edge.clone()));
        }
        if let Some(v) = self.adj.get_mut(&edge.to) {
            v.add_edge(edge.reverse());
        } else {
            self.adj.insert(edge.to, Edges::from_edge(edge.reverse()));
        }
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.adj.iter() {
            f.write_str(format!("{}: {}\n", key, value).as_str())?;
        }
        Ok(())
    }
}

pub struct Vertices(Vec<u32>);

impl Vertices {
    pub fn new() -> Vertices {
        Vertices(Vec::new())
    }

    pub fn push(&mut self, vert: u32) {
        self.0.push(vert);
    }
}

impl Display for Vertices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[");
        let len = self.0.len();
        for x in 0..len {
            let val = self.0.get(x).unwrap();
            if x == 0 {
                f.write_str(val.to_string().as_str());
            } else {
                f.write_str(format!(",{}", val).as_str());
            }
        }
        f.write_str("]");
        Ok(())
    }
}

fn visited_map(graph: &Graph) -> HashMap<u32, bool> {
    let mut visited = HashMap::new();
    for (key, _) in graph.adj.iter() {
        visited.insert(*key, false);
    }
    visited
}

pub fn breadth_first_search(graph: &Graph, first: u32) -> Vertices {
    let mut visited = visited_map(graph);
    visited.insert(first, true);
    let mut result: Vertices = Vertices::new();
    result.push(first);
    let mut fifo = VecDeque::new();
    fifo.push_back(first);
    while let Some(v) = fifo.pop_front() {
        if let Some(vert) = graph.adj.get(&v) {
            match vert {
                Edges::Empty => {}
                Edges::Edges(e) => {
                    for next in e.iter() {
                        if let Some(visited_vert) = visited.get_mut(&next.to) {
                            if !(*visited_vert) {
                                *visited_vert = true;
                                fifo.push_back(next.to);
                                result.push(next.to);
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn dfs_visit(vert: &u32, graph: &Graph, visited: &mut HashMap<u32, bool>, result: &mut Vertices) {
    visited.insert(*vert, true);
    result.push(*vert);
    if let Some(e) = graph.adj.get(vert) {
        match e {
            Edges::Empty => {}
            Edges::Edges(edges) => {
                for edge in edges.iter() {
                    if let Some(visited_vert) = visited.get(&edge.to) {
                        if !(*visited_vert) {
                            dfs_visit(&edge.to, graph, visited, result);
                        }
                    }
                }
            }
        }
    }
}

pub fn depth_first_search(graph: &Graph) -> Vertices {
    let mut result: Vertices = Vertices::new();
    let mut visited = visited_map(graph);
    for (key, _) in graph.adj.iter() {
        if let Some(visited_vert) = visited.get(key) {
            if !(*visited_vert) {
                dfs_visit(key, graph, &mut visited, &mut result);
            }
        }
    }
    result
}

pub fn depth_first_search_iter(graph: &Graph) -> Vertices {
    let mut visited = visited_map(graph);
    let mut result = Vertices::new();
    let mut stack: Vec<u32> = Vec::new();
    for (key, _) in graph.adj.iter() {
        if !visited.get(key).unwrap() {
            stack.push(*key);
            println!("Push: {}", *key);
            while let Some(lst) = stack.last() {
                println!("Curr: {}", lst);
                visited.insert(*lst, true);
                match graph.adj.get(lst).unwrap() {
                    Edges::Empty => {}
                    Edges::Edges(edges) => {
                        let mut some_to_visit = false;
                        for edge in edges.iter() {
                            if !visited.get(&edge.to).unwrap() {
                                stack.push(edge.to);
                                println!("Push: {}", edge.to);
                                some_to_visit = true;
                                break;
                            }
                        }
                        if !some_to_visit {
                            let k = stack.pop().unwrap();
                            result.push(k);
                            println!("Pop: {}", k);
                        }
                    }
                }
            }
        }
    }
    result
}

fn edges_sorted_asc(graph: &Graph) -> Vec<Edge> {
    let mut set: HashSet<Edge> = HashSet::new();
    for (_, edges) in graph.adj.iter() {
        match edges {
            Edges::Empty => {}
            Edges::Edges(e) => {
                for edge in e.iter() {
                    let rev = edge.reverse();
                    if !(set.contains(edge) || set.contains(&rev)) {
                        set.insert(edge.clone());
                    }
                }
            }
        }
    }
    let mut result: Vec<Edge> = set.into_iter().collect();
    result.sort_by(|a, b| a.compare_values(b));
    result
}

fn union_sets(sets: &mut HashMap<u32, u32>, first: u32, second: u32) {
    let second_set = sets.get(&second).unwrap().clone();
    let first_set = sets.get(&first).unwrap().clone();
    sets.insert(first, second_set);
    for (_, value) in sets.iter_mut() {
        if *value == first_set {
            *value = second_set;
        }
    }
}

pub fn minimal_spanning_tree_kruskal(graph: &Graph) -> Edges {
    let mut result = Edges::new();
    let mut sets: HashMap<u32, u32> = HashMap::new();
    for (vert, _) in graph.adj.iter() {
        sets.insert(*vert, *vert);
    }
    let edges = edges_sorted_asc(graph);
    for edge in edges.iter() {
        if !(sets.get(&edge.from).unwrap() == sets.get(&edge.to).unwrap()) {
            result.add_edge(edge.clone());
            union_sets(&mut sets, edge.from, edge.to);
        }
    }
    result
}

#[derive(Clone, Copy)]
pub enum Distance {
    Inf,
    Dist(u32),
}

impl Distance {
    pub fn set_dist(&mut self, value: u32) {
        *self = Distance::Dist(value);
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Distance::Inf => f.write_str("Inf"),
            Distance::Dist(x) => f.write_str(format!("Dist({})", x).as_str()),
        }
    }
}

impl Eq for Distance {}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Distance::Inf, Distance::Inf) => Ordering::Equal,
            (Distance::Dist(_), Distance::Inf) => Ordering::Less,
            (Distance::Inf, Distance::Dist(x)) => Ordering::Greater,
            (Distance::Dist(x), Distance::Dist(y)) => x.cmp(y),
        }
    }
}

impl PartialEq<Self> for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd<Self> for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(self.cmp(other))
    }
}

impl Add for Distance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Distance::Inf, _) => Distance::Inf,
            (_, Distance::Inf) => Distance::Inf,
            (Distance::Dist(x), Distance::Dist(y)) => Distance::Dist(x + y),
        }
    }
}

pub fn minimal_spanning_tree_prim(graph: &Graph, root: u32) -> Edges {
    let mut result = Edges::new();
    let mut min_heap: PositionTrackingMinHeap<u32> = PositionTrackingMinHeap::new();
    let mut prev = HashMap::new();
    for (vert, _) in graph.adj.iter() {
        if *vert != root {
            min_heap.insert(VertAttributes::new_inf(*vert));
        }
        prev.insert(*vert, *vert);
    }
    min_heap.insert(VertAttributes::new_zero(root));
    while !min_heap.is_empty() {
        let elem = min_heap.extract_min();
        let curr_vert = elem.id;
        match graph.adj.get(&curr_vert).unwrap() {
            Edges::Empty => {}
            Edges::Edges(edges) => {
                for edge in edges {
                    if let Some(index) = min_heap.vert_index(edge.to) {
                        let dist = Distance::Dist(edge.value.unwrap_or(1));
                        if dist < min_heap.get(index).unwrap().distance {
                            prev.insert(edge.to, curr_vert);
                            min_heap
                                .heap_decrease_key(index, VertAttributes::new_dist(edge.to, dist));
                        }
                    }
                }
            }
        }
    }
    for (from, to) in prev.iter() {
        match graph.adj.get(from).unwrap_or(&Edges::Empty) {
            Edges::Empty => {}
            Edges::Edges(edges) => {
                for edge in edges.iter() {
                    if edge.to == *to {
                        result.add_edge(edge.clone());
                    }
                }
            }
        }
    }
    result
}

fn initialize_single_source_dijkstra(graph: &Graph, source: u32) -> PositionTrackingMinHeap<u32> {
    let mut result = PositionTrackingMinHeap::new();
    for (vert, _) in graph.adj.iter() {
        if *vert != source {
            let attributes = VertAttributes::new_inf(*vert);
            result.insert(attributes);
        }
    }
    let source_attributes = VertAttributes::new_zero(source);
    result.insert(source_attributes);
    result
}

fn initialize_single_source(graph: &Graph, source: u32) -> HashMap<u32, VertAttributes<u32>> {
    let mut result = HashMap::new();
    for (vert, _) in graph.adj.iter() {
        let attributes = VertAttributes::new_inf(*vert);
        result.insert(*vert, attributes);
    }
    let source_attributes = VertAttributes::new_zero(source);
    result.insert(source, source_attributes);
    result
}

fn relax(
    attributes: &HashMap<u32, VertAttributes<u32>>,
    edge: &Edge,
) -> Option<VertAttributes<u32>> {
    let from_distance = attributes.get(&edge.from).unwrap().distance;
    let to_distance = attributes.get(&edge.to).unwrap().distance;
    let dist_through = from_distance + Distance::Dist(edge.value.unwrap_or(1));
    let mut result = Option::None;
    if to_distance > dist_through {
        let new_value = VertAttributes {
            distance: dist_through,
            predecessor: Predecessor::Some(edge.from),
            id: edge.to,
        };
        result = Option::Some(new_value);
    }
    result
}

fn recreate_path(start: u32, end: u32, predecessors: &HashMap<u32, u32>) -> Option<Edges> {
    let mut result = Edges::new();
    let mut curr = end;
    let mut found_start = false;
    while let Some(pred) = predecessors.get(&curr) {
        result.add_edge(Edge::new(*pred, curr));
        curr = *pred;
        if *pred == start {
            found_start = true;
            break;
        }
    }
    result.reverse();
    if found_start {
        Option::Some(result)
    } else {
        Option::None
    }
}

fn attributes_to_predecessors(attributes: &HashMap<u32, VertAttributes<u32>>) -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    for (vert, attr) in attributes.iter() {
        match attr.predecessor {
            None => {}
            Some(pred) => {
                result.insert(*vert, pred);
            }
        }
    }
    result
}

pub fn shortest_path_bellman_ford(graph: &Graph, start: u32, end: u32) -> Option<Edges> {
    let mut attributes = initialize_single_source(graph, start);
    for _ in 0..graph.adj.len() {
        for (_, edges) in graph.adj.iter() {
            for edge in edges.iter() {
                if let Some(new_attribute) = relax(&attributes, edge) {
                    attributes.insert(edge.to, new_attribute);
                }
            }
        }
    }
    recreate_path(start, end, &attributes_to_predecessors(&attributes))
}

pub fn shortest_path_dijkstra(graph: &Graph, start: u32, end: u32) -> Option<Edges> {
    let mut attributes = initialize_single_source(graph, start);
    let mut heap = PositionTrackingMinHeap::new();
    for (_, attr) in attributes.iter() {
        heap.insert(attr.clone());
    }
    while !heap.is_empty() {
        let vert_attr = heap.extract_min();
        for edge in graph.adj.get(&vert_attr.id).unwrap().iter() {
            if let Some(new_attribute) = relax(&mut attributes, edge) {
                if let Some(idx) = heap.vert_index(edge.to) {
                    attributes.insert(edge.to, new_attribute);
                    heap.heap_decrease_key(idx, new_attribute);
                }
            }
        }
    }
    recreate_path(start, end, &attributes_to_predecessors(&attributes))
}

enum MapSlice {
    Obstacle,
    Visited,
    Height(u32),
}

impl MapSlice {
    pub fn get_height(&self) -> Option<u32> {
        match self {
            MapSlice::Obstacle => Option::None,
            MapSlice::Visited => Option::None,
            MapSlice::Height(height) => Option::Some(*height),
        }
    }
}

impl Display for MapSlice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapSlice::Obstacle => f.write_str("O"),
            MapSlice::Height(h) => f.write_str(format!("{}", h).as_str()),
            MapSlice::Visited => f.write_str("."),
        }
    }
}

pub struct TerrainMap {
    graph: Vec<Vec<MapSlice>>,
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point2d(u32, u32);

impl Point2d {
    pub fn new(x: u32, y: u32) -> Point2d {
        Point2d(x, y)
    }

    pub fn x(&self) -> u32 {
        self.0
    }

    pub fn y(&self) -> u32 {
        self.1
    }
}

impl Display for Point2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({},{})", self.x(), self.y()).as_str())
    }
}

impl TerrainMap {
    pub fn new(width: u32, height: u32) -> TerrainMap {
        if width <= 1 || height <= 1 {
            panic!("Width and Height should be greater than 1.");
        }
        let mut rows = Vec::with_capacity(height as usize);
        for _ in 0..height {
            let mut row: Vec<MapSlice> = Vec::with_capacity(width as usize);
            for _ in 0..width {
                row.push(MapSlice::Height(1));
            }
            rows.push(row);
        }
        TerrainMap {
            width,
            height,
            graph: rows,
        }
    }

    pub fn distance(&self, curr: &Point2d, neighbor: &Point2d) -> Option<Distance> {
        let mut result = Option::None;
        if let Some(MapSlice::Height(curr_height)) = self.get(curr.x(), curr.y()) {
            if let Some(MapSlice::Height(neighbor_height)) = self.get(neighbor.x(), neighbor.y()) {
                if curr_height > neighbor_height {
                    result = Option::Some(Distance::Dist(curr_height - neighbor_height + 1));
                } else {
                    result = Option::Some(Distance::Dist(neighbor_height - curr_height + 1));
                }
            }
        }
        result
    }

    pub fn neighbors(&self, vert: &Point2d) -> Vec<Point2d> {
        let mut result = Vec::new();
        let x = vert.x();
        let y = vert.y();
        if x > 0 {
            if let Some(MapSlice::Height(_)) = self.get(x - 1, y) {
                result.push(Point2d::new(x - 1, y));
            }
        }
        if x < self.width - 1 {
            if let Some(MapSlice::Height(_)) = self.get(x + 1, y) {
                result.push(Point2d::new(x + 1, y));
            }
        }
        if y > 0 {
            if let Some(MapSlice::Height(_)) = self.get(x, y - 1) {
                result.push(Point2d::new(x, y - 1));
            }
        }
        if y < self.height - 1 {
            if let Some(MapSlice::Height(_)) = self.get(x, y + 1) {
                result.push(Point2d::new(x, y + 1));
            }
        }
        result
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&MapSlice> {
        let row_idx = self.graph.len() - 1 - y as usize;
        self.graph[row_idx].get(x as usize)
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut MapSlice> {
        let row_idx = self.graph.len() - 1 - y as usize;
        self.graph[row_idx].get_mut(x as usize)
    }

    pub fn put_obstacle(&mut self, from: Point2d, to: Point2d) {
        if from.x() != to.x() && from.y() != to.y() {
            panic!("Horizontal or vertical obstacles allowed only.");
        }
        if from.x() == to.x() {
            let dist = (from.y() as i32 - to.y() as i32).abs() as u32;
            let fst_y = if from.y() < to.y() { from.y() } else { to.y() };
            for offset in 0..(dist + 1) {
                *self.get_mut(from.x(), fst_y + offset).unwrap() = MapSlice::Obstacle;
            }
        }
        if from.y() == to.y() {
            let dist = (from.x() as i32 - to.x() as i32).abs() as u32;
            let fst_x = if from.x() < to.x() { from.x() } else { to.x() };
            for offset in 0..(dist + 1) {
                *self.get_mut(fst_x + offset, from.y()).unwrap() = MapSlice::Obstacle;
            }
        }
    }
}

impl Display for TerrainMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.graph.iter() {
            for x in 0..row.len() {
                if x != 0 {
                    f.write_str(format!(" {}", row[x]).as_str())?;
                } else {
                    f.write_str(format!("{}", row[x]).as_str())?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn a_star_recreate_path(
    attributes: &HashMap<Point2d, VertAttributes<Point2d>>,
    start: &Point2d,
    end: &Point2d,
) -> Option<Vec<Point2d>> {
    let mut result = Vec::new();
    let mut found_start = false;
    let mut curr = attributes.get(end).unwrap();
    result.push(*end);
    while let Some(pred) = curr.predecessor {
        result.push(pred.clone());
        if pred == *start {
            found_start = true;
            break;
        }
        curr = attributes.get(&pred).unwrap();
    }
    result.reverse();
    if found_start {
        Option::Some(result)
    } else {
        Option::None
    }
}

pub fn shortest_path_a_star(
    graph: &TerrainMap,
    start: Point2d,
    end: Point2d,
    heuristic: impl Fn(&Point2d, &Point2d) -> Distance,
) -> Option<Vec<Point2d>> {
    let mut real_dist: HashMap<Point2d, Distance> = HashMap::new();
    let mut attributes = HashMap::new();
    let mut heap: PositionTrackingMinHeap<Point2d> = PositionTrackingMinHeap::new();
    for row in 0..graph.height {
        for column in 0..graph.width {
            let point = Point2d::new(row, column);
            if point != start {
                real_dist.insert(point, Distance::Inf);
                attributes.insert(point, VertAttributes::new_inf(point));
            }
        }
    }
    real_dist.insert(start, Distance::Dist(0));
    let init_dist = heuristic(&start, &end);
    attributes.insert(start, VertAttributes::new_dist(start, init_dist));
    for (_, key) in attributes.iter() {
        heap.insert(key.clone());
    }
    while !heap.is_empty() {
        let min_vert = heap.extract_min();
        if min_vert.id == end {
            break;
        }
        for neighbor in graph.neighbors(&min_vert.id).iter() {
            let tentative_score =
                min_vert.distance + graph.distance(&min_vert.id, neighbor).unwrap();
            if tentative_score < *real_dist.get(neighbor).unwrap() {
                let new_attr = VertAttributes {
                    id: neighbor.clone(),
                    predecessor: Predecessor::Some(min_vert.id.clone()),
                    distance: tentative_score + heuristic(neighbor, &end),
                };
                if let Some(idx) = heap.vert_index(*neighbor) {
                    attributes.insert(*neighbor, new_attr.clone());
                    heap.heap_decrease_key(idx, new_attr);
                    real_dist.insert(*neighbor, tentative_score);
                }
            }
        }
    }
    a_star_recreate_path(&attributes, &start, &end)
}

pub fn mark_path(graph: &mut TerrainMap, path: &Vec<Point2d>) {
    for point in path.iter() {
        if let Some(slice) = graph.get_mut(point.x(), point.y()) {
            *slice = MapSlice::Visited;
        }
    }
}
