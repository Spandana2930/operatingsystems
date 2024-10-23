#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,  // Vector of queues for each priority level
    num_levels: usize,           // Number of priority levels
    time_quanta: Vec<u32>,      // Time quanta for each queue
    current_time: u32,          // Tracks the current time in the scheduler
}

impl MLFQ {
    /// Creates a new MLFQ scheduler with specified number of levels and time quanta.
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],  // Initialize empty queues for each priority level
            num_levels,
            time_quanta,
            current_time: 0,  // Start at time 0
        }
    }

    /// Adds a new process to the appropriate queue based on its priority.
    /// If the priority is out of range, it sets the process to the lowest priority queue.
    pub fn add_process(&mut self, mut process: Process) {
        // Ensure the process priority is within valid bounds
        if process.priority >= self.num_levels {
            process.priority = self.num_levels - 1;  // Set to lowest priority if out of range
        }
        // Add the process to the corresponding priority queue
        self.queues[process.priority].push(process);
    }

    /// Executes the next process in the specified queue based on the time quantum.
    /// Updates the remaining time and executed time, and manages process promotion or completion.
    pub fn execute_process(&mut self, queue_index: usize) {
        // Check if the specified queue has processes to execute
        if self.queues[queue_index].is_empty() {
            return; // No process to execute
        }
        
        let time_quantum = self.time_quanta[queue_index];  // Get the time quantum for the current queue
        let mut process = self.queues[queue_index].remove(0); // Remove the process from the queue
        
        // Determine how long the process can execute (up to time quantum or remaining time)
        let execution_time = std::cmp::min(time_quantum, process.remaining_time);
        process.remaining_time -= execution_time;  // Update remaining time
        process.total_executed_time += execution_time;  // Update total executed time
        self.current_time += execution_time;  // Update current time in the scheduler
        
        // Check if the process has completed its execution
        if process.remaining_time == 0 {
            // Process has completed, do not re-add to the queue
            println!("Process {} completed.", process.id);
        } else {
            // Process did not complete, check if it should be promoted to a lower priority
            if execution_time == time_quantum && process.priority < self.num_levels - 1 {
                process.priority += 1;  // Move to a lower priority queue
            }
            // Add the process back to the appropriate queue based on its current priority
            self.queues[process.priority].push(process);
        }
    }

    /// Boosts the priority of all processes by moving them to the highest priority queue.
    /// Resets the priority of all moved processes to 0 (highest priority).
    pub fn priority_boost(&mut self) {
        let mut boosted_processes = Vec::new();  // Temporary storage for processes to boost

        // Move all processes from lower priority queues to the highest priority queue
        for i in 1..self.num_levels {
            while let Some(mut process) = self.queues[i].pop() {
                process.priority = 0;  // Reset priority to the highest level
                boosted_processes.push(process);  // Store the boosted process
            }
        }

        // Add the boosted processes to the highest priority queue
        self.queues[0].extend(boosted_processes);
    }

    // Simulates the passing of time and triggers a boost if needed.
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;  // Increment the current time
        let boost_interval = 100;  // Define the interval for boosting priorities
        // Check if it’s time to perform a priority boost
        if self.current_time % boost_interval == 0 {
            self.priority_boost();  // Trigger the priority boost
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);  // Add a process to the highest priority queue
        mlfq.add_process(process2);  // Add a process to the second priority queue
        mlfq.add_process(process3);  // Attempt to add a process with invalid priority (5)

        // Verify that the processes are added to the correct queues
        assert_eq!(mlfq.queues[0].len(), 1);  // One process in highest priority
        assert_eq!(mlfq.queues[1].len(), 1);  // One process in second priority
        assert_eq!(mlfq.queues[2].len(), 1);  // One process in lowest priority
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);  // Execute the process from the highest priority queue

        // Verify that the process was moved to the next queue
        assert_eq!(mlfq.queues[0].len(), 0);  // No processes left in highest priority
        assert_eq!(mlfq.queues[1].len(), 1);  // One process moved to second priority
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);  // Remaining time should be updated
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);  // Total executed time should be updated
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        // Verify that all processes are boosted to the highest priority
        assert_eq!(mlfq.queues[0].len(), 2);  // Two processes should be in highest priority
        assert_eq!(mlfq.queues[1].len(), 0);  // No processes left in second priority
        assert_eq!(mlfq.queues[2].len(), 0);  // No processes left in lowest priority
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        // Verify that the boost did not occur prematurely
        assert_eq!(mlfq.queues[1].len(), 1);  // One process should still be in second priority
        assert_eq!(mlfq.queues[0].len(), 0);  // No processes in highest priority
    }
}
