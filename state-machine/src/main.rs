// You can play around in this function.
fn main() {
    let is_follower = Raft::new(/* ... */);
    // Raft typically comes in groups of 3, 5, or 7. Just 1 for us. :)

    // Simulate this node timing out first.
    let is_candidate = Raft::<Candidate>::from(is_follower);

    // It wins! How unexpected.
    let is_leader = Raft::<Leader>::from(is_candidate);

    // Then it fails and rejoins later, becoming a Follower again.
    let is_follower_again = Raft::<Follower>::from(is_leader);

    // And goes up for election...
    let is_candidate_again = Raft::<Candidate>::from(is_follower_again);

    // But this time it fails!
    let is_follower_another_time = Raft::<Follower>::from(is_candidate_again);
}

// This is our state machine.
struct Raft<S> {
    // ... Shared Values
    state: S,
}

// The three cluster states a Raft node can be in

// If the node is the Leader of the cluster services requests and replicates its state.
struct Leader {
    // ... Specific State Values
}

// If it is a Candidate it is attempting to become a leader due to timeout or initialization.
struct Candidate {
    // ... Specific State Values
}

// Otherwise the node is a follower and is replicating state it receives.
struct Follower {
    // ... Specific State Values
}

// Raft starts in the Follower state
impl Raft<Follower> {
    fn new(/* ... */) -> Self {
        // ...
        Raft {
            // ...
            state: Follower { /* ... */ },
        }
    }
}

// The following are the defined transitions between states.

// When a follower timeout triggers it begins to campaign
impl From<Raft<Follower>> for Raft<Candidate> {
    fn from(val: Raft<Follower>) -> Raft<Candidate> {
        // ... Logic prior to transition
        Raft {
            // ... attr: val.attr
            state: Candidate { /* ... */ },
        }
    }
}

// If it doesn't receive a majority of votes it loses and becomes a follower again.
impl From<Raft<Candidate>> for Raft<Follower> {
    fn from(val: Raft<Candidate>) -> Raft<Follower> {
        // ... Logic prior to transition
        Raft {
            // ... attr: val.attr
            state: Follower { /* ... */ },
        }
    }
}

// If it wins it becomes the leader.
impl From<Raft<Candidate>> for Raft<Leader> {
    fn from(val: Raft<Candidate>) -> Raft<Leader> {
        // ... Logic prior to transition
        Raft {
            // ... attr: val.attr
            state: Leader { /* ... */ },
        }
    }
}

// If the leader becomes disconnected it may rejoin to discover it is no longer leader
impl From<Raft<Leader>> for Raft<Follower> {
    fn from(val: Raft<Leader>) -> Raft<Follower> {
        // ... Logic prior to transition
        Raft {
            // ... attr: val.attr
            state: Follower { /* ... */ },
        }
    }
}
