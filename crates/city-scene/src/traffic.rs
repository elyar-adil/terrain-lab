//! Traffic: signal timing, route planning and a deterministic agent fleet.
//!
//! The project had **no traffic at all** — `urban::model::traffic` emitted a
//! static phase programme that nothing ever stepped.  This module is the whole
//! simulation: vehicles follow the connector graph, obey the signal clock, keep
//! a headway on the car in front, and stop for the signal at the end of a lane.
//!
//! It lives in Rust rather than in the renderer for one reason that matters: a
//! vehicle's route is derived from the *same* lane and connector graph the road
//! surface was drawn from.  A renderer-side simulation would have to re-derive
//! that graph from geometry and would eventually disagree with the paint on the
//! road.
//!
//! # Routes cross junctions, not lanes
//!
//! The load-bearing idea here is that a route is a sequence of **elements** —
//! lane, connector, lane, connector — not a sequence of lanes.  A route made of
//! lanes alone has to teleport from one carriageway to the next at every
//! junction, because two lanes meeting at a node are not physically joined; the
//! *connector* between them is.  Walking lanes only also means a vehicle drives
//! along the wrong path through every turn, which is precisely the junction
//! behaviour the whole port is about.
//!
//! # Model
//!
//! A rule model, documented as such: car-following with a lane-width gate,
//! signal obedience, curvature-limited corner speeds, and right-hand priority at
//! the stop line.  No lane-changing, no pedestrian interaction, no
//! traffic-capacity solution.

use std::collections::BinaryHeap;

use serde::Serialize;

use crate::math::{Path, Rng, Vec2};
use crate::network::{LaneUse, Network};
use crate::spec::Movement;
use crate::street::SignalRig;

/// Signal cycle length, seconds.  One clock drives the lamps, the vehicles and
/// any camera the renderer attaches, so a light can never disagree with the car
/// that stopped for it.
pub const CYCLE_SECONDS: f32 = 16.0;

/// Vehicle envelope, metres.
pub const CAR_LENGTH: f32 = 4.5;
pub const CAR_WIDTH: f32 = 1.85;
/// Stand-off gap kept from the car in front.
pub const MIN_GAP: f32 = 1.8;

const MAX_SPEED_RANGE: (f32, f32) = (8.5, 13.0);
const ACCEL_RANGE: (f32, f32) = (2.0, 2.8);
const BRAKE: f32 = 4.2;
const LATERAL_ACCEL: f32 = 2.2;
/// Speed allowed inside a junction box, where turning radii and pedestrians are.
const BOX_SPEED: f32 = 4.5;
/// Look-ahead distances for the curvature-based corner speed cap.
const LOOK_AHEAD: [f32; 5] = [0.0, 8.0, 18.0, 30.0, 44.0];
/// How long a vehicle may sit still before the graph is presumed deadlocked.
const DEADLOCK_SECONDS: f32 = 45.0;

/// Which aspect a signal shows at cycle time `t` for a given axis.
pub fn aspect_at(time: f32, axis: &str) -> u8 {
    let phase = time.rem_euclid(CYCLE_SECONDS);
    let (green_from, green_to, yellow_from) = if axis == "ns" {
        (0.0, 6.0, 6.0)
    } else {
        (8.0, 14.0, 14.0)
    };
    if phase >= green_from && phase < green_to {
        2
    } else if phase >= yellow_from && phase < yellow_from + 1.0 {
        1
    } else {
        0
    }
}

/// One piece of a route: a lane, or the connector that joins two lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Lane(usize),
    Connector(usize),
}

#[derive(Debug, Clone)]
struct RoutePiece {
    element: Element,
    /// Route arc length at which this piece starts, so `s` is global.
    start_s: f32,
    length: f32,
    /// Distance to the stop line at the *end* of this piece, in route arc
    /// length.  Zero when the piece does not end at a signalised junction.
    stop_line: f32,
    /// Signal axis at the end of this piece: 0 = north-south, 1 = east-west.
    axis: u8,
}

