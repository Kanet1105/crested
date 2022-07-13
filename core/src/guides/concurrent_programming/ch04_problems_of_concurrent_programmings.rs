fn main() {
    println!("Hello, world!");
}

// 4.3 Bankers's Algorithm
// 아놔.. 이해.. X.. 
#[test]
fn p152() {
    // available : 은행이 보유한 돈, 즉 얼마까지 빌려줄수 있는지. 
    // allocation : 각 고객들이 현재 빌린 돈이 얼마인지.
    // max : 각 고객들이 얼마의 최대값으로 대출을 요구할지. 
    // 시스템 상태가 stable state 이니지 unsatable state 인지 

    struct Resource<const NRES: usize, const NTH: usize> {
        available: [usize; NRES],           // 이용 가능한 리소스 available[j] 는 j번째 리소스
        allocation: [[usize; NRES]; NTH],   // thread i 가 확보중인 리소스 allocation[i][j](? [j][i] 인듯) 는 스레드 i 가 현재 확보하고 있는 리소스 j 의 수  
        max: [[usize; NRES]; NTH],          // thread i 가 필요로 하는 리소스의 최대값 max[i][j](? [j][i] 인듯) 는 스레드 i 가 필요한 리소스 j 의 최대값 
    }

    impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
        fn new(available: [usize;NRES], max: [[usize;NRES]; NTH]) -> Self {
            Resource{
                available,
                allocation: [[0;NRES]; NTH],
                max,    
            }
        }

        // 현재 상태가 데드락을 발생시키지 않는지 확인(safe -> true, deadlock, starvation -> false)
        fn is_safe(&self) -> bool {
            let mut finish = [false; NTH];            
            // 스레드 i 는 리소스 획득과 반환에 성공했는가?
            // finish[i] = true 일 때, 스레드 i 가 리소스를 확보해 처리를 수행하고, 그후, 모든 리소스를 반환할 수 있음을 나타냄
           
            let mut work = self.available.clone();  
            // 이용 가능한 리소스의 시뮬레이션 값. 
            // work[j]  는 시뮬레이션상에서의 은헹원이 보유한 리소스 j 의 수를 나타낸다. 

            loop {
                // 모든 스레드 i 와 리소스 j 에 대해, 
                // finish[i] == false && work[j] >= (self.max[j][i] - self.allocation[j][i]) 
                // 를 만족하는 스레드를 찾는다. 
                let mut found = false;  
                let mut num_true = 0;
                for (i, alc) in self.allocation.iter().enumerate() {
                    if finish[i] {
                        num_true += 1;
                        continue;
                    }

                    let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
                    let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                    if is_avail {
                        found = true;
                        finish[i] = true;
                        for (w, a) in work.iter_mut().zip(alc) {
                            *w += *a;
                        }
                        break;
                    }
                }

                if num_true == NTH {
                    return true;
                }
                if !found {
                    break;
                }
            }
            false
        }

        fn ake(&mut self, id: usize, resource: usize) -> bool {
            if id >= NTH || resource >= NRES || self.available[resource] == 0 {
                return false;
            }

            self.allocation[id][resource] += 1;
            self.available[resource] -= 1;

            if self.is_safe() {
                true
            } else {
                self.allocation[id][resource] -= 1;
                self.available[resource] += 1;
                false
            }
        }

        fn release(&mut self, id: usize, resource: usize) {
            if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
                return;
            }
            
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
        }
    }
}

// 4.4 Reentrant lock (with Mutex) 
// 만약 lock 을 획득한 thread 가 해당 lock 을 해제하기 전에 다시 lock 을 얻고자 한다 (재귀락 recursive lock)면?
// -> deadlock 발생 
// -> 재진입 가능 lock (reentrant lock) 구현 필요
#[test]
fn p158() {
    struct reent_lock {
        lock: bool,
        id:

    }
}

// 4.3 Bankers's Algorithm
// 아놔.. 이해.. X.. 
#[test]
fn p152() {
    // available : 은행이 보유한 돈, 즉 얼마까지 빌려줄수 있는지. 
    // allocation : 각 고객들이 현재 빌린 돈이 얼마인지.
    // max : 각 고객들이 얼마의 최대값으로 대출을 요구할지. 
    // 시스템 상태가 stable state 이니지 unsatable state 인지 

    struct Resource<const NRES: usize, const NTH: usize> {
        available: [usize; NRES],           // 이용 가능한 리소스 available[j] 는 j번째 리소스
        allocation: [[usize; NRES]; NTH],   // thread i 가 확보중인 리소스 allocation[i][j](? [j][i] 인듯) 는 스레드 i 가 현재 확보하고 있는 리소스 j 의 수  
        max: [[usize; NRES]; NTH],          // thread i 가 필요로 하는 리소스의 최대값 max[i][j](? [j][i] 인듯) 는 스레드 i 가 필요한 리소스 j 의 최대값 
    }

    impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
        fn new(available: [usize;NRES], max: [[usize;NRES]; NTH]) -> Self {
            Resource{
                available,
                allocation: [[0;NRES]; NTH],
                max,    
            }
        }

        // 현재 상태가 데드락을 발생시키지 않는지 확인(safe -> true, deadlock, starvation -> false)
        fn is_safe(&self) -> bool {
            let mut finish = [false; NTH];            
            // 스레드 i 는 리소스 획득과 반환에 성공했는가?
            // finish[i] = true 일 때, 스레드 i 가 리소스를 확보해 처리를 수행하고, 그후, 모든 리소스를 반환할 수 있음을 나타냄
           
            let mut work = self.available.clone();  
            // 이용 가능한 리소스의 시뮬레이션 값. 
            // work[j]  는 시뮬레이션상에서의 은헹원이 보유한 리소스 j 의 수를 나타낸다. 

            loop {
                // 모든 스레드 i 와 리소스 j 에 대해, 
                // finish[i] == false && work[j] >= (self.max[j][i] - self.allocation[j][i]) 
                // 를 만족하는 스레드를 찾는다. 
                let mut found = false;  
                let mut num_true = 0;
                for (i, alc) in self.allocation.iter().enumerate() {
                    if finish[i] {
                        num_true += 1;
                        continue;
                    }

                    let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
                    let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                    if is_avail {
                        found = true;
                        finish[i] = true;
                        for (w, a) in work.iter_mut().zip(alc) {
                            *w += *a;
                        }
                        break;
                    }
                }

                if num_true == NTH {
                    return true;
                }
                if !found {
                    break;
                }
            }
            false
        }

        fn ake(&mut self, id: usize, resource: usize) -> bool {
            if id >= NTH || resource >= NRES || self.available[resource] == 0 {
                return false;
            }

            self.allocation[id][resource] += 1;
            self.available[resource] -= 1;

            if self.is_safe() {
                true
            } else {
                self.allocation[id][resource] -= 1;
                self.available[resource] += 1;
                false
            }
        }

        fn release(&mut self, id: usize, resource: usize) {
            if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
                return;
            }
            
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
        }
    }
}

// 4.4 Reentrant lock (with Mutex) 
// 만약 lock 을 획득한 thread 가 해당 lock 을 해제하기 전에 다시 lock 을 얻고자 한다 (재귀락 recursive lock)면?
// -> deadlock 발생 
// -> 재진입 가능 lock (reentrant lock) 구현 필요
#[test]
fn p158() {
    struct reent_lock {
        lock: bool,
        id:

    }
}