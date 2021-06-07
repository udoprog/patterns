; ModuleID = 'out.c1a6822a857ac68e-cgu.0'
source_filename = "out.c1a6822a857ac68e-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%Node = type { [0 x i8], %"std::marker::PhantomPinned", [0 x i8], i64*, [0 x i64], i64*, [0 x i32], i32, [1 x i32] }
%"std::marker::PhantomPinned" = type {}

; Function Attrs: nofree norecurse nounwind nonlazybind uwtable willreturn
define void @manipulate_nodes(%Node* nocapture align 8 dereferenceable(24) %a, %Node* nocapture readnone align 8 dereferenceable(24) %b) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds %Node, %Node* %a, i64 0, i32 7
  %1 = load i32, i32* %0, align 8
  %2 = add i32 %1, 1
  store i32 %2, i32* %0, align 8
  ret void
}

attributes #0 = { nofree norecurse nounwind nonlazybind uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
