; ModuleID = 'out.c1a6822a857ac68e-cgu.0'
source_filename = "out.c1a6822a857ac68e-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%Node = type { [0 x i8], %"std::marker::PhantomPinned", [0 x i8], i64*, [0 x i64], i64*, [0 x i32], i32, [1 x i32] }
%"std::marker::PhantomPinned" = type {}

; Function Attrs: nofree nounwind nonlazybind uwtable willreturn
define void @manipulate_nodes(i64* nonnull %0, i64* nonnull %1) unnamed_addr #0 {
start:
  %2 = getelementptr inbounds i64, i64* %0, i64 2
  %3 = bitcast i64* %2 to i32*
  %4 = load i32, i32* %3, align 8
  %5 = add i32 %4, 1
  store i32 %5, i32* %3, align 8
  %6 = bitcast i64* %1 to {}**
  %7 = load {}*, {}** %6, align 8
  %8 = icmp eq {}* %7, null
  br i1 %8, label %bb8, label %bb5

bb5:                                              ; preds = %start
  %9 = bitcast {}* %7 to %Node*
  %10 = getelementptr inbounds %Node, %Node* %9, i64 0, i32 7
  %11 = load i32, i32* %10, align 8
  %12 = add i32 %11, 1
  store i32 %12, i32* %10, align 8
  br label %bb8

bb8:                                              ; preds = %start, %bb5
  ret void
}

attributes #0 = { nofree nounwind nonlazybind uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
