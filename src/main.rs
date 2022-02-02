use std::{thread, time};

const ESC: char = 27 as char;

type Position = (isize, isize);

#[derive(Debug)]
struct Entities {
    wants: Vec<u8>,
    has: Vec<u8>,
    position: Vec<Position>,
    visible: Vec<bool>,
    upstream: Vec<Vec<usize>>,
    downstream: Vec<Vec<usize>>,
}

impl Entities {
    fn new() -> Self {
        Self {
            wants: Vec::with_capacity(1024),
            has: Vec::with_capacity(1024),
            position: Vec::with_capacity(1024),
            visible: Vec::with_capacity(1024),
            upstream: Vec::with_capacity(1024),
            downstream: Vec::with_capacity(1024),
        }
    }

    fn insert(&mut self, wants: u8, has: u8, position: (isize, isize), visible: bool) {
        let len = self.position.len();
        let mut upstream = Vec::new();
        let mut downstream = Vec::new();
        let (x, y) = position;
        for i in 0..len {
            if self.position[i] == (x, y - 1) || // up
               self.position[i] == (x - 1, y) {  // left
                   upstream.push(i);
                   self.downstream[i].push(len);
               }
            if self.position[i] == (x, y + 1) || // down
               self.position[i] == (x + 1, y) {  // right
                   downstream.push(i);
                   self.upstream[i].push(len);
            }
        }
        self.wants.push(wants);
        self.has.push(has);
        self.position.push(position);
        self.visible.push(visible);
        self.upstream.push(upstream);
        self.downstream.push(downstream);
    }

    fn display(&self) -> Vec<(Position, char)> {
        let len = self.position.len();
        let mut output = Vec::with_capacity(len);
        for i in 0..len {
            if self.visible[i] {
                //let repr = if self.has[i] < 128 { '*' } else { '!' };
                let repr: char = (48 + i as u8) as char;
                output.push((self.position[i], repr));
            }
        }
        output
    }

    fn debug_entity(&self, i: usize) {
        println!("Index: {}\tHas: {}\tWants:{}\tPosition: {:?}\tVisible: {:?}\tUpstream: {:?}\tDownstream: {:?}",
                 i,
                 self.has[i],
                 self.wants[i],
                 self.position[i],
                 self.visible[i],
                 self.upstream[i],
                 self.downstream[i]);
    }

    fn update(&mut self) {
        let len = self.position.len();
        for i in 0..len {
            for u in &self.upstream[i] {
                if self.has[i] != u8::MAX {
                    if self.has[*u] >= self.wants[i] {
                        self.has[i] = self.has[i].saturating_add(self.wants[i]);
                        self.has[*u] = self.has[*u].saturating_sub(self.wants[i]);
                    } else {
                        self.has[i] = self.has[i].saturating_add(self.has[*u]);
                        self.has[*u] = 0;
                    }
                }
            }
        }
    }
}

struct World {
    entities: Entities,
    size: (usize, usize),
    ticks_per_second: u32,
    tick_time: time::Duration,
    ticks: usize,
}

impl World {
    fn new() -> Self {
        Self {
            entities: Entities::new(),
            size: (64, 32),
            ticks_per_second: 4,
            tick_time: time::Duration::from_millis(1000 / 4),
            ticks: 1,
        }
    }

    fn display_border_top(&self) {
        print!("{ESC}[1;1H╔");
        for x in 2..(self.size.0 + 2) { print!("{ESC}[1;{x}H═"); }
        let x = self.size.0 + 2;
        print!("{ESC}[1;{x}H╗");
    }

    fn display_border_bottom(&self) {
        let y = self.size.1 + 2;
        print!("{ESC}[{y};1H╚");
        for x in 2..(self.size.0 + 2) { print!("{ESC}[{y};{x}H═"); }
        let x = self.size.0 + 2;
        print!("{ESC}[{y};{x}H╝");
    }

    fn display_border_sides(&self) {
        let x = self.size.0 + 2;
        for y in 2..(self.size.1 + 2) {
            print!("{ESC}[{y};1H║{ESC}[{y};{x}H║");
        }
    }

    fn display_clear(&self) {
        print!("{ESC}[2J");
    }

    fn display(&self) {
        let output = self.entities.display();
        self.display_clear();
        self.display_border_top();
        self.display_border_sides();

        for ((x, y), repr) in output {
            print!("{ESC}[{};{}H{repr}", y + 2, x + 2);
        }

        self.display_border_bottom();
        println!("");
    }

    fn update(&mut self) {
        for i in 0..self.entities.position.len() {
            self.entities.debug_entity(i);
        }
        self.entities.update();
    }

    fn tick(&mut self) {
        let tick_duration = time::Instant::now();
        self.display();
        self.update();
        let sleep_time = self.tick_time - tick_duration.elapsed();
        println!("Render time: {:?}\nFrame time: {:?}\nTarget frame time: {:?}\tTick #: {}",
                 tick_duration.elapsed(),
                 sleep_time + tick_duration.elapsed(),
                 self.tick_time,
                 self.ticks);
        thread::sleep(sleep_time);
        self.ticks += 1;
    }
}

fn setup_chain(world: &mut World) {
    world.entities.insert(1, 100, (1, 1), true);
    world.entities.insert(1, 255, (1, 2), true);
    world.entities.insert(2, 64, (2, 2), true);
    world.entities.insert(2, 192, (3, 2), true);
    world.entities.insert(5, 0, (3, 3), true);
}

fn main() {
    let mut world = World::new();
    setup_chain(&mut world);
    loop {
        world.tick();
    }
}
