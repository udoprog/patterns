; ModuleID = 'out.c1a6822a857ac68e-cgu.0'
source_filename = "out.c1a6822a857ac68e-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nofree nounwind nonlazybind uwtable willreturn
define void @manipulate_nodes(i64* nonnull %0, i64* nonnull %1) unnamed_addr #0 {
start:
  %2 = getelementptr inbounds i64, i64* %0, i64 2
  %3 = bitcast i64* %2 to i32*
  %4 = load i32, i32* %3, align 4
  %5 = add i32 %4, 1
  store i32 %5, i32* %3, align 4
  %ptr.i = bitcast i64* %1 to i64**
  %6 = load i64*, i64** %ptr.i, align 8
  %.not = icmp eq i64* %6, null
  br i1 %.not, label %bb9, label %bb6

bb6:                                              ; preds = %start
  %7 = getelementptr inbounds i64, i64* %6, i64 2
  %8 = bitcast i64* %7 to i32*
  %9 = load i32, i32* %8, align 4
  %10 = add i32 %9, 1
  store i32 %10, i32* %8, align 4
  br label %bb9

bb9:                                              ; preds = %start, %bb6
  ret void
}

attributes #0 = { nofree nounwind nonlazybind uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
