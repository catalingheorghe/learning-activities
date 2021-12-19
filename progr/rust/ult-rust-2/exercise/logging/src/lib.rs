// logging not in standard library
// official library: log
// use log::{error, warn, info, debug, trace};
//
// warn!("msg");
// warn!(target: "Puzzle", "msg {}!", 512);
// (target is by default the module)
//
// Application must also use the plumbing in log to get the output somewhere
// eg library for app: 'env_logger' (RUST_LOG=info env variable); -> stderr
//
// Advanced logging capabilities: tracing library
//

use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Frog {
    energy: u8,
    sleeping: bool,
}

impl Frog {
    pub fn new() -> Self {
        debug!("A new Frog has been created");
        Default::default()
    }
    pub fn hop(&mut self) {
        self.energy -= 1;
        info!("Frog hopped. Energy left {}.", self.energy);
        if self.energy == 0 {
            warn!("Frog will go to sleep. Out of energy.");
            self.sleep();
        }
    }
    pub fn sleep(&mut self) {
        if !self.sleeping {
            error!("Putting frog to sleep, but it was already sleeping.");
            self.sleeping = true;
        }
    }
    pub fn oac(&self) {
        info!("Oac!");
    }
}

impl Default for Frog {
    fn default() -> Self {
        let frog = Frog {
            energy: 5,
            sleeping: false,
        };
        trace!("A default Frog was generated {:?}", frog);
        frog
    }
}