/// The yaw a renderer must give a body whose local `+X` axis should point along
/// `direction`, in the **three.js `rotation.y` convention**.
///
/// # Why this is not `Vec2::angle()`
///
/// three.js maps a local `+X` axis to `(cos y, 0, -sin y)` in the `(x, z)`
/// ground plane, so the yaw is `atan2(-z, x)` while a plan bearing is
/// `atan2(z, x)`.  The two differ by the sign of the `z` component: a car on a
/// north-south street placed with the bearing and drawn with the yaw is driving
/// **backwards**, and on a diagonal it is out by twice the error.  The renderer
/// writes `dummy.rotation.set(0, y, 0)` for both traffic and furniture, so the
/// payload states the convention once and every writer here obeys it.
pub fn heading_for(direction: Vec2) -> f32 {
    (-direction.y).atan2(direction.x)
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPose {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    /// Yaw in radians, `heading_for` convention: a local `+X` axis maps to
    /// `(cos heading, 0, -sin heading)`.
    pub heading: f32,
    pub speed: f32,
    /// Tint index into the renderer's car palette.
    pub colour: u8,
    /// `true` while the vehicle is stopped at a signal or in a queue.
    pub stopped: bool,
}

impl AgentPose {
    fn plan(&self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }
}

#[derive(Debug, Clone)]
pub struct Agent {
    route: Vec<RoutePiece>,
    piece: usize,
    /// Arc length along the current route.
    s: f32,
    speed: f32,
    max_speed: f32,
    accel: f32,
    colour: u8,
    blocked_for: f32,
    /// Last computed world pose, kept so spawn separation can test real
    /// positions rather than re-deriving them.
    pose: AgentPose,
    /// Per-agent stream for the tie-breaks the router needs.
    rng: u32,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficState {
    /// Cycle time in seconds, so the renderer can phase its own signals.
    pub time: f32,
    pub agents: Vec<AgentPose>,
    /// Lit aspect per signal rig: 0 red, 1 yellow, 2 green.
    pub aspects: Vec<u8>,
    /// Vehicles that have been stationary long enough to indicate a graph
    /// deadlock, reported rather than hidden.
    pub stalled: Vec<String>,
}

/// One agent's committed movement for a step.
#[derive(Debug, Clone, Copy)]
struct Decision {
    s: f32,
    speed: f32,
    blocked_for: f32,
}

/// A live traffic simulation over one city's lane graph.
pub struct TrafficSim {
    network: Network,
    agents: Vec<Agent>,
    time: f32,
    /// Connector indices grouped by the lane they leave.
    successors: Vec<Vec<(usize, usize)>>,
    /// Connector indices grouped by the lane they enter.
    predecessors: Vec<Vec<(usize, usize)>>,
    signals: Vec<SignalRig>,
    /// Connector cost from lane → lane, for the router.
    adjacency: Vec<Vec<(usize, usize)>>,
}

impl TrafficSim {
    /// Build a fleet.  Agents are spread over the network and given long
    /// routes, so a city reads as busy rather than as a single convoy.
    pub fn new(network: &Network, signals: Vec<SignalRig>, seed: u32, count: usize) -> Self {
        let lane_count = network.lanes.len();
        let mut successors: Vec<Vec<(usize, usize)>> = vec![Vec::new(); lane_count];
        let mut predecessors: Vec<Vec<(usize, usize)>> = vec![Vec::new(); lane_count];
        let mut lane_by_id = std::collections::HashMap::new();
        for (index, lane) in network.lanes.iter().enumerate() {
            lane_by_id.insert(lane.id.as_str(), index);
        }
        for (connector_index, connector) in network.connectors.iter().enumerate() {
            let (Some(from), Some(to)) = (
                lane_by_id.get(connector.from_lane.as_str()).copied(),
                lane_by_id.get(connector.to_lane.as_str()).copied(),
            ) else {
                continue;
            };
            successors[from].push((connector_index, to));
            predecessors[to].push((connector_index, from));
        }
        // Same `(connector, lane)` order as `successors`, so the relaxation loop
        // and the route expansion cannot disagree about which index is which.
        let adjacency: Vec<Vec<(usize, usize)>> = successors.clone();
        let mut sim = Self {
            network: network.clone(),
            agents: Vec::new(),
            time: 0.0,
            successors,
            predecessors,
            signals,
            adjacency,
        };
        let mut rng = Rng::new(seed ^ 0x7ea11c);
        let drivable: Vec<usize> = (0..lane_count)
            .filter(|index| {
                !matches!(
                    network.lanes[*index].use_kind,
                    LaneUse::Bike | LaneUse::Shoulder
                )
            })
            .collect();
        if drivable.is_empty() {
            return sim;
        }
        let placed = count.min(drivable.len() * 2);
        let mut tries = 0;
        while sim.agents.len() < placed && tries < placed * 40 {
            tries += 1;
            let start = drivable[rng.int(drivable.len() as u32) as usize];
            let Some(route) = sim.plan(start, 0x51ed_270b ^ (tries as u32)) else {
                continue;
            };
            // Reject a spawn that lands on top of an existing vehicle: the
            // follow logic would filter the pair as "self" and the overlap
            // would never resolve.
            let pose = sim.pose_at(&route, 0, 0.0, 0.0);
            let pose_plan = pose.plan();
            if sim
                .agents
                .iter()
                .any(|agent| pose_plan.distance(agent.pose.plan()) < 12.0)
            {
                continue;
            }
            let index = sim.agents.len() as u32;
            sim.agents.push(Agent {
                route,
                piece: 0,
                s: rng.range(0.0, 6.0),
                speed: 0.0,
                max_speed: rng.range(MAX_SPEED_RANGE.0, MAX_SPEED_RANGE.1),
                accel: rng.range(ACCEL_RANGE.0, ACCEL_RANGE.1),
                colour: rng.int(8) as u8,
                blocked_for: 0.0,
                pose,
                rng: 0x9e37_79b9u32.wrapping_mul(index + 1) ^ 0x1234_5678,
            });
        }
        sim
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn signals(&self) -> &[SignalRig] {
        &self.signals
    }

    pub fn network(&self) -> &Network {
        &self.network
    }

    /// A read-only view of the fleet, for tests and debugging tools.
    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }

    /// How many lanes lead into each agent's current lane, for diagnostics.
    pub fn predecessor_count(&self, index: usize) -> usize {
        self.agents
            .get(index)
            .and_then(|agent| agent.route.get(agent.piece))
            .map(|piece| match piece.element {
                Element::Lane(lane) => self.predecessors.get(lane).map(Vec::len).unwrap_or(0),
                Element::Connector(_) => 1,
            })
            .unwrap_or(0)
    }

    /// Connector indices leaving a lane, for tests and debugging tools.
    pub fn successors_of(&self, lane: usize) -> usize {
        self.successors.get(lane).map(Vec::len).unwrap_or(0)
    }

    /// Plan a route from a lane and report its piece count, for tests and
    /// debugging tools.
    pub fn try_plan(&self, lane: usize) -> Option<usize> {
        self.plan(lane, 1).map(|route| route.len())
    }

    /// Where an agent is in its route, for tests and debugging tools.
    pub fn agent_state(&self, index: usize) -> (usize, f32, usize, f32) {
        match self.agents.get(index) {
            Some(agent) => (
                agent.piece,
                agent.s,
                agent.route.len(),
                agent.speed,
            ),
            None => (0, 0.0, 0, 0.0),
        }
    }

    /// Describe one agent's route, for tests and debugging tools.
    pub fn describe_route(&self, index: usize) -> String {
        let Some(agent) = self.agents.get(index) else {
            return "no agent".into();
        };
        let mut text = format!("piece {} of {}\n", agent.piece, agent.route.len());
        for (order, piece) in agent.route.iter().enumerate() {
            let kind = match piece.element {
                Element::Lane(_) => "lane",
                Element::Connector(_) => "conn",
            };
            text.push_str(&format!(
                "  {order:>3} {kind} start {:.2} len {:.2}\n",
                piece.start_s, piece.length
            ));
        }
        text
    }

    fn path_of(&self, element: Element) -> &Path {
        match element {
            Element::Lane(index) => &self.network.lanes[index].path,
            Element::Connector(index) => &self.network.connectors[index].path,
        }
    }

    /// Advance the simulation and return every vehicle's pose.
    ///
    /// Three passes, and the split is deliberate: deciding a vehicle's speed
    /// needs an immutable view of the whole fleet (to find the leader and read
    /// the signal clock), while committing the result needs a mutable one.  In
    /// one loop each vehicle would see its neighbour's *new* position, and a
    /// queue would ripple a wave down itself every step.
    pub fn step(&mut self, dt: f32) -> TrafficState {
        // Clamp the step: a background tab or a long frame must not teleport a
        // queue through a junction.
        let dt = dt.clamp(0.0, 0.1);
        self.time += dt;
        let decisions: Vec<Decision> = (0..self.agents.len())
            .map(|index| self.decide(index, dt))
            .collect();
        for (index, decision) in decisions.iter().enumerate() {
            self.agents[index].s = decision.s;
            self.agents[index].speed = decision.speed;
            self.agents[index].blocked_for = decision.blocked_for;
        }
        for index in 0..self.agents.len() {
            self.roll_over(index);
        }
        let aspects = self
            .signals
            .iter()
            .map(|rig| aspect_at(self.time, rig.axis))
            .collect();
        let mut stalled = Vec::new();
        let agents = (0..self.agents.len())
            .map(|index| {
                let mut pose = self.pose_of(index);
                pose.stopped = pose.speed < 0.2;
                if self.agents[index].blocked_for > DEADLOCK_SECONDS {
                    stalled.push(format!("vehicle {index}"));
                }
                self.agents[index].pose = pose.clone();
                pose
            })
            .collect();
        TrafficState {
            time: self.time,
            agents,
            aspects,
            stalled,
        }
    }

    /// The pose of one agent at an arbitrary route arc length.
    fn pose_at(&self, route: &[RoutePiece], piece: usize, s: f32, speed: f32) -> AgentPose {
        let Some(entry) = route.get(piece.min(route.len().saturating_sub(1))) else {
            return AgentPose::default();
        };
        let local = (s - entry.start_s).clamp(0.0, entry.length);
        let (position, tangent) = self.path_of(entry.element).sample(local);
        AgentPose {
            x: position.x,
            y: position.y,
            z: position.z,
            heading: heading_for(tangent),
            speed,
            colour: 0,
            stopped: speed < 0.2,
        }
    }

    fn pose_of(&self, index: usize) -> AgentPose {
        let agent = &self.agents[index];
        let pose = self.pose_at(&agent.route, agent.piece, agent.s, agent.speed);
        AgentPose {
            colour: agent.colour,
            ..pose
        }
    }

    /// The stop line at the end of the current piece, if the vehicle must obey a
    /// signal there.
    fn signal_gap(&self, agent: &Agent) -> Option<f32> {
        let entry = agent.route.get(agent.piece)?;
        if entry.stop_line <= 0.0 {
            return None;
        }
        let remaining = entry.stop_line - agent.s;
        if remaining <= 0.0 {
            return None;
        }
        let aspect = aspect_at(self.time, if entry.axis == 0 { "ns" } else { "ew" });
        match aspect {
            // Red always wins.
            0 => Some(remaining),
            // Yellow is only crossed if the vehicle physically cannot stop
            // comfortably; otherwise it slows for the line like any red.
            1 => {
                if remaining < agent.speed * agent.speed / (2.0 * BRAKE) {
                    None
                } else {
                    Some(remaining)
                }
            }
            _ => None,
        }
    }

    fn decide(&self, index: usize, dt: f32) -> Decision {
        let agent = &self.agents[index];
        let mut s = agent.s;
        let speed = agent.speed;
        let mut target = agent.max_speed;

        // --- junction box and corner curvature --------------------------------
        // Speed is capped by the tightest curvature within the look-ahead window
        // and by a flat cap while the vehicle is inside a junction.
        for look in LOOK_AHEAD {
            let ahead = s + look;
            let Some(entry) = agent.route.get(agent.piece) else {
                break;
            };
            let local = ahead - entry.start_s;
            if local > entry.length {
                break;
            }
            let curvature = self.curvature_of(entry.element, local);
            if curvature > 1.0e-4 {
                let corner = (LATERAL_ACCEL / curvature).sqrt();
                target = target.min((corner * corner + 2.0 * BRAKE * look).sqrt());
            }
        }
        if let Some(entry) = agent.route.get(agent.piece)
            && matches!(entry.element, Element::Connector(_))
        {
            target = target.min(BOX_SPEED);
        }

        // --- signals ------------------------------------------------------------
        if let Some(remaining) = self.signal_gap(agent) {
            if remaining <= 0.25 {
                target = 0.0;
            } else {
                target = target.min((2.0 * BRAKE * (remaining - 0.15)).sqrt());
            }
        }

        // --- the vehicle ahead ---------------------------------------------------
        let (lead_gap, lead_speed) = self.leader(index, s);
        if let Some(gap) = lead_gap {
            let free = gap - CAR_LENGTH - MIN_GAP;
            if free <= 0.0 {
                target = 0.0;
            } else {
                target = target.min((lead_speed * lead_speed + 2.0 * BRAKE * free).sqrt());
            }
        }

        // --- integrate -------------------------------------------------------------
        let new_speed = if target > speed {
            (speed + agent.accel * dt).min(target)
        } else {
            (speed - BRAKE * dt).max(target).max(0.0)
        };
        let blocked_for = if new_speed < 0.2 {
            agent.blocked_for + dt
        } else {
            0.0
        };
        s += new_speed * dt;
        Decision {
            s,
            speed: new_speed,
            blocked_for,
        }
    }

    /// Move an agent onto the next route piece once it has run off the end of
    /// this one, keeping the arc length continuous so the vehicle never jumps.
    fn roll_over(&mut self, index: usize) {
        loop {
            let (piece, s) = {
                let agent = &self.agents[index];
                (agent.piece, agent.s)
            };
            let Some(entry) = self.agents[index].route.get(piece).cloned() else {
                return;
            };
            if s < entry.start_s + entry.length {
                return;
            }
            if piece + 1 < self.agents[index].route.len() {
                let agent = &mut self.agents[index];
                agent.piece += 1;
                agent.s = agent.route[agent.piece].start_s;
                continue;
            }
            // Route exhausted: plan a new one so the fleet never drains.
            //
            // The new route must *continue* the vehicle, not replace it.  The
            // re-plan starts from the lane the vehicle is on and the arc length
            // is re-seated so the vehicle is still at the same world point; a
            // fresh route starting at arc length zero would teleport it back to
            // the far end of its first lane, which is the last thing that made
            // vehicles appear to jump across junctions.
            let (origin_lane, ended_on_lane) = match entry.element {
                Element::Lane(lane) => (Some(lane), true),
                Element::Connector(connector) => (
                    self.network
                        .lanes
                        .iter()
                        .position(|lane| lane.id == self.network.connectors[connector].to_lane),
                    false,
                ),
            };
            let rng = self.agents[index].rng.wrapping_add(1);
            let mut replacement = origin_lane.and_then(|lane| self.plan(lane, rng));
            // A dead end (a cul-de-sac, or a street whose far junction has no
            // onward lane) has no route forward. Turn round onto the opposite lane
            // of the same road instead of stopping for good.
            let mut turned_round = false;
            if replacement.is_none() {
                let opposite = origin_lane.and_then(|lane| {
                    let here = &self.network.lanes[lane];
                    self.network.lanes.iter().position(|other| {
                        other.road == here.road && other.direction == -here.direction && other.index == here.index
                    })
                });
                replacement = opposite.and_then(|lane| self.plan(lane, rng));
                turned_round = replacement.is_some();
            }
            let agent = &mut self.agents[index];
            agent.blocked_for = 0.0;
            match replacement {
                Some(route) => {
                    // Seat the vehicle at the same world point it already
                    // occupies: the far end of its current lane, or the near end
                    // of the next one if it was already on a connector. A vehicle
                    // that turned round starts at the near end of the opposite
                    // lane, which is where this lane's far end is.
                    let seat = if ended_on_lane && !turned_round {
                        route[0].length
                    } else {
                        0.0
                    };
                    agent.piece = 0;
                    agent.route = route;
                    agent.s = seat;
                }
                None => {
                    // No route could be planned. Stay exactly where the vehicle
                    // is: on the same route piece, at the end of it. (Resetting
                    // the piece to the first one while `s` still counts along
                    // the old route put the vehicle at the end of the *first*
                    // lane, a jump of the whole route's length.)
                    agent.piece = piece;
                    agent.s = entry.start_s + entry.length;
                    if !ended_on_lane {
                        agent.s = entry.start_s;
                    }
                }
            }
            return;
        }
    }

    /// Distance to the nearest car ahead in the same lane, and its speed.
    ///
    /// The lateral gate is what stops a vehicle braking for the car in the
    /// adjacent lane: aligned traffic must be within 1.95 m laterally, while a
    /// merging or opposing movement is allowed a wider window but only nearby.
    fn leader(&self, index: usize, s: f32) -> (Option<f32>, f32) {
        let agent = &self.agents[index];
        let Some(entry) = agent.route.get(agent.piece) else {
            return (None, 0.0);
        };
        let path = self.path_of(entry.element);
        let (position, tangent) = path.sample((s - entry.start_s).clamp(0.0, entry.length));
        let here = Vec2::new(position.x, position.z);
        let mut best: Option<(f32, f32)> = None;
        for (other_index, other) in self.agents.iter().enumerate() {
            if other_index == index {
                continue;
            }
            let Some(other_entry) = other.route.get(other.piece) else {
                continue;
            };
            let other_path = self.path_of(other_entry.element);
            let (other_position, other_tangent) = other_path.sample(
                (other.s - other_entry.start_s).clamp(0.0, other_entry.length),
            );
            let delta = Vec2::new(other_position.x - here.x, other_position.z - here.y);
            let forward = delta.dot(tangent);
            if !(0.05..=34.0).contains(&forward) {
                continue;
            }
            let lateral = delta.dot(tangent.left_normal()).abs();
            let alignment = tangent.dot(other_tangent);
            let gate = if alignment > 0.9 {
                if forward < 12.0 { 1.95 } else { 1.70 }
            } else {
                3.0
            };
            if alignment <= 0.9 && forward > 24.0 {
                continue;
            }
            if lateral > gate {
                continue;
            }
            if best.is_none_or(|(existing, _)| forward < existing) {
                best = Some((forward, other.speed));
            }
        }
        match best {
            Some((gap, speed)) => (Some(gap), speed),
            None => (None, 0.0),
        }
    }

    /// Turn rate over a six-metre window, as curvature in radians per metre.
    fn curvature_of(&self, element: Element, local: f32) -> f32 {
        let path = self.path_of(element);
        let length = path.length();
        if length < 1.0 {
            return 0.0;
        }
        let first = path.tangent_at((local - 3.0).clamp(0.0, length));
        let second = path.tangent_at((local + 3.0).clamp(0.0, length));
        let delta = (second.angle() - first.angle()).abs();
        let wrapped = delta.min(std::f32::consts::TAU - delta);
        wrapped / 6.0
    }

    /// Plan a route through the connector graph.
    ///
    /// Dijkstra runs over *lanes*, where an edge is a connector.  The result is
    /// then expanded into the lane/connector/lane sequence a vehicle physically
    /// drives, which is the whole point: the path a car takes through a junction
    /// is the connector curve that was drawn there.
    fn plan(&self, origin_lane: usize, salt: u32) -> Option<Vec<RoutePiece>> {
        if origin_lane >= self.network.lanes.len() {
            return None;
        }
        let lane_count = self.network.lanes.len();
        let mut best = vec![f32::INFINITY; lane_count];
        let mut previous: Vec<Option<(usize, usize)>> = vec![None; lane_count];
        let mut open = BinaryHeap::new();
        best[origin_lane] = 0.0;
        open.push(QueueEntry {
            cost: 0.0,
            lane: origin_lane,
        });
        let mut target = usize::MAX;
        let mut target_cost = 0.0_f32;
        let mut settled = 0;
        while let Some(QueueEntry { cost, lane }) = open.pop() {
            if cost > best[lane] {
                continue;
            }
            settled += 1;
            if lane != origin_lane && cost > target_cost {
                target = lane;
                target_cost = cost;
            }
            if cost > 400.0 || settled > 800 {
                break;
            }
            for &(connector, to) in &self.adjacency[lane] {
                if to >= lane_count {
                    continue;
                }
                let path = &self.network.connectors[connector].path;
                // The movement preference is a *cost* in the search, not a
                // post-hoc re-sample.  Re-sampling the chosen connector after
                // the fact is what produced routes that stepped from one lane
                // straight to another with no connector between them, and a
                // vehicle on such a route teleports across the junction.
                let bias = match self.network.connectors[connector].movement {
                    Movement::Straight => 1.00,
                    Movement::Right => 1.15,
                    Movement::Left => 1.35,
                    Movement::UTurn => 3.00,
                };
                let cost = cost
                    + path.length() / 7.0 * bias
                    + self.network.lanes[to].path.length() / 11.0;
                if cost < best[to] {
                    best[to] = cost;
                    previous[to] = Some((lane, connector));
                    open.push(QueueEntry { cost, lane: to });
                }
            }
        }
        if target == usize::MAX {
            return None;
        }
        // Walk the chain back to the origin, collecting (from, connector, to).
        let mut chain: Vec<(usize, usize)> = Vec::new();
        let mut cursor = target;
        let mut guard = 0;
        while cursor != origin_lane && guard < 256 {
            let Some((from, connector)) = previous[cursor] else {
                return None;
            };
            chain.push((from, connector));
            cursor = from;
            guard += 1;
        }
        chain.reverse();
        if chain.is_empty() {
            return None;
        }
        // Expand into pieces.  The connector chain Dijkstra returned is used
        // verbatim: it is the only thing guaranteed to join the two lanes, and
        // re-choosing a connector here is exactly how a lane-to-lane step with
        // no geometry in between slips in.
        let mut lanes: Vec<usize> = vec![origin_lane];
        let mut connectors: Vec<usize> = Vec::with_capacity(chain.len());
        for &(from, connector) in &chain {
            let Some((_, to)) = self.successors[from]
                .iter()
                .find(|(index, _)| *index == connector)
            else {
                return None;
            };
            connectors.push(connector);
            lanes.push(*to);
        }
        // Build the element sequence first.  The lane/connector alternation is
        // *structural*: emitting pieces in a single pass and skipping any piece
        // with a degenerate length silently produced lane→lane steps with no
        // connector between them, and a vehicle on such a route teleports across
        // the junction.  Deciding the sequence up front makes that impossible.
        let mut steps: Vec<Element> = Vec::with_capacity(lanes.len() + connectors.len());
        steps.push(Element::Lane(lanes[0]));
        for (index, connector) in connectors.iter().enumerate() {
            if self.network.connectors[*connector].path.length() > 0.2 {
                steps.push(Element::Connector(*connector));
            }
            steps.push(Element::Lane(lanes[index + 1]));
        }
        let mut pieces: Vec<RoutePiece> = Vec::with_capacity(steps.len());
        let mut start_s = 0.0;
        for (position, step) in steps.iter().enumerate() {
            let length = self.path_of(*step).length();
            if length < 0.2 {
                continue;
            }
            // A stop line only exists where a lane actually ends at a
            // signalised junction, which is what `stop_line` means.
            let (stop_line, axis) = match step {
                Element::Lane(lane) if position + 1 < steps.len() => self.junction_end(*lane),
                _ => (0.0, 0),
            };
            pieces.push(RoutePiece {
                element: *step,
                start_s,
                length,
                stop_line,
                axis,
            });
            start_s += length;
        }
        let _ = salt;
        if pieces.len() < 2 {
            return None;
        }
        Some(pieces)
    }

    /// Where a lane ends: the distance to the stop line and the signal axis.
    fn junction_end(&self, lane_index: usize) -> (f32, u8) {
        let lane = &self.network.lanes[lane_index];
        let length = lane.path.length();
        if length < 1.0 {
            return (0.0, 0);
        }
        let Some(junction) = self.network.junction(lane.to_node) else {
            return (0.0, 0);
        };
        if junction.ports.len() < 3 {
            return (0.0, 0);
        }
        // Only signalised junctions bind.  A give-way junction is handled by the
        // curvature cap, not by a light that does not exist.
        let signalised = self
            .signals
            .iter()
            .any(|rig| rig.junction == junction.node && rig.road == lane.road);
        if !signalised {
            return (0.0, 0);
        }
        let axis = self
            .signals
            .iter()
            .find(|rig| rig.junction == junction.node && rig.road == lane.road)
            .map(|rig| if rig.axis == "ns" { 0 } else { 1 })
            .unwrap_or(0);
        // The lane ends at the junction kerb face, and the stop line sits
        // `stop_line_gap` metres back from it.
        (length - self.network.spec.stop_line_gap, axis)
    }

    /// Plan a route starting from any lane leaving `node`.
    #[allow(dead_code)]
    fn plan_from_node(&self, node: u32, salt: u32) -> Option<Vec<RoutePiece>> {
        let candidates: Vec<usize> = self
            .network
            .lanes
            .iter()
            .enumerate()
            .filter(|(_, lane)| lane.from_node == node)
            .map(|(index, _)| index)
            .filter(|index| {
                !matches!(
                    self.network.lanes[*index].use_kind,
                    LaneUse::Bike | LaneUse::Shoulder
                )
            })
            .collect();
        if candidates.is_empty() {
            return None;
        }
        let mut rng = Rng::new(salt ^ 0x27d4_eb2f);
        for _ in 0..6 {
            let lane = candidates[rng.int(candidates.len() as u32) as usize];
            if let Some(route) = self.plan(lane, salt) {
                return Some(route);
            }
        }
        None
    }
}

#[derive(PartialEq)]
struct QueueEntry {
    cost: f32,
    lane: usize,
}

// `f32` is not `Eq`, but the ordering below is a total order over the finite
// costs this queue ever holds, which is all `BinaryHeap` requires.  Stating that
// explicitly is the standard way to express it.
impl Eq for QueueEntry {}

/// Reversed ordering, so `BinaryHeap` behaves as a min-heap.
impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| other.lane.cmp(&self.lane))
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Build a fleet for a city.  Returns `None` when the network has no drivable
/// lane, so the renderer can skip the whole traffic layer.
pub fn simulate(
    network: &Network,
    signals: Vec<SignalRig>,
    seed: u32,
    count: usize,
) -> Option<TrafficSim> {
    let sim = TrafficSim::new(network, signals, seed, count);
    if sim.agent_count() == 0 {
        None
    } else {
        Some(sim)
    }
}

/// Initial fleet state at cycle time zero, for the renderer's first frame.
pub fn initial_state(sim: &TrafficSim) -> TrafficState {
    let aspects = sim
        .signals()
        .iter()
        .map(|rig| aspect_at(0.0, rig.axis))
        .collect();
    let mut agents = Vec::with_capacity(sim.agent_count());
    for index in 0..sim.agent_count() {
        let mut pose = sim.pose_of(index);
        pose.stopped = pose.speed < 0.2;
        agents.push(pose);
    }
    TrafficState {
        time: 0.0,
        agents,
        aspects,
        stalled: Vec::new(),
    }
}

/// A movement's signal permission, exposed for tests and for tooling that wants
/// to explain a phase.  China permits right on red, so a right turn is the one
/// movement a red does not block.
pub fn permits(aspect: u8, movement: Movement) -> bool {
    match aspect {
        2 => true,
        1 => movement == Movement::Straight || movement == Movement::Right,
        _ => movement == Movement::Right,
    }
}

/// Convenience for tools that want a lane's identity from an agent's route.
pub fn route_lane_count(sim: &TrafficSim, index: usize) -> usize {
    sim.agents
        .get(index)
        .map(|agent| {
            agent
                .route
                .iter()
                .filter(|piece| matches!(piece.element, Element::Lane(_)))
                .count()
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::MeshBuilder;
    use crate::network::derive;
    use crate::spec::JunctionSpec;
    use crate::street::build;
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    fn city() -> (urban::ModernCity, Network, Vec<SignalRig>) {
        let city = generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        });
        let network = derive(
            &city.nodes,
            &city.sd_roads,
            &city.hd_roads,
            city.frame,
            JunctionSpec::default(),
            city.seed,
        );
        let mut builder = MeshBuilder::new();
        let street = build(&network, &mut builder, city.seed);
        (city, network, street.signals)
    }

    #[test]
    fn a_pose_yaw_puts_the_nose_where_the_vehicle_is_going() {
        // The renderer writes `dummy.rotation.set(0, heading, 0)`, and three.js
        // maps a local `+X` axis to `(cos h, 0, -sin h)`.  If the payload's
        // heading were a plan bearing instead, a vehicle on a north-south street
        // would be drawn driving backwards.
        for (direction, expected_x, expected_z) in [
            (Vec2::new(1.0, 0.0), 1.0, 0.0),
            (Vec2::new(0.0, 1.0), 0.0, 1.0),
            (Vec2::new(0.0, -1.0), 0.0, -1.0),
            (Vec2::new(-1.0, 0.0), -1.0, 0.0),
        ] {
            let heading = heading_for(direction);
            // Reproduce three.js' Y-Euler on a local +X axis.
            let (sin, cos) = heading.sin_cos();
            let mapped = Vec2::new(cos, -sin);
            assert!(
                (mapped.x - expected_x).abs() < 1.0e-4
                    && (mapped.y - expected_z).abs() < 1.0e-4,
                "heading {heading} maps +X to {mapped:?}, expected ({expected_x}, {expected_z})"
            );
        }
    }

    #[test]
    fn a_city_gets_a_fleet_that_actually_moves() {
        let (city, network, signals) = city();
        let mut sim = simulate(&network, signals, city.seed, 40).expect("no fleet");
        assert!(sim.agent_count() >= 8, "only {} agents", sim.agent_count());
        let start = initial_state(&sim);
        for _ in 0..240 {
            sim.step(1.0 / 30.0);
        }
        let later = sim.step(0.0);
        let moved = later
            .agents
            .iter()
            .zip(start.agents.iter())
            .filter(|(a, b)| (a.x - b.x).hypot(a.z - b.z) > 5.0)
            .count();
        assert!(
            moved > 3,
            "only {moved} of {} vehicles moved",
            later.agents.len()
        );
    }

    #[test]
    fn vehicles_never_teleport_through_a_junction() {
        let (city, network, signals) = city();
        let mut sim = simulate(&network, signals, city.seed, 30).unwrap();
        let mut previous = initial_state(&sim).agents;
        for _ in 0..900 {
            let state = sim.step(1.0 / 20.0);
            for (index, pose) in state.agents.iter().enumerate() {
                let previous_pose = &previous[index];
                let jump = (pose.x - previous_pose.x).hypot(pose.z - previous_pose.z);
                // At 20 Hz a 13 m/s vehicle covers 0.65 m.  6 m is a generous
                // bound that still catches a graph discontinuity — and it is the
                // assertion that failed before routes were rebuilt to include the
                // connector curves.
                assert!(jump < 6.0, "vehicle {index} jumped {jump:.2} m in one step");
            }
            previous = state.agents;
        }
    }

    #[test]
    fn routes_actually_traverse_the_connectors() {
        let (city, network, signals) = city();
        let sim = simulate(&network, signals, city.seed, 30).unwrap();
        // Every vehicle must be routed across junctions, not along a single
        // lane's centreline: a multi-leg route is what makes the intersection
        // geometry in `network.connectors` load-bearing.
        let multi = (0..sim.agent_count())
            .filter(|index| route_lane_count(&sim, *index) >= 3)
            .count();
        assert!(
            multi * 2 >= sim.agent_count(),
            "only {multi} of {} routes cross a junction",
            sim.agent_count()
        );
    }

    #[test]
    fn vehicles_stay_on_the_lane_or_connector_geometry() {
        let (city, network, signals) = city();
        let mut sim = simulate(&network, signals, city.seed, 24).unwrap();
        // Precompute the polyline of every drivable path once; the per-frame test
        // is a distance-to-polyline over hundreds of thousands of segments
        // otherwise.
        let polylines: Vec<Vec<Vec2>> = (0..network.lanes.len())
            .map(|index| {
                network.lanes[index]
                    .path
                    .points()
                    .iter()
                    .map(|v| Vec2::new(v.x, v.z))
                    .collect()
            })
            .chain(network.connectors.iter().map(|connector| {
                connector
                    .path
                    .points()
                    .iter()
                    .map(|v| Vec2::new(v.x, v.z))
                    .collect()
            }))
            .collect();
        for _ in 0..200 {
            let state = sim.step(1.0 / 20.0);
            for pose in &state.agents {
                let point = Vec2::new(pose.x, pose.z);
                let nearest = polylines
                    .iter()
                    .map(|plan| crate::math::distance_to_polyline(point, plan))
                    .fold(f32::MAX, f32::min);
                // A vehicle sits on a lane centre, so its body reaches half a
                // width either side; allow that plus a small numeric margin.
                assert!(
                    nearest < CAR_WIDTH * 0.5 + 0.4,
                    "a vehicle sits {nearest:.2} m from any driven path"
                );
            }
        }
    }

    #[test]
    fn the_signal_clock_gives_each_axis_a_real_green() {
        // Sixteen-second cycle: north-south green, then east-west green, with a
        // second of yellow and a second of all-red on each change.
        assert_eq!(aspect_at(0.0, "ns"), 2);
        assert_eq!(aspect_at(5.9, "ns"), 2);
        assert_eq!(aspect_at(6.4, "ns"), 1);
        assert_eq!(aspect_at(7.5, "ns"), 0);
        assert_eq!(aspect_at(8.5, "ns"), 0);
        assert_eq!(aspect_at(9.0, "ew"), 2);
        assert_eq!(aspect_at(14.4, "ew"), 1);
        assert_eq!(aspect_at(15.5, "ew"), 0);
        for step in 0..320 {
            let t = step as f32 * 0.1;
            assert!(
                !(aspect_at(t, "ns") == 2 && aspect_at(t, "ew") == 2),
                "both axes green at {t:.1}s"
            );
        }
    }

    #[test]
    fn a_vehicle_holds_at_a_red_and_releases_on_green() {
        let (city, network, signals) = city();
        let mut sim = simulate(&network, signals, city.seed, 40).unwrap();
        let mut saw_stop = false;
        let mut saw_moving = false;
        for _ in 0..900 {
            let state = sim.step(1.0 / 20.0);
            saw_stop |= state.agents.iter().any(|pose| pose.stopped);
            saw_moving |= state.agents.iter().any(|pose| pose.speed > 3.0);
        }
        assert!(saw_stop, "no vehicle ever stopped: signals are not binding");
        assert!(saw_moving, "the whole fleet is stopped");
        assert!(
            sim.step(0.0).stalled.is_empty(),
            "the fleet deadlocked, which means the graph has a trap"
        );
    }

    #[test]
    fn the_simulation_is_deterministic() {
        let (city, network, signals) = city();
        let run = || {
            let mut sim = simulate(&network, signals.clone(), city.seed, 20).unwrap();
            for _ in 0..120 {
                sim.step(1.0 / 30.0);
            }
            sim.step(0.0)
        };
        let first = run();
        let second = run();
        assert_eq!(first.agents.len(), second.agents.len());
        for (a, b) in first.agents.iter().zip(second.agents.iter()) {
            assert!((a.x - b.x).abs() < 1.0e-6 && (a.z - b.z).abs() < 1.0e-6);
        }
    }
}
