use std::collections::{HashMap, VecDeque};

#[inline]
fn dist(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum AmphType {
    A,
    B,
    C,
    D,
}

const ALL_AMPH_TYPES: [AmphType; 4] = [AmphType::A, AmphType::B, AmphType::C, AmphType::D];

impl AmphType {
    fn id(self) -> usize {
        match self {
            AmphType::A => 0,
            AmphType::B => 1,
            AmphType::C => 2,
            AmphType::D => 3,
        }
    }

    fn move_cost(self) -> usize {
        match self {
            AmphType::A => 1,
            AmphType::B => 10,
            AmphType::C => 100,
            AmphType::D => 1000,
        }
    }

    fn next(self) -> Self {
        match self {
            AmphType::A => AmphType::B,
            AmphType::B => AmphType::C,
            AmphType::C => AmphType::D,
            AmphType::D => AmphType::A,
        }
    }

    fn prev(self) -> Self {
        match self {
            AmphType::A => AmphType::D,
            AmphType::B => AmphType::A,
            AmphType::C => AmphType::B,
            AmphType::D => AmphType::C,
        }
    }

    fn room_entry_distance(r1: AmphType, r2: AmphType) -> usize {
        2 * dist(r1.id(), r2.id())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Position {
    FarLeft,
    FarRight,
    LeftOf(AmphType),
    RightOfD,
    In(AmphType),
    DeepIn(AmphType),
    DeeperIn(AmphType),
    DeepestIn(AmphType),
}

const OUT_POSITIONS: [Position; Position::OUT_ID_COUNT] = [
    Position::FarLeft,
    Position::FarRight,
    Position::LeftOf(AmphType::A),
    Position::LeftOf(AmphType::B),
    Position::LeftOf(AmphType::C),
    Position::LeftOf(AmphType::D),
    Position::RightOfD,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Path {
    Left,
    Right,
    Up(AmphType),
}

impl Position {
    const ID_COUNT: usize = 23;
    const OUT_ID_COUNT: usize = 7;

    fn id(self) -> usize {
        match self {
            Position::FarLeft => 0,
            Position::FarRight => 1,
            Position::LeftOf(t) => 2 + t.id(),
            Position::RightOfD => 6,
            Position::In(t) => 7 + t.id(),
            Position::DeepIn(t) => 11 + t.id(),
            Position::DeeperIn(t) => 15 + t.id(),
            Position::DeepestIn(t) => 19 + t.id(),
        }
    }

    fn right_of(amph: AmphType) -> Self {
        match amph {
            AmphType::D => Position::RightOfD,
            a => Position::LeftOf(a.next()),
        }
    }

    fn closest_room_entry_and_distance(&self) -> [Option<(AmphType, usize, Path)>; 2] {
        match self {
            Position::FarLeft => [Some((AmphType::A, 2, Path::Right)), None],
            Position::FarRight => [Some((AmphType::D, 2, Path::Left)), None],
            Position::LeftOf(AmphType::A) => [Some((AmphType::A, 1, Path::Right)), None],
            Position::LeftOf(amph_type) => [
                Some((*amph_type, 1, Path::Right)),
                Some((amph_type.prev(), 1, Path::Left)),
            ],
            Position::RightOfD => [Some((AmphType::D, 1, Path::Left)), None],
            Position::In(amph_type) => [Some((*amph_type, 1, Path::Up(*amph_type))), None],
            Position::DeepIn(amph_type) => [Some((*amph_type, 2, Path::Up(*amph_type))), None],
            Position::DeeperIn(amph_type) => [Some((*amph_type, 3, Path::Up(*amph_type))), None],
            Position::DeepestIn(amph_type) => [Some((*amph_type, 4, Path::Up(*amph_type))), None],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Move {
    from: Position,
    to: Position,
}

impl Move {
    fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    #[allow(dead_code)]
    fn reversed(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
        }
    }

    fn distance(&self) -> usize {
        let mut min_dist = usize::MAX;
        for (from_entry, from_dist, from_path) in self
            .from
            .closest_room_entry_and_distance()
            .into_iter()
            .flatten()
        {
            for (to_entry, to_dist, to_path) in self
                .to
                .closest_room_entry_and_distance()
                .into_iter()
                .flatten()
            {
                let dist = AmphType::room_entry_distance(from_entry, to_entry)
                    + if from_path == to_path {
                        dist(from_dist, to_dist)
                    } else {
                        from_dist + to_dist
                    };
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
        min_dist
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct State {
    positions: [Option<AmphType>; Position::ID_COUNT],
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = |x: Option<AmphType>| match x {
            None => '.',
            Some(AmphType::A) => 'A',
            Some(AmphType::B) => 'B',
            Some(AmphType::C) => 'C',
            Some(AmphType::D) => 'D',
        };
        f.write_fmt(format_args!(
            "\n#############\n#{}{}.{}.{}.{}.{}{}#\n###{}#{}#{}#{}###\n  #{}#{}#{}#{}#\n  #{}#{}#{}#{}#\n  #{}#{}#{}#{}#\n  #########",
            fmt(self.get(Position::FarLeft)),
            fmt(self.get(Position::LeftOf(AmphType::A))),
            fmt(self.get(Position::LeftOf(AmphType::B))),
            fmt(self.get(Position::LeftOf(AmphType::C))),
            fmt(self.get(Position::LeftOf(AmphType::D))),
            fmt(self.get(Position::RightOfD)),
            fmt(self.get(Position::FarRight)),
            fmt(self.get(Position::In(AmphType::A))),
            fmt(self.get(Position::In(AmphType::B))),
            fmt(self.get(Position::In(AmphType::C))),
            fmt(self.get(Position::In(AmphType::D))),
            fmt(self.get(Position::DeepIn(AmphType::A))),
            fmt(self.get(Position::DeepIn(AmphType::B))),
            fmt(self.get(Position::DeepIn(AmphType::C))),
            fmt(self.get(Position::DeepIn(AmphType::D))),
            fmt(self.get(Position::DeeperIn(AmphType::A))),
            fmt(self.get(Position::DeeperIn(AmphType::B))),
            fmt(self.get(Position::DeeperIn(AmphType::C))),
            fmt(self.get(Position::DeeperIn(AmphType::D))),
            fmt(self.get(Position::DeepestIn(AmphType::A))),
            fmt(self.get(Position::DeepestIn(AmphType::B))),
            fmt(self.get(Position::DeepestIn(AmphType::C))),
            fmt(self.get(Position::DeepestIn(AmphType::D))),
        ))
    }
}

impl State {
    fn new_shallow(in_amphs: [AmphType; 4], deep_in_amphs: [AmphType; 4]) -> Self {
        Self::new_extended(in_amphs, deep_in_amphs, ALL_AMPH_TYPES, ALL_AMPH_TYPES)
    }

    fn new_standard_extended(in_amphs: [AmphType; 4], deepest_in_amphs: [AmphType; 4]) -> Self {
        Self::new_extended(
            in_amphs,
            [AmphType::D, AmphType::C, AmphType::B, AmphType::A],
            [AmphType::D, AmphType::B, AmphType::A, AmphType::C],
            deepest_in_amphs,
        )
    }

    fn new_extended(
        in_amphs: [AmphType; 4],
        deep_in_amphs: [AmphType; 4],
        deeper_in_amphs: [AmphType; 4],
        deepest_in_amphs: [AmphType; 4],
    ) -> Self {
        let mut positions: [Option<AmphType>; Position::ID_COUNT] = Default::default();
        let mut counts = [0; 4];
        for (pos_amph_type, in_amph_type) in ALL_AMPH_TYPES.iter().zip(in_amphs) {
            positions[Position::In(*pos_amph_type).id()] = Some(in_amph_type);
            counts[in_amph_type.id()] += 1;
        }
        for (pos_amph_type, in_amph_type) in ALL_AMPH_TYPES.iter().zip(deep_in_amphs) {
            positions[Position::DeepIn(*pos_amph_type).id()] = Some(in_amph_type);
            counts[in_amph_type.id()] += 1;
        }
        for (pos_amph_type, in_amph_type) in ALL_AMPH_TYPES.iter().zip(deeper_in_amphs) {
            positions[Position::DeeperIn(*pos_amph_type).id()] = Some(in_amph_type);
            counts[in_amph_type.id()] += 1;
        }
        for (pos_amph_type, in_amph_type) in ALL_AMPH_TYPES.iter().zip(deepest_in_amphs) {
            positions[Position::DeepestIn(*pos_amph_type).id()] = Some(in_amph_type);
            counts[in_amph_type.id()] += 1;
        }
        assert_eq!(counts, [4, 4, 4, 4]);
        Self { positions }
    }

    #[allow(dead_code)]
    fn moved(&self, mv: Move) -> Self {
        self.with_move_applied(mv).1
    }

    fn with_move_applied(&self, mv: Move) -> (usize, Self) {
        let mut positions = self.positions;
        let amph = positions[mv.from.id()]
            .take()
            .expect("Can't move without amph");
        assert!(positions[mv.to.id()].is_none());
        positions[mv.to.id()] = Some(amph);
        (mv.distance() * amph.move_cost(), Self { positions })
    }

    fn is_solved(&self) -> bool {
        for amph_type in ALL_AMPH_TYPES {
            if self.positions[Position::In(amph_type).id()] != Some(amph_type) {
                return false;
            }
            if self.positions[Position::DeepIn(amph_type).id()] != Some(amph_type) {
                return false;
            }
            if self.positions[Position::DeeperIn(amph_type).id()] != Some(amph_type) {
                return false;
            }
            if self.positions[Position::DeepestIn(amph_type).id()] != Some(amph_type) {
                return false;
            }
        }
        true
    }

    fn get(&self, position: Position) -> Option<AmphType> {
        self.positions[position.id()]
    }

    fn is_empty(&self, position: Position) -> bool {
        self.get(position).is_none()
    }

    /// true if room has only its proper type inside (or is empty)
    fn is_room_ordered(&self, room: AmphType) -> bool {
        for position in [
            Position::In(room),
            Position::DeepIn(room),
            Position::DeeperIn(room),
            Position::DeepestIn(room),
        ] {
            if let Some(amph) = self.get(position) {
                if amph != room {
                    return false;
                }
            }
        }
        true
    }

    /// room_availability[amph_id] == possible amph_id room position to occupy
    fn room_availability(&self) -> [Option<Position>; 4] {
        let mut room_availability: [Option<Position>; 4] = Default::default();
        for room in ALL_AMPH_TYPES {
            if self.is_room_ordered(room) {
                for position in [
                    Position::DeepestIn(room),
                    Position::DeeperIn(room),
                    Position::DeepIn(room),
                    Position::In(room),
                ] {
                    if self.is_empty(position) {
                        room_availability[room.id()] = Some(position);
                        break;
                    }
                }
            }
        }
        room_availability
    }

    /// room_exits[amph_id] == amph_id room position that can exit that room
    fn room_exits(&self) -> [Option<Position>; 4] {
        let mut exits: [Option<Position>; 4] = Default::default();
        for amph in ALL_AMPH_TYPES {
            exits[amph.id()] = if self.is_room_ordered(amph) {
                None
            } else if !self.is_empty(Position::In(amph)) {
                Some(Position::In(amph))
            } else if !self.is_empty(Position::DeepIn(amph)) {
                Some(Position::DeepIn(amph))
            } else if !self.is_empty(Position::DeeperIn(amph)) {
                Some(Position::DeeperIn(amph))
            } else if !self.is_empty(Position::DeepestIn(amph)) {
                Some(Position::DeepestIn(amph))
            } else {
                None
            }
        }
        exits
    }

    /// room_reachability[out_position_id][amph_id] == room amph_id is reachable from out_position_id
    fn room_reachability(&self) -> [[bool; 4]; Position::OUT_ID_COUNT] {
        let mut reachability: [[bool; 4]; Position::OUT_ID_COUNT] = Default::default();

        for room in ALL_AMPH_TYPES {
            // LeftOf can always reach its room
            reachability[Position::LeftOf(room).id()][room.id()] = true;
        }

        for room in [AmphType::B, AmphType::C, AmphType::D] {
            // LeftOf can always reach the room on the other side
            reachability[Position::LeftOf(room).id()][room.prev().id()] = true;
        }

        // RightOfD can always reach room D
        reachability[Position::RightOfD.id()][AmphType::D.id()] = true;

        // too lazy to get all connectivity now (would need some drawing) so for now we just try iterating a few more times to find it all
        for (position, passes_through) in [
            (Position::LeftOf(AmphType::A), Position::LeftOf(AmphType::B)),
            (Position::LeftOf(AmphType::B), Position::LeftOf(AmphType::C)),
            (Position::LeftOf(AmphType::C), Position::LeftOf(AmphType::D)),
            (Position::LeftOf(AmphType::D), Position::LeftOf(AmphType::C)),
            (Position::LeftOf(AmphType::C), Position::LeftOf(AmphType::B)),
            (Position::LeftOf(AmphType::A), Position::LeftOf(AmphType::B)),
            (Position::LeftOf(AmphType::B), Position::LeftOf(AmphType::C)),
            (Position::LeftOf(AmphType::C), Position::LeftOf(AmphType::D)),
            (Position::LeftOf(AmphType::D), Position::LeftOf(AmphType::C)),
            (Position::LeftOf(AmphType::C), Position::LeftOf(AmphType::B)),
            (Position::LeftOf(AmphType::A), Position::LeftOf(AmphType::B)),
            (Position::LeftOf(AmphType::B), Position::LeftOf(AmphType::C)),
            (Position::LeftOf(AmphType::C), Position::LeftOf(AmphType::D)),
            (Position::LeftOf(AmphType::D), Position::LeftOf(AmphType::C)),
            (Position::LeftOf(AmphType::C), Position::LeftOf(AmphType::B)),
            (Position::FarLeft, Position::LeftOf(AmphType::A)),
            (Position::RightOfD, Position::LeftOf(AmphType::D)),
            (Position::FarRight, Position::RightOfD),
        ] {
            if self.is_empty(passes_through) {
                // passes_through is empty so position can reach a room if passes_through can reach it
                let others_rooms = reachability[passes_through.id()];
                for (this, others) in reachability[position.id()].iter_mut().zip(others_rooms) {
                    *this = *this || others;
                }
            }
        }

        reachability
    }

    // this could be a proper iter but I'm too lazy for that
    fn iter_possible_moves<F>(&self, mut process_move: F)
    where
        F: FnMut(Move),
    {
        let room_availability = self.room_availability();
        let room_exits = self.room_exits();
        let room_reachability = self.room_reachability();

        // from all out positions occuppied by an amph to their room (if available)
        for out_position in OUT_POSITIONS {
            if let Some(amph) = self.get(out_position) {
                if let Some(target_room_position) = room_availability[amph.id()] {
                    if room_reachability[out_position.id()][amph.id()] {
                        process_move(Move::new(out_position, target_room_position));
                    }
                }
            }
        }

        // from inside all rooms to all other rooms
        for room in ALL_AMPH_TYPES {
            if let Some(source_room_position) = room_exits[room.id()] {
                if let Some(amph) = self.get(source_room_position) {
                    if let Some(target_room_position) = room_availability[amph.id()] {
                        if room_reachability[Position::LeftOf(room).id()][amph.id()]
                            && room_reachability[Position::right_of(room).id()][amph.id()]
                        {
                            process_move(Move::new(source_room_position, target_room_position));
                        }
                    }
                }
            }
        }

        // from all rooms to all reachable (and free) out positions
        for room in ALL_AMPH_TYPES {
            if let Some(source_room_position) = room_exits[room.id()] {
                for out_position in OUT_POSITIONS {
                    if self.is_empty(out_position)
                        && room_reachability[out_position.id()][room.id()]
                    {
                        process_move(Move::new(source_room_position, out_position));
                    }
                }
            }
        }
    }
}

fn find_least_energy(initial_state: &State) -> usize {
    find_least_energy_debug(initial_state).0
}

fn find_least_energy_debug(initial_state: &State) -> (usize, HashMap<State, Move>) {
    let mut queue = VecDeque::new();
    let mut costs = HashMap::new();
    let mut by_move = HashMap::new();
    let mut lowest_solved_cost = None;
    queue.push_back(*initial_state);
    costs.insert(*initial_state, 0);
    while let Some(state) = queue.pop_front() {
        let base_cost = *costs.get(&state).expect("Should have move cost");
        state.iter_possible_moves(|possible_move| {
            let (move_cost, new_state) = state.with_move_applied(possible_move);
            let new_cost = base_cost + move_cost;
            if lowest_solved_cost
                .map(|solved_cost| new_cost < solved_cost)
                .unwrap_or(true)
                && costs
                    .get(&new_state)
                    .map(|cost| new_cost < *cost)
                    .unwrap_or(true)
            {
                costs.insert(new_state, new_cost);
                by_move.insert(new_state, possible_move);
                if new_state.is_solved() {
                    lowest_solved_cost = Some(new_cost);
                } else {
                    queue.push_back(new_state);
                }
            }
        });
    }
    (
        lowest_solved_cost.expect("Should have found a solution by now"),
        by_move,
    )
}

fn input_state() -> State {
    State::new_shallow(
        [AmphType::D, AmphType::A, AmphType::D, AmphType::C],
        [AmphType::B, AmphType::C, AmphType::B, AmphType::A],
    )
}

fn input_state_extended() -> State {
    State::new_standard_extended(
        [AmphType::D, AmphType::A, AmphType::D, AmphType::C],
        [AmphType::B, AmphType::C, AmphType::B, AmphType::A],
    )
}

fn main() {
    println!("Part 1: {}", find_least_energy(&input_state()));
    println!("Part 2: {}", find_least_energy(&input_state_extended()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solved_state() -> State {
        State::new_shallow(ALL_AMPH_TYPES.clone(), ALL_AMPH_TYPES.clone())
    }

    fn sample_state() -> State {
        State::new_shallow(
            [AmphType::B, AmphType::C, AmphType::B, AmphType::D],
            [AmphType::A, AmphType::D, AmphType::C, AmphType::A],
        )
    }

    fn sample_extended_state() -> State {
        State::new_standard_extended(
            [AmphType::B, AmphType::C, AmphType::B, AmphType::D],
            [AmphType::A, AmphType::D, AmphType::C, AmphType::A],
        )
    }

    #[test]
    fn test_solved_sanity() {
        assert!(solved_state().is_solved());
    }

    #[test]
    fn test_distance() {
        assert_eq!(
            Move::new(Position::LeftOf(AmphType::D), Position::RightOfD).distance(),
            2
        );
        assert_eq!(
            Move::new(Position::LeftOf(AmphType::D), Position::LeftOf(AmphType::B)).distance(),
            4
        );
        assert_eq!(
            Move::new(Position::In(AmphType::D), Position::DeepIn(AmphType::B)).distance(),
            7
        );
        assert_eq!(
            Move::new(Position::In(AmphType::C), Position::LeftOf(AmphType::B)).distance(),
            4
        );
        assert_eq!(
            Move::new(Position::DeepIn(AmphType::B), Position::LeftOf(AmphType::C)).distance(),
            3
        );
    }

    #[test]
    fn test_move() {
        let (_, state) = solved_state().with_move_applied(Move::new(
            Position::In(AmphType::A),
            Position::LeftOf(AmphType::C),
        ));
        assert_eq!(state.get(Position::In(AmphType::A)), None);
        assert_eq!(state.get(Position::LeftOf(AmphType::C)), Some(AmphType::A));
        assert_eq!(state.is_empty(Position::In(AmphType::A)), true);
        assert_eq!(state.is_empty(Position::LeftOf(AmphType::C)), false);
    }

    #[test]
    fn test_solved_reachability() {
        let reachability = solved_state().room_reachability();
        // #############
        // #...........#
        // ###A#B#C#D###
        //   #A#B#C#D#
        //   #########
        for position in OUT_POSITIONS {
            for room in ALL_AMPH_TYPES {
                assert!(
                    reachability[position.id()][room.id()],
                    "position = {:?}, room = {:?}",
                    position,
                    room
                );
            }
        }
    }

    #[test]
    fn test_reachability() {
        let (_, state) = solved_state().with_move_applied(Move::new(
            Position::In(AmphType::A),
            Position::LeftOf(AmphType::C),
        ));
        let reachability = state.room_reachability();
        // #############
        // #.....A.....#
        // ###.#B#C#D###
        //   #A#B#C#D#
        //   #########
        for position in [Position::LeftOf(AmphType::C)] {
            for room in ALL_AMPH_TYPES {
                assert!(
                    reachability[position.id()][room.id()],
                    "position = {:?}, room = {:?}",
                    position,
                    room
                );
            }
        }
        for position in [
            Position::FarLeft,
            Position::LeftOf(AmphType::A),
            Position::LeftOf(AmphType::B),
        ] {
            for room in [AmphType::A, AmphType::B] {
                assert!(
                    reachability[position.id()][room.id()],
                    "position = {:?}, room = {:?}",
                    position,
                    room
                );
            }
            for room in [AmphType::C, AmphType::D] {
                assert!(
                    !reachability[position.id()][room.id()],
                    "position = {:?}, room = {:?}",
                    position,
                    room
                );
            }
        }
        for position in [
            Position::FarRight,
            Position::LeftOf(AmphType::D),
            Position::RightOfD,
        ] {
            for room in [AmphType::A, AmphType::B] {
                assert!(
                    !reachability[position.id()][room.id()],
                    "position = {:?}, room = {:?}",
                    position,
                    room
                );
            }
            for room in [AmphType::C, AmphType::D] {
                assert!(
                    reachability[position.id()][room.id()],
                    "position = {:?}, room = {:?}",
                    position,
                    room
                );
            }
        }
    }

    fn is_in_possible_moves(state: &State, mv: Move) -> bool {
        let mut found = false;
        state.iter_possible_moves(|possible_move| found = found || mv == possible_move);
        found
    }

    #[test]
    fn test_find_least_energy_sample_details() {
        let mut state = sample_state();
        let expected_moves_with_energy = vec![
            // #############
            // #...........#
            // ###B#C#B#D###
            //   #A#D#C#A#
            //   #########
            (
                Move::new(Position::In(AmphType::C), Position::LeftOf(AmphType::B)),
                40,
            ),
            // #############
            // #...B.......#
            // ###B#C#.#D###
            //   #A#D#C#A#
            //   #########
            (
                Move::new(Position::In(AmphType::B), Position::In(AmphType::C)),
                400,
            ),
            // #############
            // #...B.......#
            // ###B#.#C#D###
            //   #A#D#C#A#
            //   #########
            (
                Move::new(Position::DeepIn(AmphType::B), Position::LeftOf(AmphType::C)),
                3000,
            ),
            // #############
            // #...B.D.....#
            // ###B#.#C#D###
            //   #A#.#C#A#
            //   #########
            (
                Move::new(Position::LeftOf(AmphType::B), Position::DeepIn(AmphType::B)),
                30,
            ),
            // #############
            // #.....D.....#
            // ###B#.#C#D###
            //   #A#B#C#A#
            //   #########
            (
                Move::new(Position::In(AmphType::A), Position::In(AmphType::B)),
                40,
            ),
            // #############
            // #.....D.....#
            // ###.#B#C#D###
            //   #A#B#C#A#
            //   #########
            (
                Move::new(Position::In(AmphType::D), Position::LeftOf(AmphType::D)),
                2000,
            ),
            // #############
            // #.....D.D...#
            // ###.#B#C#.###
            //   #A#B#C#A#
            //   #########
            (
                Move::new(Position::DeepIn(AmphType::D), Position::RightOfD),
                3,
            ),
            // #############
            // #.....D.D.A.#
            // ###.#B#C#.###
            //   #A#B#C#.#
            //   #########
            (
                Move::new(Position::LeftOf(AmphType::D), Position::DeepIn(AmphType::D)),
                3000,
            ),
            // #############
            // #.....D...A.#
            // ###.#B#C#.###
            //   #A#B#C#D#
            //   #########
            (
                Move::new(Position::LeftOf(AmphType::C), Position::In(AmphType::D)),
                4000,
            ),
            // #############
            // #.........A.#
            // ###.#B#C#D###
            //   #A#B#C#D#
            //   #########
            (Move::new(Position::RightOfD, Position::In(AmphType::A)), 8),
            // #############
            // #...........#
            // ###A#B#C#D###
            //   #A#B#C#D#
            //   #########
        ];
        for (mv, cost) in expected_moves_with_energy.iter() {
            assert!(
                is_in_possible_moves(&state, *mv),
                "not in possible moves: {:?}",
                mv
            );
            let (new_cost, new_state) = state.with_move_applied(*mv);
            assert_eq!(new_cost, *cost);
            state = new_state;
        }
        assert!(state.is_solved());
        assert_eq!(
            expected_moves_with_energy
                .iter()
                .map(|(_, cost)| *cost)
                .sum::<usize>(),
            12521
        );
    }

    #[test]
    #[ignore]
    fn test_find_least_energy_sample() {
        assert_eq!(find_least_energy(&sample_state()), 12521);
    }

    #[test]
    #[ignore]
    fn test_find_least_energy_sample_extended() {
        assert_eq!(find_least_energy(&sample_extended_state()), 44169);
    }

    #[test]
    #[ignore]
    fn test_find_least_energy_sample_debug() {
        let initial_state = sample_state();
        let (energy, by_move) = find_least_energy_debug(&initial_state);
        let mut state = solved_state();
        dbg!(state);
        while let Some(mv) = by_move.get(&state) {
            let mv = *mv;
            dbg!(mv);
            state = state.with_move_applied(mv.reversed()).1;
            dbg!(state);
        }
        assert_eq!(energy, 12521);
    }

    #[test]
    fn test_possible_moves_issue() {
        let state = State::new_shallow(
            [AmphType::D, AmphType::B, AmphType::C, AmphType::D],
            [AmphType::A, AmphType::B, AmphType::C, AmphType::A],
        )
        .moved(Move::new(
            Position::In(AmphType::A),
            Position::LeftOf(AmphType::D),
        ))
        .moved(Move::new(Position::In(AmphType::D), Position::RightOfD));
        assert!(!is_in_possible_moves(
            &state,
            Move::new(Position::DeepIn(AmphType::D), Position::In(AmphType::A))
        ));
    }

    #[test]
    #[ignore]
    fn test_solved_part1() {
        assert_eq!(find_least_energy(&input_state()), 14348);
    }
}
