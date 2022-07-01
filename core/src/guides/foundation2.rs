/// * 앞에 부분은 추가 작업 예정...;;;
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// ---------------------------------------------------------------------------
/// Generic Lifetime
/// 프로그래밍을 하다보면, variable 의 reference 상태로 가지고 있어햐 하는 경우가 있다. 
/// 그런데 borrow checker 가 정상적으로 작동하기 위해 reference 의 lifetime 명확히 해주어야 한다.
/// 따라서 reference 는 상황에 따라 추가된 별도의 lifetime 의 정의가 필요할수 있다. 
/// 
/// 또한, 함수의 매개변수로 받은 reference 와 함수의 결과물 return reference 간에 lifetime 이 다를 수 있다. 
/// 위와 같은, 다양한 상황에 대해 variable 의 lifetime 을 규정할 때 generic lifetime 을 사용한다.
/// ---------------------------------------------------------------------------
/// 
/// ---------------------------------------------------------------------------
/// Lifetime Variance
/// 
/// subtype 이란?
/// 다른 OOP languages 와 다르게 lifetime 에 대해 사용하며,
/// lifetime 을 명시적으로 추가 부여했을 때, 
/// 해당 type 이 기본적으로 가지고 있는 lifetime 의 위치가 subtype 이 됨. 
///
/// supertype   : higher in rank/ added / explicit / specific
/// subtype     : lower in rank / base  / implicit / general
/// 
/// Variance (전체적으로 참고 : https://doc.rust-lang.org/reference/subtyping.html) 
/// 하나의 variable 에 여러 lifetime 이 존재하므로, 각 lifetime 간 차이가 variance 이다.
/// 이들간 어떤 lifetime 을 사용해야 하는지, 상황에 따라 다를 수 있다.  
///  
/// Convariant / Contravariant / Invariant
///  1. OOP적 개념 정리 (Convariance vs Contravariance) (참고 : https://see-ro-e.tistory.com/245)
///     covariance(공변성)      : X -> Y일때 C<T>가 C<X> -> C<Y>
///     contravariance(반공변성): X -> Y일때 C<T>가 C<Y> -> C<X> 
/// 
///     covariance 인 구조는 x > y 관계의 input 에 대해서 output이 동일한 C<x> > C<y> 구조로 나오는 것. 
///     ex) input   : 애완동물 > 해수어 > 흰동가리
///         function: ...를 키운다.
///         결과    :
///                               개    상어  흰동가리                                                        
///                 애완동물   :   T      T       T      
///                 해수어     :   F      T       T     
///                 흰동가리   :   F      F       T  
///         -> input 에 정의한 x > y 구조대로 결과값(C<x> > C<y>)이 도출됨
/// 
///     contravariance 의 경우, input : x > y 구조가 반대 구조의 output (C<x> < C<y>)을 발생시킴.
///     ex) input   :  한국거주 철수 >  (한국 10년 프랑스 10년 산) 교포 은수 > (한국 10년 프랑스 10년 거주, 디자인 경력 5년) 프로그래머 금수
///         function:  어떤 회사에 갈 수 있을까?
///         결과    :
///                         현대자동차    르노자동차    카르띠에
///                 철수 :      T            F            F
///                 은수 :      T            T            F     
///                 금수 :      T            T            T
///         -> 가장 구체적 특성을 가진 금수가 가장 넒은 분야에서 직장을 구할 수 있다.(예시가 마음에 들지 않지만..) 
/// 
///     * object 가 상속을 받아 내려가면서 자식 object 는 좀더 구체적 의미를 부여받는다.
///     (아래 2가지는 개인적인 생각)
///     * 만약 질문(function) 이 각 object 의 상태에 대한 것이라면, 이는 covariant 한 경우가 된다.
///     * 만약 각 object 이 가진 의미(기능/능력?) 에 의존한 것이라면, contravariant 이다.
/// 
///  2. rust lifetime variance 관점에서
///     OOP 개념에서 상속에 의한 x > y 에서 자식에게 부가되는 가치를 lifetime 으로 바꿔서 생각해보면,
///     variable 에 추가적으로 (specific 한) generic lifetime 을 부가했을 때, 
///     상대적으로 기본 subtype 은 좀더 일반적인(긴) lifetime 을 가지게 된다. 
///     (찾아본 여러 자료에서 모두 해당 예시로 subtype : &'static str 만 들어서... 
///      모든 subtype 이 무조건 더 긴 lifetime 을 갖는것이 보장되는지 잘 모르겠다.)
///     
///     그리고 책에서 언급된 "useful" 의 의미와 결합했을 때,
///     더 긴(genernal) 한 lifetime 인 subtype 을 적용했을 때 "useful" 하다면 -> covariant
///     짧은(specific) 한 lifetime 인 supertype 을 적용했을 때 "useful" 하다면 -> contravariant
///     
///     "useful" 하다는 것을 OOP 개념과 비교해서 이야기하면,
///     프로그램 내에서 좀더 일반적으로 사용할수 있는가 (OOP covariant 예시에서는 '애완동물' / contravariant 예시에서는 '금수')  
///
///     마지막으로, invariant 는 단순히 사전적 의미인 'never changing' 으로 쉽게 이해할수 있다.
///     어떤 제약조건이 있기 때문에 "useful" 함을 따질 필요없이, 해당 lifetime 을 그대로 유지해야 하는 조건이다.
/// 
///   3. 예시로 Convariant / Contravariant / Invariant 개념 확인
///      (해당 예시 참조 table : https://doc.rust-lang.org/reference/subtyping.html)
///     
///         -----------------------------------------------------
///             Type	         Variance in 'a	    Variance in T
///         -----------------------------------------------------
///    (6)  &'a T	              covariant	         covariant
///    (7)  &'a mut T	          covariant	         invariant
///    (5)  *const T		                         covariant
///    (4)  *mut T		                             invariant
///         [T] and [T; n]		                     covariant
///    (1)  fn() -> T		                         covariant
///    (2)  fn(T) -> ()		                         contravariant
///    (3)  fn(T) -> T		                         invariant
///         std::cell::UnsafeCell<T>		         invariant
///         std::marker::PhantomData<T>		         covariant
///    (8)  dyn Trait<T> + 'a	  covariant	         invariant
///         -----------------------------------------------------
/// 
///     (이해가 부족해서 일부만 작성...)
///     (1) fn() -> T 
///         retrun value 의 lifetime 만 고려하면된다. 
///         다른 제약조건이 없으므로, return T lifetime 이 길수록 해당 함수는 "useful" 해진다.
///         -> covariant
/// 
///     (2) fn(T) -> ()
///         return value 없이 argument litetime 만 고려애햐 한다면, 
///         특정(짧은) lifetime 만 보장되도 쓸수 있는 supertype 을 적용하는것이 함수를 적용하기 "useful" 해진다.
///         -> contravariant
///         -> 위 표에서 contravariant 는 이거 밖에 없다. 그만큼 특수한 경우에만 적용 가능 조건으로 생각됨
///            (단 하나의 case 이지만 실제로는 해당 case 자체가 많이 쓰일 수도 있을듯)
///            (함수가 아닌 일반 variable 선언에서 짧은 lifetime 을 갖는 조건이 "useful" 하다는게 성립하지 않을듯..)
/// 
///     (3) fn(T) -> T
///         input parameter 와 return value 가 동일한 generic 이므로 "useful" 을 따질수 없이, 조건 확정됨.
///         -> invariant
/// 
///     (4) *mut T
///         (2) 추가 결론에서 이야기한 바와 같이, 함수에 적용되지 않은 자체 variable 의 lifetime 에서는
///         contravariant 발생경우가 없을것으로 생각되므로, 
///         subtype 적용이 가능하다면 : covariant,
///         제약 조건으로 불가능하다면 : invariant 로 생각해도 될듯 하다. 
///         mutable 이므로 lifetime 을 더 길게 사용할수 없으므로  
///         -> invariant
/// 
///     (5) *const T
///         lifetime 에 대한 추가 고려 조건이 없으므로 subtype 이 "useful"
///         -> covariant
/// 
///     (6) &'a T 
///         generic type T 의 자체 lifetime 과 추가 부여된 lifetime 'a 모두 추가 고려 조건 없음
///         -> 둘다 covariant
/// 
///     (7) &'a mut T
///         (4) 번 조건과 같이 mutable Type 이므로 T 에 대해서는 lifetime 을 확정해줘야 함.
///         -> Variance in 'a : covariant	    Variance in T : invariant
///        
///     (8) dyn Trait<T> + 'a
///         (7) 번과 유사하게 Trait bound 된 generic type T 에 대해서 lifetime 을 건드릴수 없음.
///         -> Variance in 'a : covariant	    Variance in T : invariant
/// 
/// ---------------------------------------------------------------------------
