;  Brainfck compiler
;  Copyright (C) 2025  František Slivko <slivko.frantisek@gmail.com>
;
;  This program is free software: you can redistribute it and/or modify
;  it under the terms of the GNU General Public License as published by
;  the Free Software Foundation, either version 3 of the License, or
;  (at your option) any later version.
;
;  This program is distributed in the hope that it will be useful,
;  but WITHOUT ANY WARRANTY; without even the implied warranty of
;  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;  GNU General Public License for more details.
;
;  You should have received a copy of the GNU General Public License
;  along with this program.  If not, see <https://www.gnu.org/licenses/>.



; @hello = private constant [13 x i8] c"Hello world!\00"
; @prompt = private constant [3 x i8] c"> \00"
@i_print = private constant [4 x i8] c"%i\0A\00"

@unimplemented = private constant [14 x i8] c"unimplemented\00"
@none = private constant [5 x i8] c"none\00"
@panic_f = private constant [23 x i8] c"\0Apanicked: %s | at %i\0A\00";

declare void @printf(i8*, ...) nounwind
declare i8 @getchar() nounwind
declare void @putchar(i8) nounwind
declare void @puts(i8*) nounwind

declare i32 @tcgetattr(i32, ptr) nounwind
;int tcgetattr(int fd, struct termios *termios_p);
declare i32 @tcsetattr(i32, i32, ptr)
;int tcsetattr(int fd, int optional_actions,
;              const struct termios *termios_p);



define i8 @main() {
  %old = alloca [40 x i32]
  call i32 @tcgetattr(i32 0, ptr %old) ; retrieves present terminal settings so that they could be restored afterwards

  %new = alloca [40 x i32]
  call void @llvm.memcpy.p0.p0.i32(ptr %new, ptr %old, i32 40, i1 0) ; copies old settings


  %c_lflag_ptr = getelementptr i32, ptr %new, i32 3 ; pointer to c_lflag
  %c_lflag_val = load i32, i32* %c_lflag_ptr

  %mask = xor i32 10, -1
  %new_c_lflag = and i32 %c_lflag_val, %mask

  store i32 %new_c_lflag, i32* %c_lflag_ptr
  %set = call i32 @tcsetattr(i32 0, i32 0, ptr %new) ; sets new terminal settings

  %exit_v = call i8 @code()

  call i32 @tcsetattr(i32 0, i32 0, ptr %old) ; restores original terminal settings

  ret i8 %exit_v
}


