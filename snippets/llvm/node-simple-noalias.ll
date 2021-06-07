; ModuleID = 'out.c1a6822a857ac68e-cgu.0'
source_filename = "out.c1a6822a857ac68e-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%Node = type { [0 x i64], i64*, [0 x i64], i64*, [0 x i32], i32, [1 x i32] }

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
  %.not = icmp eq {}* %7, null
  br i1 %.not, label %bb6, label %bb4

bb4:                                              ; preds = %start
  %8 = bitcast {}* %7 to %Node*
  %9 = getelementptr inbounds %Node, %Node* %8, i64 0, i32 5
  %10 = load i32, i32* %9, align 8
  %11 = add i32 %10, 1
  store i32 %11, i32* %9, align 8
  br label %bb6

bb6:                                              ; preds = %start, %bb4
  ret void
}

attributes #0 = { nofree nounwind nonlazybind uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
