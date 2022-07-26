/// ch06 멀티태스크
///
/// - green thread 를 Rust 언어를 사용하여 mutitasking
/// - 간단한 actor model 구현
/// 
/// 6.1 Mutlitask 
/// context : register (또는 stack 정보) 등의 프로세스 상태에 관현 정보
/// context switching : 어떤 process 에서 다른 process 로 실행을 전환 (이때, context 의 저장 복원 등 일련의 처리)
/// 
/// fairness
///                   (약한, 일반적) fairness         vs          (강한, unconditional) fairness
/// 
/// 실행 가능 상태             ------------------                 ---------         ---------       -------               
/// 실행 불가 상태    ---------                             ------         ---------         -------        
///                  일정 시점 이후 작업 가능                     작업 가능 상태와 불가 상태를 무한 반복
///                  이런 조건에서 프로세스 실행 구현              이런 조건에서 프로세스 실행 구현
///                  -> 현실적 시스템에서 필수 구현 조건           -> 현실적으로 구현이 어려움
///
/// 
/// cooperative (non-preemptive (비선점적)) multitasking : 각각의 process 가 자발적으로 context switching  수행 
/// non-cooperative (preemptive (선점적)) multitasking   : process 간 협조 없이 외부 작동에 따라 context switching 수행
/// 
/// scheduler : context switching 전략을 결정하는 process, module, 함수 등.
/// 
/// cooperative multitasking 
///  - 예   : Rust 나 Python 의 async/await, Windows 3.1 또는 classic Mac 등 구형 OS
///  - 단점 : process 가 자발적으로 context switching 을 해야 함 
///           -> 어떤 process 가 무한 루프 빠지거나 정지하면 cpu 를 계속 점유하므로 해당 자원을 쓸수 없게 된다. 
///           -> 해결 방법은 곧 버그 없는 프로그램 작성 뿐
///
/// non-cooperative multitasking 
///  - 예   : Windows, Linux 등 현대적 OS -> app 간 충돌이 os 에 영향을 미치지 않음
///           Erlang, Go 등
///  - 단점 : 처리 시스템 구현이 어려움
///           fairness 확보를 위해 context switching 이 좀더 빈번하게 발생 -> 해당 오버헤드가 상대적으로 높음
/// 


/// 이후 내용은 아직 지식이 부족해서..;; 추후에 다시 학습 후 정리.. 
/// 6.2 협조적 green thread 구현
///  

/// -------------------------------------------------
fn eof(){}