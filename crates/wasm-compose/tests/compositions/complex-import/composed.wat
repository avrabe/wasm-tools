(component
  (type (;0;)
    (component
      (type (;0;)
        (instance
          (type (;0;) (func (param "x" string) (result string)))
          (export (;0;) "m" (func (type 0)))
        )
      )
      (import "b1" (instance (;0;) (type 0)))
      (import "b2" (instance (;1;) (type 0)))
      (type (;1;) (func (param "x" string) (result string)))
      (export (;0;) "m1" (func (type 1)))
      (export (;1;) "m2" (func (type 1)))
    )
  )
  (import "input" (component (;0;) (type 0)))
  (type (;1;)
    (component
      (type (;0;)
        (instance
          (type (;0;) (func))
          (export (;0;) "a" (func (type 0)))
          (type (;1;) (func (param "x" s8)))
          (export (;1;) "b" (func (type 1)))
          (type (;2;) (func (param "x" u8)))
          (export (;2;) "c" (func (type 2)))
          (type (;3;) (func (param "x" s16)))
          (export (;3;) "d" (func (type 3)))
          (type (;4;) (func (param "x" u16)))
          (export (;4;) "e" (func (type 4)))
          (type (;5;) (func (param "x" s32)))
          (export (;5;) "f" (func (type 5)))
          (type (;6;) (func (param "x" u32)))
          (export (;6;) "g" (func (type 6)))
          (type (;7;) (func (param "x" s64)))
          (export (;7;) "h" (func (type 7)))
          (type (;8;) (func (param "x" u64)))
          (export (;8;) "i" (func (type 8)))
          (type (;9;) (func (param "x" float32)))
          (export (;9;) "j" (func (type 9)))
          (type (;10;) (func (param "x" float64)))
          (export (;10;) "k" (func (type 10)))
          (type (;11;) (func (param "x" bool)))
          (export (;11;) "l" (func (type 11)))
          (type (;12;) (func (param "x" string)))
          (export (;12;) "m" (func (type 12)))
          (type (;13;) (record (field "a" s8) (field "b" u8) (field "c" s16) (field "d" u16) (field "e" s32) (field "f" u32) (field "g" s64) (field "h" u64) (field "i" float32) (field "j" float64) (field "k" bool) (field "l" string)))
          (export (;14;) "record1" (type (eq 13)))
          (type (;15;) (func (param "x" 14)))
          (export (;13;) "n" (func (type 15)))
          (type (;16;) (list 14))
          (type (;17;) (func (param "x" 16)))
          (export (;14;) "o" (func (type 17)))
          (type (;18;) (tuple 14 string))
          (type (;19;) (func (param "x" 18)))
          (export (;15;) "p" (func (type 19)))
          (type (;20;) (flags "a" "b" "c"))
          (export (;21;) "flags1" (type (eq 20)))
          (type (;22;) (func (param "x" 21)))
          (export (;16;) "q" (func (type 22)))
          (type (;23;) (enum "a" "b" "c"))
          (export (;24;) "enum1" (type (eq 23)))
          (type (;25;) (func (param "x" 24)))
          (export (;17;) "r" (func (type 25)))
          (type (;26;) (option 14))
          (export (;27;) "option1" (type (eq 26)))
          (type (;28;) (func (param "x" 27)))
          (export (;18;) "s" (func (type 28)))
          (type (;29;) (variant (case "a" s8) (case "b" u8) (case "c" s16) (case "d" u16) (case "e" s32) (case "f" u32) (case "g" s64) (case "h" u64) (case "i" float32) (case "j" float64) (case "k" bool) (case "l" string) (case "m" 14)))
          (export (;30;) "variant1" (type (eq 29)))
          (type (;31;) (option 30))
          (type (;32;) (func (param "x" 31)))
          (export (;19;) "t" (func (type 32)))
          (type (;33;) (result 14 (error string)))
          (type (;34;) (func (result 33)))
          (export (;20;) "u" (func (type 34)))
        )
      )
      (import "a" (instance (;0;) (type 0)))
      (type (;1;)
        (instance
          (type (;0;) (func (param "x" string) (result string)))
          (export (;0;) "m" (func (type 0)))
        )
      )
      (export (;1;) "x" (instance (type 1)))
    )
  )
  (import "b" (component (;1;) (type 1)))
  (type (;2;)
    (component
      (type (;0;) (record (field "a" s8) (field "b" u8) (field "c" s16) (field "d" u16) (field "e" s32) (field "f" u32) (field "g" s64) (field "h" u64) (field "i" float32) (field "j" float64) (field "k" bool) (field "l" string)))
      (export (;1;) "record1" (type (eq 0)))
      (type (;2;) (flags "a" "b" "c"))
      (export (;3;) "flags1" (type (eq 2)))
      (type (;4;) (enum "a" "b" "c"))
      (export (;5;) "enum1" (type (eq 4)))
      (type (;6;) (option 1))
      (export (;7;) "option1" (type (eq 6)))
      (type (;8;) (variant (case "a" s8) (case "b" u8) (case "c" s16) (case "d" u16) (case "e" s32) (case "f" u32) (case "g" s64) (case "h" u64) (case "i" float32) (case "j" float64) (case "k" bool) (case "l" string) (case "m" 1)))
      (export (;9;) "variant1" (type (eq 8)))
      (type (;10;) (func))
      (export (;0;) "a" (func (type 10)))
      (type (;11;) (func (param "x" s8)))
      (export (;1;) "b" (func (type 11)))
      (type (;12;) (func (param "x" u8)))
      (export (;2;) "c" (func (type 12)))
      (type (;13;) (func (param "x" s16)))
      (export (;3;) "d" (func (type 13)))
      (type (;14;) (func (param "x" u16)))
      (export (;4;) "e" (func (type 14)))
      (type (;15;) (func (param "x" s32)))
      (export (;5;) "f" (func (type 15)))
      (type (;16;) (func (param "x" u32)))
      (export (;6;) "g" (func (type 16)))
      (type (;17;) (func (param "x" s64)))
      (export (;7;) "h" (func (type 17)))
      (type (;18;) (func (param "x" u64)))
      (export (;8;) "i" (func (type 18)))
      (type (;19;) (func (param "x" float32)))
      (export (;9;) "j" (func (type 19)))
      (type (;20;) (func (param "x" float64)))
      (export (;10;) "k" (func (type 20)))
      (type (;21;) (func (param "x" bool)))
      (export (;11;) "l" (func (type 21)))
      (type (;22;) (func (param "x" string)))
      (export (;12;) "m" (func (type 22)))
      (type (;23;) (func (param "x" 1)))
      (export (;13;) "n" (func (type 23)))
      (type (;24;) (list 1))
      (type (;25;) (func (param "x" 24)))
      (export (;14;) "o" (func (type 25)))
      (type (;26;) (tuple 1 string))
      (type (;27;) (func (param "x" 26)))
      (export (;15;) "p" (func (type 27)))
      (type (;28;) (func (param "x" 3)))
      (export (;16;) "q" (func (type 28)))
      (type (;29;) (func (param "x" 5)))
      (export (;17;) "r" (func (type 29)))
      (type (;30;) (func (param "x" 7)))
      (export (;18;) "s" (func (type 30)))
      (type (;31;) (option 9))
      (type (;32;) (func (param "x" 31)))
      (export (;19;) "t" (func (type 32)))
      (type (;33;) (result 1 (error string)))
      (type (;34;) (func (result 33)))
      (export (;20;) "u" (func (type 34)))
    )
  )
  (import "a" (component (;2;) (type 2)))
  (instance (;0;) (instantiate 2))
  (instance (;1;) (instantiate 1
      (with "a" (instance 0))
    )
  )
  (instance (;2;) (instantiate 1
      (with "a" (instance 0))
    )
  )
  (alias export 2 "x" (instance (;3;)))
  (alias export 1 "x" (instance (;4;)))
  (instance (;5;) (instantiate 0
      (with "b1" (instance 3))
      (with "b2" (instance 4))
    )
  )
  (alias export 5 "m1" (func (;0;)))
  (export (;1;) "m1" (func 0))
  (alias export 5 "m2" (func (;2;)))
  (export (;3;) "m2" (func 2))
)
