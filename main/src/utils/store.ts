import { create } from "zustand";

const useUserStore = create((set) => ({
    token: "",
    setToken: (token: string) => set((state: any) => ({ ...state, token: token }))
}))

export default useUserStore;
