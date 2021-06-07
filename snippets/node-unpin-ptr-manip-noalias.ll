; ModuleID = 'out.c1a6822a857ac68e-cgu.0'
source_filename = "out.c1a6822a857ac68e-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"std::panic::Location" = type { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }

@alloc2 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Option::unwrap()` on a `None` value" }>, align 1
@alloc3 = private unnamed_addr constant <{ [32 x i8] }> <{ [32 x i8] c"snippets/node-unpin-ptr-manip.rs" }>, align 1
@alloc4 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [32 x i8] }>, <{ [32 x i8] }>* @alloc3, i32 0, i32 0, i32 0), [16 x i8] c" \00\00\00\00\00\00\00#\00\00\00 \00\00\00" }>, align 8

; Function Attrs: nonlazybind uwtable
define void @manipulate_nodes(i64* nonnull %0, i64* nonnull %1) unnamed_addr #0 {
start:
  %2 = getelementptr inbounds i64, i64* %0, i64 2
  %3 = bitcast i64* %2 to i32*
  %4 = load i32, i32* %3, align 4
  %5 = add i32 %4, 1
  store i32 %5, i32* %3, align 4
  %ptr.i = bitcast i64* %1 to i64**
  %_12 = load i64*, i64** %ptr.i, align 8
  %6 = icmp eq i64* %_12, null
  br i1 %6, label %bb1.i, label %"_ZN4core6option15Option$LT$T$GT$6unwrap17h0f5393abaa4eb231E.exit"

bb1.i:                                            ; preds = %start
; call core::panicking::panic
  tail call void @_ZN4core9panicking5panic17hedbcaa5a849f0fe8E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [43 x i8] }>* @alloc2 to [0 x i8]*), i64 43, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc4 to %"std::panic::Location"*))
  unreachable

"_ZN4core6option15Option$LT$T$GT$6unwrap17h0f5393abaa4eb231E.exit": ; preds = %start
  %7 = getelementptr inbounds i64, i64* %_12, i64 2
  %8 = bitcast i64* %7 to i32*
  %9 = load i32, i32* %8, align 4
  %10 = add i32 %9, 1
  store i32 %10, i32* %8, align 4
  ret void
}

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17hedbcaa5a849f0fe8E([0 x i8]* noalias nonnull readonly align 1, i64, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24)) unnamed_addr #1

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
