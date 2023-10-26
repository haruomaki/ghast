; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32 %0)

define i32 @main() {
entry:
  %putchar = call i32 @putchar(i32 72)
  %putchar1 = call i32 @putchar(i32 105)
  ret i32 0
}
