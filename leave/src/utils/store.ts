import { Employee } from "@/bindings/Employee";
import { LeaveSummarywDetais } from "@/bindings/LeaveSummarywDetais";
import { Schedule } from "@/bindings/Schedule";
import { create } from "zustand";

const useUserStore = create((set) => ({
    user: {},
    sched: {},
    leave: {},
    setUser: (userInfo: Employee) => set((state: any) => ({ ...state, user: userInfo })),
    setSched: (schedInfo: Schedule) => set((state: any) => ({ ...state, sched: schedInfo })),
    setLeave: (leaveInfo: LeaveSummarywDetais) => set((state: any) => ({ ...state, leave: leaveInfo }))
}))

export default useUserStore;
