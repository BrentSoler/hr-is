import { COASummarywDetails } from "@/bindings/COASummarywDetails";
import { Employee } from "@/bindings/Employee";
import { Schedule } from "@/bindings/Schedule";
import { create } from "zustand";

const useUserStore = create((set) => ({
    user: { emp_id: "", emp_last: "", emp_first: "", emp_mid: "", emp_dept: "", emp_loc: "", emp_pswd: "" },
    sched: {},
    coa: {},
    setUser: (userInfo: Employee) => set((state: any) => ({ ...state, user: userInfo })),
    setSched: (schedInfo: Schedule) => set((state: any) => ({ ...state, sched: schedInfo })),
    setCOA: (coaInfo: COASummarywDetails) => set((state: any) => ({ ...state, coa: coaInfo }))
}))

export default useUserStore;
