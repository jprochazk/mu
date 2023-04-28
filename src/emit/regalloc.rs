use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Default)]
pub struct RegAlloc {
  preserve: Vec<Option<Register>>,
  intervals: Vec<Interval>,
  event: usize,
}

#[derive(Clone, Copy)]
struct Interval {
  index: usize,
  start: usize,
  end: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register(usize);

impl RegAlloc {
  pub fn new() -> Self {
    Self {
      preserve: Vec::new(),
      intervals: Vec::new(),
      event: 0,
    }
  }

  fn event(&mut self) -> usize {
    let event = self.event;
    self.event += 1;
    event
  }

  pub fn access(&mut self, register: Register) -> usize {
    let index = register.0;
    self.intervals[index].end = self.event();
    index
  }

  pub fn alloc(&mut self) -> Register {
    let index = self.intervals.len();
    let event = self.event();
    self.intervals.push(Interval {
      index,
      start: event,
      end: event,
    });
    Register(index)
  }

  pub fn finish(self) -> (usize, Vec<usize>) {
    linear_scan(&self.intervals)
  }
}

fn linear_scan(intervals: &[Interval]) -> (usize, Vec<usize>) {
  let mut mapping = Vec::with_capacity(intervals.len());
  mapping.fill(0usize);

  let mut free = BinaryHeap::new();
  let mut active = Active::new();
  let mut registers = 0usize;

  // TODO: aren't they already sorted by increasing start point?
  // intervals sorted in order of increasing start point
  let mut intervals_by_start = intervals.to_vec();
  intervals_by_start.sort_unstable_by(|a, b| a.start.cmp(&b.start));

  // foreach live interval i, in order of increasing start point
  for i in intervals_by_start.iter() {
    // expire old intervals
    expire_old_intervals(i, &mut free, &mut active);
    // Note: we never spill
    // register[i] ← a register removed from pool of free registers
    // Note: in our case, we either remove from the pool, or allocate a new one
    let register = allocate(&mut free, &mut registers);
    // add i to active, sorted by increasing end point
    // Note: we only do this to keep track of which registers are in use,
    //       because we do not need to perform spills
    active.map.insert(i.index, (*i, register));
    // in our case, we construct a mapping from intervals to final registers
    // this is later used to patch the bytecode
    mapping.insert(i.index, register);
  }

  (registers, mapping)
}

struct Active {
  map: HashMap<usize, (Interval, usize)>,
  scratch: Vec<Interval>,
}

impl Active {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
      scratch: Vec::new(),
    }
  }

  pub fn sort_by_end(&mut self) {
    self.scratch.clear();
    self.scratch.extend(self.map.values().map(|v| v.0));
    self.scratch.sort_unstable_by(|a, b| a.end.cmp(&b.end));
  }
}

fn allocate(free: &mut BinaryHeap<Reverse<usize>>, registers: &mut usize) -> usize {
  // attempt to acquire a free register, and fall back to allocating a new one
  if let Some(Reverse(reg)) = free.pop() {
    reg
  } else {
    let reg = *registers;
    *registers += 1;
    reg
  }
}

fn expire_old_intervals(i: &Interval, free: &mut BinaryHeap<Reverse<usize>>, active: &mut Active) {
  // TODO: is the sorting here actually necessary? if yes, use `binary_search`
  // into `Vec::insert` instead of keeping an extra hashmap.
  // Currently `sort_by_end` is `O(n + n * log(n))`, but the binary search
  // and insert would be `O(n + log(n))`, and would decrease space
  // complexity here by a full `n` required to store the hashmap.

  // foreach interval j in active, in order of increasing end point
  active.sort_by_end();
  for j in active.scratch.iter() {
    // if endpoint[j] ≥ startpoint[i] then
    if j.end >= i.start {
      // return
      return;
    }

    // remove j from active
    let register = active.map.remove(&j.index).unwrap().1;
    // add register[j] to pool of free registers
    free.push(Reverse(register));
  }
}
